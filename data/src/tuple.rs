use crate::protocol::{OpenRGBReadableSync, OpenRGBWritableSync};
use crate::OpenRGBError;
use crate::{OpenRGBReadable, OpenRGBWritable};

impl<A: OpenRGBWritable, B: OpenRGBWritable> OpenRGBWritable for (A, B) {
    fn size(&self, protocol: u32) -> usize {
        self.0.size(protocol) + self.1.size(protocol)
    }

    fn write(
        self,
        stream: &mut impl OpenRGBWritableSync,
        protocol: u32,
    ) -> Result<(), OpenRGBError> {
        stream.write_value(self.0, protocol)?;
        stream.write_value(self.1, protocol)?;
        Ok(())
    }
}

impl<A: OpenRGBReadable, B: OpenRGBReadable> OpenRGBReadable for (A, B) {
    fn read(stream: &mut impl OpenRGBReadableSync, protocol: u32) -> Result<Self, OpenRGBError> {
        Ok((
            stream.read_value::<A>(protocol)?,
            stream.read_value::<B>(protocol)?,
        ))
    }
}

impl<A: OpenRGBWritable, B: OpenRGBWritable, C: OpenRGBWritable> OpenRGBWritable for (A, B, C) {
    fn size(&self, protocol: u32) -> usize {
        self.0.size(protocol) + self.1.size(protocol) + self.2.size(protocol)
    }

    fn write(
        self,
        stream: &mut impl OpenRGBWritableSync,
        protocol: u32,
    ) -> Result<(), OpenRGBError> {
        stream.write_value(self.0, protocol)?;
        stream.write_value(self.1, protocol)?;
        stream.write_value(self.2, protocol)?;
        Ok(())
    }
}

impl<A: OpenRGBReadable, B: OpenRGBReadable, C: OpenRGBReadable> OpenRGBReadable for (A, B, C) {
    fn read(stream: &mut impl OpenRGBReadableSync, protocol: u32) -> Result<Self, OpenRGBError> {
        Ok((
            stream.read_value::<A>(protocol)?,
            stream.read_value::<B>(protocol)?,
            stream.read_value::<C>(protocol)?,
        ))
    }
}

impl<A: OpenRGBWritable, B: OpenRGBWritable, C: OpenRGBWritable, D: OpenRGBWritable> OpenRGBWritable
    for (A, B, C, D)
{
    fn size(&self, protocol: u32) -> usize {
        self.0.size(protocol)
            + self.1.size(protocol)
            + self.2.size(protocol)
            + self.3.size(protocol)
    }

    fn write(
        self,
        stream: &mut impl OpenRGBWritableSync,
        protocol: u32,
    ) -> Result<(), OpenRGBError> {
        stream.write_value(self.0, protocol)?;
        stream.write_value(self.1, protocol)?;
        stream.write_value(self.2, protocol)?;
        stream.write_value(self.3, protocol)?;
        Ok(())
    }
}

impl<A: OpenRGBReadable, B: OpenRGBReadable, C: OpenRGBReadable, D: OpenRGBReadable> OpenRGBReadable
    for (A, B, C, D)
{
    fn read(stream: &mut impl OpenRGBReadableSync, protocol: u32) -> Result<Self, OpenRGBError> {
        Ok((
            stream.read_value::<A>(protocol)?,
            stream.read_value::<B>(protocol)?,
            stream.read_value::<C>(protocol)?,
            stream.read_value::<D>(protocol)?,
        ))
    }
}

// #[cfg(test)]
// mod tests {
//     use std::error::Error;

//     use tokio_test::io::Builder;

//     use crate::data::DeviceType;
//     use crate::protocol::{OpenRGBReadableStream, OpenRGBWritableStream};
//     use crate::tests::setup;
//     use crate::DEFAULT_PROTOCOL;

//     #[tokio::test]
//     async fn test_read_001() -> Result<(), Box<dyn Error>> {
//         setup()?;

//         let mut stream = Builder::new()
//             .read(&37_u8.to_le_bytes())
//             .read(&1337_u32.to_le_bytes())
//             .read(&(-1337_i32).to_le_bytes())
//             .read(&4_u32.to_le_bytes())
//             .build();

//         assert_eq!(
//             stream
//                 .read_value::<(u8, u32, i32, DeviceType)>(DEFAULT_PROTOCOL)
//                 .await?,
//             (37, 1337, -1337, DeviceType::LEDStrip)
//         );

//         Ok(())
//     }

//     #[tokio::test]
//     async fn test_write_001() -> Result<(), Box<dyn Error>> {
//         setup()?;

//         let mut stream = Builder::new()
//             .write(&37_u8.to_le_bytes())
//             .write(&1337_u32.to_le_bytes())
//             .write(&(-1337_i32).to_le_bytes())
//             .write(&4_u32.to_le_bytes())
//             .build();

//         stream
//             .write_value(
//                 (37_u8, 1337_u32, (-1337_i32), DeviceType::LEDStrip),
//                 DEFAULT_PROTOCOL,
//             )
//             .await?;

//         Ok(())
//     }
// }
