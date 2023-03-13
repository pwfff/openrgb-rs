use async_trait::async_trait;

use super::Header;
use crate::{
    data::{Controller, OpenRGBReadable, OpenRGBWritable, PacketId},
    protocol::{OpenRGBReadableStream, OpenRGBStream, OpenRGBWritableStream},
    OpenRGBError,
};

#[async_trait]
pub trait RequestPacket<T>: Sync + Send {
    fn header(&self) -> &Header;

    async fn read(
        &mut self,
        // host *controller.Host,
        // device *hid.Device,
        header: Header,
        stream: &mut T,
        protocol: u32,
    ) -> Result<(), OpenRGBError>
    where
        T: OpenRGBReadableStream;

    async fn handle(
        &self,
        // host *controller.Host,
        // device *hid.Device,
        stream: &mut T,
    ) -> Result<(), OpenRGBError>
    where
        T: OpenRGBWritableStream;
}

#[async_trait]
trait ResponsePacket: OpenRGBWritable {
    fn header(&self) -> &Header;
}

pub async fn read_any<T: OpenRGBStream>(
    stream: &mut T,
    protocol: u32,
) -> Result<Box<dyn RequestPacket<T>>, OpenRGBError> {
    let header = stream.read_value::<Header>(protocol).await?;

    let mut p = match header.packet_id {
        PacketId::RequestControllerCount => Ok(Packet::<RequestControllerCount>::new()),
        PacketId::RequestControllerData => Ok(Packet::<RequestControllerCount>::new()),
        _ => Err(OpenRGBError::UnsupportedOperation {
            operation: format!("{:?}", header.packet_id),
            current_protocol_version: 1,
            min_protocol_version: 199,
        }),
    }?;

    p.read(header, stream, protocol).await?;

    Ok(p)
}

pub struct Packet<T>
where
    T: PacketBody,
{
    header: Header,
    body: Option<T>,
}

impl<T> Packet<T>
where
    T: PacketBody,
{
    fn new() -> Box<Self> {
        Box::new(Self {
            header: Header::default(),
            body: None,
        })
    }
}

#[async_trait]
impl<T: RequestPacketBody<U>, U: OpenRGBStream> RequestPacket<U> for Packet<T> {
    fn header(&self) -> &Header {
        &self.header
    }

    async fn read(
        &mut self,
        // host *controller.Host,
        // device *hid.Device,
        header: Header,
        stream: &mut U,
        protocol: u32,
    ) -> Result<(), OpenRGBError> {
        self.body = Some(stream.read_value(protocol).await?);
        Ok(())
    }

    async fn handle(
        &self,
        // host *controller.Host,
        // device *hid.Device,
        stream: &mut U,
    ) -> Result<(), OpenRGBError> {
        todo!()
    }
}

#[async_trait]
impl<T: PacketBody> OpenRGBWritable for Packet<T> {
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
pub trait PacketBody: OpenRGBReadable + OpenRGBWritable {}

#[async_trait]
trait RequestPacketBody<T: OpenRGBStream>: PacketBody {
    async fn handle(
        &self,
        // host *controller.Host,
        // device *hid.Device,
        stream: &mut T,
    ) -> Result<(), OpenRGBError>;
}

// async fn read<T: PacketBody>(
//     header: Header,
//     stream: &mut impl OpenRGBReadableStream,
//     protocol: u32,
// ) -> Result<T, OpenRGBError> {
// }

pub struct RequestControllerCount {}

#[async_trait]
impl PacketBody for RequestControllerCount {}

#[async_trait]
impl<T: OpenRGBStream> RequestPacketBody<T> for RequestControllerCount {
    async fn handle(
        &self,
        // host *controller.Host,
        // device *hid.Device,
        stream: &mut T,
    ) -> Result<(), OpenRGBError> {
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

// #[async_trait]
// impl OpenRGBWritable for RequestControllerCountResponse {}

pub struct RequestControllerData {
    controller: Controller,
}

#[async_trait]
impl PacketBody for RequestControllerData {}

#[async_trait]
impl<T: OpenRGBStream> RequestPacketBody<T> for RequestControllerData {
    async fn handle(
        &self,
        // host *controller.Host,
        // device *hid.Device,
        stream: &mut T,
    ) -> Result<(), OpenRGBError> {
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
