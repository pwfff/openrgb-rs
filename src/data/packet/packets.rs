use async_trait::async_trait;

use super::*;

pub struct RequestProtocolVersion {
    header: Header,
}

#[async_trait]
impl Packet for RequestProtocolVersion {
    async fn read(
        header: Box<Header>,
        stream: &mut impl OpenRGBReadableStream,
        protocol: u32,
    ) -> Result<Self, OpenRGBError> {
        u32::read(stream, protocol).await?;
        Ok(Self { header: *header })
    }

    fn header(&self) -> &Header {
        &self.header
    }

    fn size(&self, protocol: u32) -> usize {
        protocol.size(protocol)
    }

    async fn write_body(
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

pub struct RequestControllerCount {
    header: Header,
}

#[async_trait]
impl Packet for RequestControllerCount {
    async fn read(
        header: Box<Header>,
        stream: &mut impl OpenRGBReadableStream,
        protocol: u32,
    ) -> Result<Self, OpenRGBError> {
        u32::read(stream, protocol).await?;
        Ok(Self { header: *header })
    }

    fn header(&self) -> &Header {
        &self.header
    }

    fn size(&self, protocol: u32) -> usize {
        protocol.size(protocol)
    }

    async fn write_body(
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
                PacketId::RequestControllerCount,
                protocol,
            )
            .await
    }
}
