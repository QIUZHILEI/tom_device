use core::fmt::{Debug, Write};

use crate::{Device, DeviceError};

pub trait CharDevice: Device + Write + Sync {
    fn get_char(&self) -> Result<u8, DeviceError>;
    fn put_char(&self, data: u8) -> Result<(), DeviceError>;
}
