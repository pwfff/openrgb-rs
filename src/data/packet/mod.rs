use async_trait::async_trait;

use crate::data::{OpenRGBReadable, PacketId};
use crate::protocol::{OpenRGBReadableStream, OpenRGBWritableStream, MAGIC};
use crate::OpenRGBError;
use crate::OpenRGBError::ProtocolError;

use super::OpenRGBWritable;
use packets::*;

mod packets;

#[async_trait]
pub trait Packet: OpenRGBWritable
where
    Self: Sized,
{
    async fn read_any<P: Packet>(
        header: Box<Header>,
        stream: &mut impl OpenRGBReadableStream,
        protocol: u32,
    ) -> Result<P, OpenRGBError> {
        P::read(header, stream, protocol).await
    }

    async fn read(
        header: Box<Header>,
        stream: &mut impl OpenRGBReadableStream,
        protocol: u32,
    ) -> Result<Self, OpenRGBError>;
    fn header(&self) -> &Header;

    fn size(&self, protocol: u32) -> usize {
        0
    }

    async fn write(
        self,
        stream: &mut impl OpenRGBWritableStream,
        protocol: u32,
    ) -> Result<(), OpenRGBError> {
        let header = self.header();
        stream
            .write_header(protocol, header.device_id, header.packet_id, 0)
            .await
    }
}

pub struct Header {
    pub device_id: u32,
    pub packet_id: PacketId,
    pub data_length: u32,
}

impl Header {
    async fn read(
        self,
        stream: &mut impl OpenRGBReadableStream,
        protocol: u32,
    ) -> Result<impl Packet, OpenRGBError> {
        match self.packet_id {
            PacketId::RequestProtocolVersion => {
                Packet::read_any::<RequestProtocolVersion>(self.into(), stream, protocol).await
            }
            // PacketId::RequestControllerCount => {
            //     Packet::read_any::<RequestControllerCount>(self.into(), stream, protocol).await
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
                self.packet_id
            ))),
        }
    }
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
