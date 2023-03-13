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

    match header.packet_id {
        PacketId::RequestControllerCount => {
            RequestControllerCount::new(stream, header, protocol).await
        }
        PacketId::RequestControllerData => {
            RequestControllerData::new(stream, header, protocol).await
        }

        _ => Err(OpenRGBError::UnsupportedOperation {
            operation: "handle packet".to_string(),
            current_protocol_version: 1,
            min_protocol_version: 199,
        }),
    }
}

pub struct Packet<T: PacketBody> {
    header: Header,
    body: T,
}

#[async_trait]
impl<T: RequestPacketBody<U>, U: OpenRGBStream> RequestPacket<U> for Packet<T> {
    fn header(&self) -> &Header {
        &self.header
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
        self.body.size(protocol)
    }

    async fn write(
        self,
        stream: &mut impl OpenRGBWritableStream,
        protocol: u32,
    ) -> Result<(), OpenRGBError> {
        self.body.write(stream, protocol).await
    }
}

#[async_trait]
pub trait PacketBody: OpenRGBReadable + OpenRGBWritable {}

#[async_trait]
trait RequestPacketBody<T: OpenRGBStream>: PacketBody {
    async fn new(
        stream: &mut T,
        header: Header,
        protocol: u32,
    ) -> Result<Box<dyn RequestPacket<T>>, OpenRGBError>
    where
        Self: 'static,
    {
        let mut p = Packet::<Self> {
            header: header,
            body: stream.read_value(protocol).await?,
        };
        Ok(Box::new(p))
    }

    async fn handle(
        &self,
        // host *controller.Host,
        // device *hid.Device,
        stream: &mut T,
    ) -> Result<(), OpenRGBError>
    where
        Self: Sized;
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
    ) -> Result<Self, OpenRGBError>
    where
        Self: Sized,
    {
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
    ) -> Result<(), OpenRGBError>
    where
        Self: Sized,
    {
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
    ) -> Result<Self, OpenRGBError>
    where
        Self: Sized,
    {
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
    ) -> Result<(), OpenRGBError>
    where
        Self: Sized,
    {
        todo!()
    }
}
