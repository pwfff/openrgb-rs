use std::any::Any;

use async_trait::async_trait;
use enum_dispatch::enum_dispatch;

use crate::data::{OpenRGBReadable, PacketId};
use crate::protocol::{OpenRGBReadableStream, OpenRGBStream, OpenRGBWritableStream, MAGIC};
use crate::OpenRGBError;
use crate::OpenRGBError::ProtocolError;

use super::OpenRGBWritable;
use packets::*;

mod packets;

#[enum_dispatch]
pub enum Packet {
    RequestProtocolVersion(RequestProtocolVersion),
    RequestControllerCount(RequestControllerCount),
}

// trait PacketBody: Sized + OpenRGBReadable {}

pub async fn read_any(
    stream: &mut impl OpenRGBStream,
    protocol: u32,
) -> Result<Packet, OpenRGBError> {
    let header = Header::read(stream, protocol).await?;
    match header.packet_id {
        PacketId::RequestProtocolVersion => {
            let p = RequestProtocolVersion { header };
            p.read(stream, protocol).await
        }
        PacketId::RequestControllerCount => {
            let p = RequestControllerCount { header };
            p.read(stream, protocol).await
        }
        // PacketId::RequestControllerCount => {
        //     RequestControllerCount::read(header, stream, protocol).await
        // }
        // PacketId::SetClientName => {
        //     // consume client name
        //     // TODO: use this??
        //     self.read_value::<String>(protocol).await;
        //     Ok(())
        // }
        // PacketId::RequestControllerCount => {
        //     // consume client protocol version
        //     self.read_value::<String>(protocol).await;
        //     // TODO: actually count controllers?
        //     self.write_packet(protocol, 0, RequestControllerCount, 1u32)
        //         .await
        // }
        // PacketId::RequestControllerData => stream.read_packet::<Controller>(protocol).await,
        _ => Err(OpenRGBError::ProtocolError(format!(
            "don't know how to respond to packet ID: {:?}",
            header.packet_id
        ))),
    }
}

#[async_trait]
pub trait PacketT: Sync {
    async fn read(
        &self,
        stream: &mut impl OpenRGBReadableStream,
        protocol: u32,
    ) -> Result<Packet, OpenRGBError>;

    fn header(&self) -> &Header;

    fn size(&self, protocol: u32) -> usize;

    async fn write_body(
        &self,
        stream: &mut impl OpenRGBWritableStream,
        protocol: u32,
    ) -> Result<(), OpenRGBError>;
}

async fn write<T: PacketT>(
    p: &T,
    stream: &mut impl OpenRGBWritableStream,
    protocol: u32,
) -> Result<(), OpenRGBError> {
    println!("writing header");
    let header = p.header();
    stream
        .write_header(
            protocol,
            header.device_id,
            header.packet_id,
            p.size(protocol),
        )
        .await?;
    p.write_body(stream, protocol).await
}

pub struct Header {
    pub device_id: u32,
    pub packet_id: PacketId,
    pub data_length: u32,
}

#[async_trait]
#[doc(hidden)]
impl OpenRGBReadable for Header {
    async fn read(
        stream: &mut impl OpenRGBReadableStream,
        protocol: u32,
    ) -> Result<Self, OpenRGBError> {
        for c in MAGIC {
            if stream.read_u8().await? != c {
                return Err(ProtocolError(format!(
                    "expected OpenRGB magic value, got \"{}\"",
                    c
                )));
            }
        }

        let device_id = stream.read_value::<u32>(protocol).await?;
        let packet_id = stream.read_value::<PacketId>(protocol).await?;
        let data_length = stream
            .read_value::<u32>(protocol)
            .await
            .map_err(|e| ProtocolError(format!("received invalid data length: {}", e)))?;

        Ok(Self {
            device_id,
            packet_id,
            data_length,
        })
    }
}
