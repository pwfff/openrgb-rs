use async_trait::async_trait;

use crate::data::{OpenRGBReadable, PacketId};
use crate::protocol::{OpenRGBReadableStream, MAGIC};
use crate::OpenRGBError;

#[derive(Default)]
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
                return Err(OpenRGBError::ProtocolError(format!(
                    "expected OpenRGB magic value, got \"{}\"",
                    c
                )));
            }
        }

        let device_id = stream.read_value::<u32>(protocol).await?;
        let packet_id = stream.read_value::<PacketId>(protocol).await?;
        let data_length = stream.read_value::<u32>(protocol).await.map_err(|e| {
            OpenRGBError::ProtocolError(format!("received invalid data length: {}", e))
        })?;

        Ok(Self {
            device_id,
            packet_id,
            data_length,
        })
    }
}
