//! OpenRGB data types.
//!
//! See [OpenRGB SDK documentation](https://gitlab.com/CalcProgrammer1/OpenRGB/-/wikis/OpenRGB-SDK-Documentation) for more information.
use async_trait::async_trait;

pub use color::*;
pub use color_mode::*;
pub use controller::*;
pub use device_type::*;
pub use direction::*;
pub use led::*;
pub use mode::*;
pub use mode_flag::*;
#[doc(hidden)]
pub use packet_id::*;
pub use primitive::*;
pub use string::*;
pub use tuple::*;
pub use vec::*;
pub use zone::*;
pub use zone_type::*;

use crate::protocol::{OpenRGBReadableStream, OpenRGBStream, OpenRGBWritableStream};
use crate::OpenRGBError;

mod color;
mod color_mode;
mod controller;
mod device_type;
mod direction;
mod led;
mod mode;
mod mode_flag;
pub mod packet;
mod packet_id;
mod primitive;
mod string;
mod tuple;
mod vec;
mod zone;
mod zone_type;

#[async_trait]
#[doc(hidden)]
pub trait OpenRGBReadable: Sized + Send + Sync {
    async fn read(
        stream: &mut impl OpenRGBReadableStream,
        protocol: u32,
    ) -> Result<Self, OpenRGBError>;
}

#[async_trait]
#[doc(hidden)]
pub trait OpenRGBWritable: Sized + Send + Sync {
    fn size(&self, protocol: u32) -> usize;
    async fn write(
        self,
        stream: &mut impl OpenRGBWritableStream,
        protocol: u32,
    ) -> Result<(), OpenRGBError>;
}
