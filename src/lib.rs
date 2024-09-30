#![no_std]
#![feature(error_generic_member_access)]
#[cfg(feature = "block")]
mod block_dev;
#[cfg(feature = "char")]
mod char_dev;
mod device;
mod err;
mod register;
#[cfg(feature = "block")]
pub use block_dev::*;
#[cfg(feature = "char")]
pub use char_dev::*;
pub use device::*;
pub use err::DeviceError;
pub use register::*;
#[cfg(feature = "block")]
extern crate alloc;
