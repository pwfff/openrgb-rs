use async_trait::async_trait;
use macros::impl_packet;

use super::*;

#[impl_packet(RequestProtocolVersionBody)]
pub struct RequestProtocolVersion {
    header: Header,
    body: RequestProtocolVersionBody,
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
        protocol.size(protocol)
    }

    async fn write(
        self,
        stream: &mut impl OpenRGBWritableStream,
        protocol: u32,
    ) -> Result<(), OpenRGBError> {
        // respond with our version
        println!("responding with protocol version");
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

// #[impl_packet(RequestControllerCountBody)]
// pub struct RequestControllerCount {
//     header: Header,
//     body: RequestControllerCountBody,
// }

// pub struct RequestControllerCountBody {}

// #[async_trait]
// impl OpenRGBReadable for RequestControllerCountBody {
//     async fn read(
//         stream: &mut impl OpenRGBReadableStream,
//         protocol: u32,
//     ) -> Result<Self, OpenRGBError> {
//         // consume client protocol version
//         stream.read_value::<u32>(protocol).await?;
//         Ok(RequestControllerCountBody {})
//     }
// }

// #[async_trait]
// impl OpenRGBWritable for RequestControllerCount {
//     fn size(&self, protocol: u32) -> usize {
//         protocol.size(protocol)
//     }

//     async fn write(
//         self,
//         stream: &mut impl OpenRGBWritableStream,
//         protocol: u32,
//     ) -> Result<(), OpenRGBError> {
//         // respond with our version
//         println!("responding with protocol version");
//         stream
//             .write_packet(
//                 protocol,
//                 self.header.device_id,
//                 PacketId::RequestControllerCount,
//                 protocol,
//             )
//             .await
//     }
// }
