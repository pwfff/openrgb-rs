use crate::protocol::OpenRGBReadableSync;
use crate::OpenRGBError;
use crate::OpenRGBReadable;
use crate::OpenRGBString;

/// A single LED.
#[derive(Debug, Eq, PartialEq)]
pub struct LED {
    /// LED name.
    pub name: OpenRGBString,

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
