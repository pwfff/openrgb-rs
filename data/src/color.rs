use crate::protocol::{OpenRGBReadableSync, OpenRGBWritableSync};
use crate::OpenRGBError;
use crate::{OpenRGBReadable, OpenRGBWritable};

/// RGB controller color, aliased to [rgb] crate's [RGB8] type.
///
/// See [Open SDK documentation](https://gitlab.com/CalcProgrammer1/OpenRGB/-/wikis/OpenRGB-SDK-Documentation) for more information.
#[derive(Default, Debug, Eq, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl OpenRGBReadable for Color {
    fn read(stream: &mut impl OpenRGBReadableSync, protocol: u32) -> Result<Self, OpenRGBError> {
        let r = stream.read_value(protocol)?;
        let g = stream.read_value(protocol)?;
        let b = stream.read_value(protocol)?;
        let _padding = stream.read_value::<u8>(protocol)?;
        Ok(Color { r, g, b })
    }
}

impl OpenRGBWritable for Color {
    fn size(&self, _protocol: u32) -> usize {
        4 * 1
    }

    fn write(
        self,
        stream: &mut impl OpenRGBWritableSync,
        protocol: u32,
    ) -> Result<(), OpenRGBError> {
        stream.write_value(self.r, protocol)?;
        stream.write_value(self.g, protocol)?;
        stream.write_value(self.b, protocol)?;
        stream.write_value(0u8, protocol)?;
        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use std::error::Error;

//     use tokio_test::io::Builder;

//     use crate::data::Color;
//     use crate::protocol::{OpenRGBReadableStream, OpenRGBWritableStream};
//     use crate::tests::setup;
//     use crate::DEFAULT_PROTOCOL;

//     #[tokio::test]
//     async fn test_read_001() -> Result<(), Box<dyn Error>> {
//         setup()?;

//         let mut stream = Builder::new().read(&[37_u8, 54_u8, 126_u8, 0_u8]).build();

//         assert_eq!(
//             stream.read_value::<Color>(DEFAULT_PROTOCOL).await?,
//             Color {
//                 r: 37,
//                 g: 54,
//                 b: 126
//             }
//         );

//         Ok(())
//     }

//     #[tokio::test]
//     async fn test_write_001() -> Result<(), Box<dyn Error>> {
//         setup()?;

//         let mut stream = Builder::new().write(&[37_u8, 54_u8, 126_u8, 0_u8]).build();

//         stream
//             .write_value(
//                 Color {
//                     r: 37,
//                     g: 54,
//                     b: 126,
//                 },
//                 DEFAULT_PROTOCOL,
//             )
//             .await?;

//         Ok(())
//     }
// }
