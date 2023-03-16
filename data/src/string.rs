use smallvec::{smallvec, SmallVec};

use crate::protocol::{OpenRGBReadableSync, OpenRGBWritableSync};
use crate::OpenRGBError;
use crate::{OpenRGBReadable, OpenRGBWritable};

// FIXME buggy for non ASCII strings

// this seems bad...
const MAX_STRING: usize = 0x1000;

#[derive(Debug, Eq, PartialEq)]
pub struct OpenRGBString {
    data: SmallVec<[u8; MAX_STRING]>,
    len: u16,
}

impl OpenRGBWritable for OpenRGBString {
    fn size(&self, _protocol: u32) -> usize {
        // len, str, null
        2 + self.len.try_into().unwrap_or(usize::MAX) + 1
    }

    fn write(
        self,
        stream: &mut impl OpenRGBWritableSync,
        protocol: u32,
    ) -> Result<(), OpenRGBError> {
        stream.write_value((self.len + 1) as u16, protocol)?;
        stream.write_value(self.data, protocol)
    }
}

impl OpenRGBReadable for OpenRGBString {
    fn read(stream: &mut impl OpenRGBReadableSync, protocol: u32) -> Result<Self, OpenRGBError> {
        let len = stream.read_value::<u16>(protocol)?;
        // 1k should be enough for everybody
        let mut buf: SmallVec<[u8; MAX_STRING]> = smallvec![0u8; len as usize];
        stream
            .read_exact(&mut buf)
            .map_err(|_| OpenRGBError::CommunicationError())?;
        buf.pop();
        Ok(OpenRGBString {
            data: buf,
            len: len,
        })
        // core::str::from_utf8(&buf).map_err(|_| OpenRGBError::CommunicationError())
    }
}

// pub struct RawString(&'a str);

// impl<'a> OpenRGBWritable for RawString<'a> {
//     fn size(&self, _protocol: u32) -> usize {
//         self.0.len() + 1
//     }

//     fn write(
//         self,
//         stream: &mut impl OpenRGBWritableSync,
//         protocol: u32,
//     ) -> Result<(), OpenRGBError> {
//         stream
//             .write_all(self.0.as_bytes())
//             .map_err(|_| OpenRGBError::CommunicationError())?;
//         stream.write_value::<u8>(0, protocol)
//     }
// }

// #[cfg(test)]
// mod tests {
//     use std::error::Error;

//     use tokio_test::io::Builder;

//     use crate::data::RawString;
//     use crate::protocol::{OpenRGBReadableStream, OpenRGBWritableStream};
//     use crate::tests::setup;
//     use crate::DEFAULT_PROTOCOL;

//     #[tokio::test]
//     async fn test_read_001() -> Result<(), Box<dyn Error>> {
//         setup()?;

//         let mut stream = Builder::new()
//             .read(&5_u16.to_le_bytes())
//             .read(b"test\0")
//             .build();

//         assert_eq!(
//             stream.read_value::<String>(DEFAULT_PROTOCOL).await?,
//             "test".to_string()
//         );

//         Ok(())
//     }

//     #[tokio::test]
//     async fn test_write_001() -> Result<(), Box<dyn Error>> {
//         setup()?;

//         let mut stream = Builder::new()
//             .write(&5_u16.to_le_bytes())
//             .write(b"test\0")
//             .build();

//         stream
//             .write_value("test".to_string(), DEFAULT_PROTOCOL)
//             .await?;

//         Ok(())
//     }

//     #[tokio::test]
//     async fn test_write_raw_001() -> Result<(), Box<dyn Error>> {
//         setup()?;

//         let mut stream = Builder::new().write(b"test\0").build();

//         stream
//             .write_value(RawString("test".to_string()), DEFAULT_PROTOCOL)
//             .await?;

//         Ok(())
//     }
// }
