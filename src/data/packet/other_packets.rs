use async_trait::async_trait;

use super::Header;
use crate::{
    data::{Controller, OpenRGBReadable, OpenRGBWritable, PacketId},
    protocol::{OpenRGBReadableStream, OpenRGBWritableStream},
    OpenRGBError,
};

#[async_trait]
trait RequestPacket {
    fn header(&self) -> &Header;

    async fn read(
        &mut self,
        stream: &mut impl OpenRGBReadableStream,
    ) -> Result<&Self, OpenRGBError>
    where
        Self: Sized;

    async fn handle(
        &self,
        // host *controller.Host,
        // device *hid.Device,
        stream: &mut impl OpenRGBWritableStream,
    ) -> Option<OpenRGBError>
    where
        Self: Sized;
}

#[async_trait]
trait ResponsePacket: OpenRGBWritable {
    fn header(&self) -> &Header;
}

async fn read_any(
    stream: &mut impl OpenRGBReadableStream,
    protocol: u32,
) -> Result<Box<dyn RequestPacket>, OpenRGBError> {
    let header = stream.read_value::<Header>(protocol).await?;

    // let s: Box<dyn RequestPacket> = match header.packet_id {
    //     PacketId::RequestControllerCount => RequestControllerCount::newp(header),
    //     PacketId::RequestControllerData => RequestControllerData::newp(header),

    //     _ => Err(OpenRGBError::UnsupportedOperation {
    //         operation: "handle packet".to_string(),
    //         current_protocol_version: 1,
    //         min_protocol_version: 199,
    //     }),
    // }?;

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

struct Packet<T: PacketBody> {
    header: Header,
    body: T,
}

#[async_trait]
impl<T: PacketBody> RequestPacket for Packet<T> {
    fn header(&self) -> &Header {
        &self.header
    }

    async fn read(&mut self, stream: &mut impl OpenRGBReadableStream) -> Result<&Self, OpenRGBError>
    where
        Self: Sized,
    {
        self.body = stream.read_value(1).await?;
        Ok(self)
    }

    async fn handle(
        &self,
        // host *controller.Host,
        // device *hid.Device,
        stream: &mut impl OpenRGBWritableStream,
    ) -> Option<OpenRGBError>
    where
        Self: Sized,
    {
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
trait PacketBody: OpenRGBReadable + OpenRGBWritable {
    async fn new(
        stream: &mut impl OpenRGBReadableStream,
        header: Header,
        protocol: u32,
    ) -> Result<Box<dyn RequestPacket>, OpenRGBError>
    where
        Self: 'static,
    {
        let mut p = Packet::<Self> {
            header: header,
            body: stream.read_value(protocol).await?,
        };
        p.read(stream).await?;
        Ok(Box::new(p))
    }
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
