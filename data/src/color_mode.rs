use alloc::format;
use enum_primitive_derive::Primitive;
use num_traits::FromPrimitive;

use crate::protocol::{OpenRGBReadableSync, OpenRGBWritableSync};
use crate::OpenRGBError;
use crate::OpenRGBError::ProtocolError;
use crate::{OpenRGBReadable, OpenRGBWritable};

/// RGB controller color mode.
///
/// See [Open SDK documentation](https://gitlab.com/CalcProgrammer1/OpenRGB/-/wikis/OpenRGB-SDK-Documentation) for more information.
#[derive(Primitive, Eq, PartialEq, Debug, Copy, Clone)]
pub enum ColorMode {
    /// No color mode.
    None = 0,

    /// Per LED colors.
    PerLED = 1,

    /// Mode specific colors.
    ModeSpecific = 2,

    /// Random colors.
    Random = 3,
}

impl Default for ColorMode {
    fn default() -> Self {
        ColorMode::None
    }
}

impl OpenRGBWritable for ColorMode {
    fn size(&self, _protocol: u32) -> usize {
        4 * 1
    }

    fn write(
        self,
        stream: &mut impl OpenRGBWritableSync,
        protocol: u32,
    ) -> Result<(), OpenRGBError> {
        stream.write_value(self as u32, protocol)
    }
}

impl OpenRGBReadable for ColorMode {
    fn read(stream: &mut impl OpenRGBReadableSync, protocol: u32) -> Result<Self, OpenRGBError> {
        stream.read_value(protocol).and_then(|id| {
            ColorMode::from_u32(id)
                .ok_or_else(|| ProtocolError(format!("error reading color mode")))
        })
    }
}

// #[cfg(test)]
// mod tests {
//     use std::error::Error;

//     use tokio_test::io::Builder;

//     use crate::data::ColorMode;
//     use crate::protocol::{OpenRGBReadableStream, OpenRGBWritableStream};
//     use crate::tests::setup;
//     use crate::DEFAULT_PROTOCOL;

//     #[tokio::test]
//     async fn test_read_001() -> Result<(), Box<dyn Error>> {
//         setup()?;

//         let mut stream = Builder::new().read(&3u32.to_le_bytes()).build();

//         assert_eq!(
//             stream.read_value::<ColorMode>(DEFAULT_PROTOCOL).await?,
//             ColorMode::Random
//         );

//         Ok(())
//     }

//     #[tokio::test]
//     async fn test_write_001() -> Result<(), Box<dyn Error>> {
//         setup()?;

//         let mut stream = Builder::new().write(&3u32.to_le_bytes()).build();

//         stream
//             .write_value(ColorMode::Random, DEFAULT_PROTOCOL)
//             .await?;

//         Ok(())
//     }
// }
