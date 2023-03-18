#![cfg_attr(not(test), no_std)]
//! OpenRGB data types.
//!
//! See [OpenRGB SDK documentation](https://gitlab.com/CalcProgrammer1/OpenRGB/-/wikis/OpenRGB-SDK-Documentation) for more information.
extern crate alloc;

pub use color::*;
pub use color_mode::*;
pub use controller::*;
pub use device_type::*;
pub use direction::*;
pub use led::*;
pub use mode::*;
pub use mode_flag::*;
#[doc(hidden)]
pub use packet::*;
pub use primitive::*;
pub use string::*;
pub use tuple::*;
pub use vec::*;
pub use zone::*;
pub use zone_type::*;

use crate::protocol::{OpenRGBReadableSync, OpenRGBWritableSync};
use error::OpenRGBError;

mod color;
mod color_mode;
mod controller;
mod device_type;
mod direction;
mod error;
mod led;
mod mode;
mod mode_flag;
mod packet;
mod primitive;
mod protocol;
mod string;
mod tuple;
mod vec;
mod zone;
mod zone_type;

#[doc(hidden)]
pub trait OpenRGBReadable: Sized + Send + Sync {
    fn read(stream: &mut impl OpenRGBReadableSync, protocol: u32) -> Result<Self, OpenRGBError>;
}

#[doc(hidden)]
pub trait OpenRGBWritable: Sized + Send + Sync {
    fn size(&self, protocol: u32) -> usize;
    fn write(
        self,
        stream: &mut impl OpenRGBWritableSync,
        protocol: u32,
    ) -> Result<(), OpenRGBError>;
}
