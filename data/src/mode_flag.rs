use alloc::format;
use flagset::{flags, FlagSet};

use crate::protocol::{OpenRGBReadableSync, OpenRGBWritableSync};
use crate::OpenRGBError;
use crate::OpenRGBError::ProtocolError;
use crate::{OpenRGBReadable, OpenRGBWritable};

flags! {
    /// RGB controller mode flags.
    ///
    /// See [Open SDK documentation](https://gitlab.com/CalcProgrammer1/OpenRGB/-/wikis/OpenRGB-SDK-Documentation) for more information.
    pub enum ModeFlag: u32 {
        /// Mode has speed parameter.
        HasSpeed = 1 << 0,

        /// Mode has left/right parameter.
        HasDirectionLR = 1 << 1,

        /// Mode has up/down parameter.
        HasDirectionUD = 1 << 2,

        /// Mode has horiz/vert parameter.
        HasDirectionHV = 1 << 3,

        /// Mode has direction parameter.
        HasDirection = (ModeFlag::HasDirectionLR | ModeFlag::HasDirectionUD | ModeFlag::HasDirectionHV).bits(),

        /// Mode has brightness parameter.
        HasBrightness = 1 << 4,

        /// Mode has per-LED colors.
        HasPerLEDColor = 1 << 5,

        /// Mode has mode specific colors.
        HasModeSpecificColor = 1 << 6,

        /// Mode has random color option.
        HasRandomColor = 1 << 7,

        /// Mode can manually be saved.
        ManualSave = 1 << 8,

        /// Mode automatically saves.
        AutomaticSave = 1 << 9,
    }
}

impl OpenRGBWritable for FlagSet<ModeFlag> {
    fn size(&self, _protocol: u32) -> usize {
        4 * 1
    }

    fn write(
        self,
        stream: &mut impl OpenRGBWritableSync,
        protocol: u32,
    ) -> Result<(), OpenRGBError> {
        stream.write_value(self.bits(), protocol)
    }
}

impl OpenRGBReadable for FlagSet<ModeFlag> {
    fn read(stream: &mut impl OpenRGBReadableSync, protocol: u32) -> Result<Self, OpenRGBError> {
        let value = stream.read_value(protocol)?;
        FlagSet::<ModeFlag>::new(value).map_err(|e| ProtocolError(format!("{:?}: {}", e, value)))
    }
}

// #[cfg(test)]
// mod tests {
//     use std::error::Error;

//     use flagset::FlagSet;
//     use tokio_test::io::Builder;

//     use crate::data::ModeFlag;
//     use ModeFlag::*;

//     use crate::protocol::{OpenRGBReadableStream, OpenRGBWritableStream};
//     use crate::tests::setup;
//     use crate::DEFAULT_PROTOCOL;

//     #[tokio::test]
//     async fn test_read_001() -> Result<(), Box<dyn Error>> {
//         setup()?;

//         let mut stream = Builder::new().read(&154_u32.to_le_bytes()).build();

//         assert_eq!(
//             stream
//                 .read_value::<FlagSet<ModeFlag>>(DEFAULT_PROTOCOL)
//                 .await?,
//             HasDirectionLR | HasDirectionHV | HasBrightness | HasRandomColor
//         );

//         Ok(())
//     }

//     #[tokio::test]
//     async fn test_write_001() -> Result<(), Box<dyn Error>> {
//         setup()?;

//         let mut stream = Builder::new().write(&31_u32.to_le_bytes()).build();

//         stream
//             .write_value(HasDirection | HasSpeed | HasBrightness, DEFAULT_PROTOCOL)
//             .await?;

//         Ok(())
//     }
// }
