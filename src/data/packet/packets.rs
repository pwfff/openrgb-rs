use async_trait::async_trait;

use super::header::Header;
use crate::{
    data::{Controller, OpenRGBReadable, OpenRGBWritable, PacketId},
    protocol::{OpenRGBReadableStream, OpenRGBStream, OpenRGBWritableStream},
    OpenRGBError,
};

/// read_any reads a full packet from the readable stream.
pub async fn read_any<T: OpenRGBReadableStream>(
    stream: &mut T,
    protocol: u32,
) -> Result<RequestPacket<impl RequestPacketBody>, OpenRGBError> {
    let header = stream.read_value::<Header>(protocol).await?;

    let mut p = match header.packet_id {
        PacketId::RequestControllerCount => Ok(RequestPacket::<RequestControllerCount>::new()),
        PacketId::RequestControllerData => Ok(RequestPacket::<RequestControllerCount>::new()),
        _ => Err(OpenRGBError::UnsupportedOperation {
            operation: format!("{:?}", header.packet_id),
            current_protocol_version: 1,
            min_protocol_version: 199,
        }),
    }?;

    p.read(header, stream, protocol).await?;

    Ok(p)
}

/// Packet<T> could be a request or a response packet
pub struct RequestPacket<T>
where
    T: RequestPacketBody,
{
    header: Header,
    body: Option<T>,
}

impl<T> RequestPacket<T>
where
    T: RequestPacketBody,
{
    // default implementation for constructing an empty packet, usually for reading.
    // header will be set when the packet is read.
    fn new() -> Self {
        Self {
            header: Header::default(),
            body: None,
        }
    }
}

/// RequestPacketReader is anything that has a header and can be read.
#[async_trait]
pub trait RequestPacketReader: Sync + Send {
    /// reads the packet, setting header and body.
    async fn read<S: OpenRGBReadableStream>(
        &mut self,
        header: Header,
        stream: &mut S,
        protocol: u32,
    ) -> Result<(), OpenRGBError>;
}

#[async_trait]
pub trait RequestPacketHandler: Sync + Send {
    /// handles the packet, returning the response packet.
    /// maybe this should on the host though...
    async fn handle<S: OpenRGBWritableStream>(
        &self,
        // host *controller.Host,
        // device *hid.Device,
        stream: &mut S,
        protocol: u32,
    ) -> Result<(), OpenRGBError>;
}

#[async_trait]
pub trait ResponsePacket: OpenRGBWritable {}

#[async_trait]
impl<T: RequestPacketBody> RequestPacketReader for RequestPacket<T> {
    async fn read<S>(
        &mut self,
        // host *controller.Host,
        // device *hid.Device,
        header: Header,
        stream: &mut S,
        protocol: u32,
    ) -> Result<(), OpenRGBError>
    where
        S: OpenRGBReadableStream,
    {
        self.header = header;
        self.body = Some(stream.read_value(protocol).await?);
        Ok(())
    }
}

#[async_trait]
impl<T: RequestPacketBody> RequestPacketHandler for RequestPacket<T> {
    async fn handle<S>(
        &self,
        // host *controller.Host,
        // device *hid.Device,
        stream: &mut S,
        protocol: u32,
    ) -> Result<(), OpenRGBError>
    where
        S: OpenRGBWritableStream,
    {
        let resp = self
            .body
            .as_ref()
            .ok_or(OpenRGBError::ProtocolError("()".to_string()))?
            .handle(stream)
            .await?;
        stream.write_value(resp, protocol).await
    }
}

#[async_trait]
impl<T: PacketBody> OpenRGBWritable for RequestPacket<T> {
    fn size(&self, protocol: u32) -> usize {
        match &self.body {
            Some(body) => body.size(protocol),
            None => 0,
        }
    }

    async fn write(
        self,
        stream: &mut impl OpenRGBWritableStream,
        protocol: u32,
    ) -> Result<(), OpenRGBError> {
        match self.body {
            Some(body) => body.write(stream, protocol).await,
            None => Err(OpenRGBError::ProtocolError(
                "attempted to write before setting body".to_string(),
            )),
        }
    }
}

#[async_trait]
pub trait PacketBody: RequestPacketBody + ResponsePacketBody {}

#[async_trait]
pub trait RequestPacketBody: OpenRGBReadable {
    type Response: ResponsePacket;

    async fn handle<S>(
        &self,
        // host *controller.Host,
        // device *hid.Device,
        stream: &mut S,
    ) -> Result<Self::Response, OpenRGBError>
    where
        S: OpenRGBWritableStream;
}

pub trait ResponsePacketBody: OpenRGBWritable {}

pub struct RequestControllerCount {}

#[async_trait]
impl RequestPacketBody for RequestControllerCount {
    type Response = RequestControllerCountResponse;

    async fn handle<S: OpenRGBWritableStream>(
        &self,
        // host *controller.Host,
        // device *hid.Device,
        stream: &mut S,
    ) -> Result<Self::Response, OpenRGBError> {
        todo!()
    }
}

#[async_trait]
impl OpenRGBReadable for RequestControllerCount {
    async fn read(
        stream: &mut impl OpenRGBReadableStream,
        protocol: u32,
    ) -> Result<Self, OpenRGBError> {
        Ok(Self {})
    }
}

#[async_trait]
impl OpenRGBWritable for RequestControllerCount {
    fn size(&self, protocol: u32) -> usize {
        todo!()
    }

    async fn write(
        self,
        stream: &mut impl OpenRGBWritableStream,
        protocol: u32,
    ) -> Result<(), OpenRGBError> {
        todo!()
    }
}

pub struct RequestControllerCountResponse {
    count: u32,
}

#[async_trait]
impl ResponsePacket for RequestControllerCountResponse {}

#[async_trait]
impl OpenRGBWritable for RequestControllerCountResponse {
    fn size(&self, protocol: u32) -> usize {
        0
    }

    async fn write(
        self,
        stream: &mut impl OpenRGBWritableStream,
        protocol: u32,
    ) -> Result<(), OpenRGBError> {
        stream.write_value(0 as u32, protocol).await
    }
}

pub struct RequestControllerData {
    controller: Controller,
}

#[async_trait]
impl ResponsePacketBody for RequestControllerData {}

#[async_trait]
impl RequestPacketBody for RequestControllerData {
    // TODO: baaaad, fix
    type Response = RequestControllerCountResponse;

    async fn handle<S: OpenRGBWritableStream>(
        &self,
        // host *controller.Host,
        // device *hid.Device,
        stream: &mut S,
    ) -> Result<Self::Response, OpenRGBError> {
        todo!()
    }
}

#[async_trait]
impl OpenRGBReadable for RequestControllerData {
    async fn read(
        stream: &mut impl OpenRGBReadableStream,
        protocol: u32,
    ) -> Result<Self, OpenRGBError> {
        let controller = stream.read_value(protocol).await?;
        Ok(Self { controller })
    }
}

#[async_trait]
impl OpenRGBWritable for RequestControllerData {
    fn size(&self, protocol: u32) -> usize {
        todo!()
    }
    async fn write(
        self,
        stream: &mut impl OpenRGBWritableStream,
        protocol: u32,
    ) -> Result<(), OpenRGBError> {
        todo!()
    }
}
