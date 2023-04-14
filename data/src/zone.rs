use alloc::format;
use alloc::string::String;
use smallvec::SmallVec;

use crate::protocol::OpenRGBReadableSync;
use crate::{OpenRGBError, OpenRGBWritable};
use crate::{OpenRGBReadable, ZoneType};

const MAX_LEDS: usize = 2048;

#[derive(Debug, Eq, PartialEq)]
pub struct Matrix {
    len: u16,

    height: u32,

    width: u32,

    data: SmallVec<[u32; MAX_LEDS]>,
}

/// RGB controller zone.
///
/// See [Open SDK documentation](https://gitlab.com/CalcProgrammer1/OpenRGB/-/wikis/OpenRGB-SDK-Documentation#zone-data) for more information.
#[derive(Debug, Eq, PartialEq)]
pub struct Zone {
    /// Zone name.
    pub name: String,

    /// Zone type.
    pub r#type: ZoneType,

    /// Zone minimum LED number.
    pub leds_min: u32,

    /// Zone maximum LED number.
    pub leds_max: u32,

    /// Zone LED count.
    pub leds_count: u32,

    /// Zone LED matrix (if [Zone::type] is [ZoneType::Matrix]).
    pub matrix: Option<Matrix>,
}

impl OpenRGBReadable for Zone {
    fn read(stream: &mut impl OpenRGBReadableSync, protocol: u32) -> Result<Self, OpenRGBError> {
        let name = stream.read_value(protocol)?;
        let r#type = stream.read_value(protocol)?;
        let leds_min = stream.read_value(protocol)?;
        let leds_max = stream.read_value(protocol)?;
        let leds_count = stream.read_value(protocol)?;
        let matrix_len = stream.read_value::<u16>(protocol)? as usize;
        let matrix = match matrix_len {
            0 => None,
            _ => Some({
                let matrix_height = stream.read_value::<u32>(protocol)?;
                let matrix_width = stream.read_value::<u32>(protocol)?;
                let matrix_size: u16 = (matrix_height * matrix_width).try_into().map_err(|_| {
                    OpenRGBError::CommunicationError(format!("failed reading matrix size"))
                })?;
                let mut matrix_data = SmallVec::with_capacity(matrix_size.into());
                for _ in 0..matrix_size {
                    matrix_data.push(stream.read_value(protocol)?);
                }
                Matrix {
                    len: matrix_size,
                    height: matrix_height,
                    width: matrix_width,
                    data: matrix_data,
                }
            }),
        };
        Ok(Zone {
            name,
            r#type,
            leds_min,
            leds_max,
            leds_count,
            matrix,
        })
    }
}

impl OpenRGBWritable for Zone {
    fn size(&self, protocol: u32) -> usize {
        let mut size = 0;
        size += self.name.size(protocol);
        size += self.r#type.size(protocol);
        size += self.leds_min.size(protocol);
        size += self.leds_max.size(protocol);
        size += self.leds_count.size(protocol);

        match &self.matrix {
            None => {
                size += 0_u16.size(protocol);
            }
            Some(matrix) => {
                size += matrix.len.size(protocol);
                size += matrix.height.size(protocol);
                size += matrix.width.size(protocol);
                size += 0_u32.size(protocol) * matrix.data.len();
            }
        };

        size
    }

    fn write(
        self,
        stream: &mut impl crate::OpenRGBWritableSync,
        protocol: u32,
    ) -> Result<(), OpenRGBError> {
        stream.write_value(self.name, protocol)?;
        stream.write_value(self.r#type, protocol)?;
        stream.write_value(self.leds_min, protocol)?;
        stream.write_value(self.leds_max, protocol)?;
        stream.write_value(self.leds_count, protocol)?;

        match self.matrix {
            None => {
                stream.write_value::<u16>(0, protocol)?;
            }
            Some(matrix) => {
                stream.write_value::<u16>(matrix.len, protocol)?;
                stream.write_value::<u32>(matrix.height, protocol)?;
                stream.write_value::<u32>(matrix.width, protocol)?;
                for data in matrix.data {
                    stream.write_value(data, protocol)?;
                }
            }
        };

        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use std::error::Error;

//     use array2d::Array2D;
//     use tokio_test::io::Builder;

//     use crate::data::{Zone, ZoneType};
//     use crate::protocol::OpenRGBReadableStream;
//     use crate::tests::setup;
//     use crate::DEFAULT_PROTOCOL;

//     #[tokio::test]
//     async fn test_read_001() -> Result<(), Box<dyn Error>> {
//         setup()?;

//         let mut stream = Builder::new()
//             .read(&5_u16.to_le_bytes()) // name len
//             .read(b"test\0") // name
//             .read(&1_u32.to_le_bytes()) // type
//             .read(&3_u32.to_le_bytes()) // leds_min
//             .read(&18_u32.to_le_bytes()) // leds_max
//             .read(&15_u32.to_le_bytes()) // leds_count
//             .read(&0_u16.to_le_bytes()) // matrix_len
//             .build();

//         assert_eq!(
//             stream.read_value::<Zone>(DEFAULT_PROTOCOL).await?,
//             Zone {
//                 name: "test".to_string(),
//                 r#type: ZoneType::Linear,
//                 leds_min: 3,
//                 leds_max: 18,
//                 leds_count: 15,
//                 matrix: None,
//             }
//         );

//         Ok(())
//     }

//     #[tokio::test]
//     async fn test_read_002() -> Result<(), Box<dyn Error>> {
//         setup()?;

//         let mut stream = Builder::new()
//             .read(&5_u16.to_le_bytes()) // name len
//             .read(b"test\0") // name
//             .read(&1_u32.to_le_bytes()) // type
//             .read(&3_u32.to_le_bytes()) // leds_min
//             .read(&18_u32.to_le_bytes()) // leds_max
//             .read(&15_u32.to_le_bytes()) // leds_count
//             .read(&32_u16.to_le_bytes()) // matrix_len
//             .read(&2_u32.to_le_bytes()) // matrix_height
//             .read(&3_u32.to_le_bytes()) // matrix_width
//             .read(&0_u32.to_le_bytes()) // matrix[0]
//             .read(&1_u32.to_le_bytes()) // matrix[1]
//             .read(&2_u32.to_le_bytes()) // matrix[2]
//             .read(&3_u32.to_le_bytes()) // matrix[3]
//             .read(&4_u32.to_le_bytes()) // matrix[4]
//             .read(&5_u32.to_le_bytes()) // matrix[5]
//             .build();

//         assert_eq!(
//             stream.read_value::<Zone>(DEFAULT_PROTOCOL).await?,
//             Zone {
//                 name: "test".to_string(),
//                 r#type: ZoneType::Linear,
//                 leds_min: 3,
//                 leds_max: 18,
//                 leds_count: 15,
//                 matrix: Some(Array2D::from_rows(&[vec![0, 1, 2], vec![3, 4, 5]])),
//             }
//         );

//         Ok(())
//     }
// }
