use async_trait::async_trait;

use crate::{data::packet::*, protocol::OpenRGBWritableStream, OpenRGBError};

pub struct Foo {}

#[async_trait]
impl RequestPacketHandler for Foo {
    async fn handle<S: OpenRGBWritableStream>(
        &self,
        stream: &mut S,
        packet: Requests,
    ) -> Result<(), OpenRGBError> {
        match packet {
            Requests::RequestControllerCount(p) => Ok(()),
            Requests::RequestControllerData(p) => Ok(()),
            _ => Err(OpenRGBError::ProtocolError("()".to_string())),
        }
    }
}
