use alloc::format;
use enum_primitive_derive::Primitive;
use num_traits::FromPrimitive;

use crate::protocol::{OpenRGBReadableSync, OpenRGBWritableSync};
use crate::OpenRGBError;
use crate::OpenRGBError::ProtocolError;
use crate::{OpenRGBReadable, OpenRGBWritable};

/// Direction for [Mode](crate::data::Mode).
#[derive(Primitive, Eq, PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    /// Left direction.
    Left = 0,

    /// Right direction.
    Right = 1,

    /// Up direction.
    Up = 2,

    /// Down direction.
    Down = 3,

    /// Horizontal direction.
    Horizontal = 4,

    /// Vertical direction.
    Vertical = 5,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Left
    }
}

impl OpenRGBWritable for Direction {
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

impl OpenRGBReadable for Direction {
    fn read(stream: &mut impl OpenRGBReadableSync, protocol: u32) -> Result<Self, OpenRGBError> {
        stream.read_value(protocol).and_then(|id| {
            Direction::from_u32(id).ok_or_else(|| ProtocolError(format!("invalid direction")))
        })
    }
}

// #[cfg(test)]
// mod tests {
//     use std::error::Error;

//     use crate::data::Direction;
//     use tokio_test::io::Builder;

//     use crate::protocol::{OpenRGBReadableStream, OpenRGBWritableStream};
//     use crate::tests::setup;
//     use crate::DEFAULT_PROTOCOL;

//     #[tokio::test]
//     async fn test_read_001() -> Result<(), Box<dyn Error>> {
//         setup()?;

//         let mut stream = Builder::new().read(&4_u32.to_le_bytes()).build();

//         assert_eq!(
//             stream.read_value::<Direction>(DEFAULT_PROTOCOL).await?,
//             Direction::Horizontal
//         );

//         Ok(())
//     }

//     #[tokio::test]
//     async fn test_write_001() -> Result<(), Box<dyn Error>> {
//         setup()?;

//         let mut stream = Builder::new().write(&4_u32.to_le_bytes()).build();

//         stream
//             .write_value(Direction::Horizontal, DEFAULT_PROTOCOL)
//             .await?;

//         Ok(())
//     }
// }
