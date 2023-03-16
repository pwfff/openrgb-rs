use enum_primitive_derive::Primitive;
use num_traits::FromPrimitive;

use crate::protocol::{OpenRGBReadableSync, OpenRGBWritableSync};
use crate::OpenRGBError;
use crate::OpenRGBError::ProtocolError;
use crate::{OpenRGBReadable, OpenRGBWritable};

/// RGB controller [Zone](crate::data::Zone) type.
///
/// See [Open SDK documentation](https://gitlab.com/CalcProgrammer1/OpenRGB/-/wikis/OpenRGB-SDK-Documentation#zone-data) for more information.
#[derive(Primitive, Eq, PartialEq, Debug, Copy, Clone)]
pub enum ZoneType {
    /// Single zone.
    Single = 0,

    /// Linear zone.
    Linear = 1,

    /// Matrix zone.
    Matrix = 2,
}

impl OpenRGBWritable for ZoneType {
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

impl OpenRGBReadable for ZoneType {
    fn read(stream: &mut impl OpenRGBReadableSync, protocol: u32) -> Result<Self, OpenRGBError> {
        stream
            .read_value(protocol)
            .and_then(|id| ZoneType::from_u32(id).ok_or_else(|| ProtocolError()))
    }
}

// #[cfg(test)]
// mod tests {
//     use std::error::Error;

//     use tokio_test::io::Builder;

//     use crate::data::ZoneType;
//     use crate::DEFAULT_PROTOCOL;
//     use crate::protocol::{OpenRGBReadableStream, OpenRGBWritableStream};
//     use crate::tests::setup;

//     #[tokio::test]
//     async fn test_read_001() -> Result<(), Box<dyn Error>> {
//         setup()?;

//         let mut stream = Builder::new()
//             .read(&1_u32.to_le_bytes())
//             .build();

//         assert_eq!(stream.read_value::<ZoneType>(DEFAULT_PROTOCOL).await?, ZoneType::Linear);

//         Ok(())
//     }

//     #[tokio::test]
//     async fn test_write_001() -> Result<(), Box<dyn Error>> {
//         setup()?;

//         let mut stream = Builder::new()
//             .write(&1_u32.to_le_bytes())
//             .build();

//         stream.write_value(ZoneType::Linear, DEFAULT_PROTOCOL).await?;

//         Ok(())
//     }
// }
