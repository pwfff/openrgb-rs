use async_trait::async_trait;

use crate::DEFAULT_PROTOCOL;

use super::*;

pub struct RequestProtocolVersion {
    pub header: Header,
}

#[async_trait]
impl PacketT for RequestProtocolVersion {
    async fn read(
        &self,
        stream: &mut impl OpenRGBReadableStream,
        protocol: u32,
    ) -> Result<Packet, OpenRGBError> {
        u32::read(stream, protocol).await?;
        Ok(Packet::RequestProtocolVersion(*self))
    }

    fn header(&self) -> &Header {
        &self.header
    }

    fn size(&self, protocol: u32) -> usize {
        protocol.size(protocol)
    }

    async fn write_body(
        &self,
        stream: &mut impl OpenRGBWritableStream,
        protocol: u32,
    ) -> Result<(), OpenRGBError> {
        // respond with our version
        println!("responding with protocol version");
        stream.write_value(DEFAULT_PROTOCOL, protocol).await
    }
}

pub struct RequestControllerCount {
    pub header: Header,
}

#[async_trait]
impl PacketT for RequestControllerCount {
    async fn read(
        &self,
        stream: &mut impl OpenRGBReadableStream,
        protocol: u32,
    ) -> Result<Packet, OpenRGBError> {
        Ok(Packet::RequestControllerCount(*self))
    }

    fn header(&self) -> &Header {
        &self.header
    }

    fn size(&self, protocol: u32) -> usize {
        protocol.size(protocol)
    }

    async fn write_body(
        &self,
        stream: &mut impl OpenRGBWritableStream,
        protocol: u32,
    ) -> Result<(), OpenRGBError> {
        // respond with our version
        println!("responding with controller count");
        stream.write_value(5, protocol).await
    }
}
