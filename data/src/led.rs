use alloc::string::String;

use crate::protocol::OpenRGBReadableSync;
use crate::OpenRGBError;
use crate::OpenRGBReadable;
use crate::OpenRGBWritable;

/// A single LED.
#[derive(Debug, Eq, PartialEq)]
pub struct LED {
    /// LED name.
    pub name: String,

    /// LED value.
    pub value: u32,
}

impl OpenRGBReadable for LED {
    fn read(stream: &mut impl OpenRGBReadableSync, protocol: u32) -> Result<Self, OpenRGBError> {
        Ok(LED {
            name: stream.read_value(protocol)?,
            value: stream.read_value(protocol)?,
        })
    }
}

impl OpenRGBWritable for LED {
    fn size(&self, protocol: u32) -> usize {
        self.name.size(protocol) + self.value.size(protocol)
    }

    fn write(
        self,
        stream: &mut impl crate::OpenRGBWritableSync,
        protocol: u32,
    ) -> Result<(), OpenRGBError> {
        stream.write_value(self.name, protocol)?;
        stream.write_value(self.value, protocol)?;

        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use std::error::Error;

//     use tokio_test::io::Builder;

//     use crate::data::LED;
//     use crate::protocol::OpenRGBReadableStream;
//     use crate::tests::setup;
//     use crate::DEFAULT_PROTOCOL;

//     #[tokio::test]
//     async fn test_read_001() -> Result<(), Box<dyn Error>> {
//         setup()?;

//         let mut stream = Builder::new()
//             .read(&5_u16.to_le_bytes())
//             .read(b"test\0")
//             .read(&45_u32.to_le_bytes())
//             .build();

//         assert_eq!(
//             stream.read_value::<LED>(DEFAULT_PROTOCOL).await?,
//             LED {
//                 name: "test".to_string(),
//                 value: 45
//             }
//         );

//         Ok(())
//     }
// }
