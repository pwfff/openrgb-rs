use async_trait::async_trait;

use crate::data::{OpenRGBReadable, PacketId};
use crate::protocol::{OpenRGBReadableStream, OpenRGBWritableStream, MAGIC};
use crate::OpenRGBError;
use crate::OpenRGBError::ProtocolError;

use super::OpenRGBWritable;

pub trait Packet<T>
where
    Self: Sized,
    T: OpenRGBReadable,
{
    fn header(&self) -> Header;
    fn body(&self) -> T;
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

pub struct RequestProtocolVersion {
    header: Header,
    body: RequestProtocolVersionBody,
}

impl RequestProtocolVersion {
    pub fn new(header: Header, body: RequestProtocolVersionBody) -> Self {
        Self { header, body }
    }
}

pub struct RequestProtocolVersionBody {}

#[async_trait]
impl OpenRGBReadable for RequestProtocolVersionBody {
    async fn read(
        stream: &mut impl OpenRGBReadableStream,
        protocol: u32,
    ) -> Result<Self, OpenRGBError> {
        // consume client protocol version
        stream.read_value::<u32>(protocol).await?;
        Ok(RequestProtocolVersionBody {})
    }
}

#[async_trait]
impl OpenRGBWritable for RequestProtocolVersion {
    fn size(&self, protocol: u32) -> usize {
        PacketId::RequestProtocolVersion.size(protocol)
    }

    async fn write(
        self,
        stream: &mut impl OpenRGBWritableStream,
        protocol: u32,
    ) -> Result<(), OpenRGBError> {
        // respond with our version
        stream
            .write_packet(
                protocol,
                self.header.device_id,
                PacketId::RequestProtocolVersion,
                protocol,
            )
            .await
    }
}
