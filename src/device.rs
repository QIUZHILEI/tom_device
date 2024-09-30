use crate::DeviceError;
use core::fmt::Debug;
#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub enum DeviceStatus {
    Offline,
    Initializing,
    Error,
    Idle,
    Transfer,
    Suspended,
    Removed,
    Resetting,
}

#[derive(Debug, Clone, Copy)]
pub enum DeviceType {
    Block,
    Char,
}

pub trait Device: Sync {
    fn init(&mut self) -> Result<(), DeviceError>;
    fn close(&mut self) -> Result<(), DeviceError>;
    fn reinit(&mut self) -> Result<(), DeviceError>;
    fn status(&self) -> DeviceStatus;
    fn device_type(&self) -> DeviceType;
    fn error_handle(&self) -> DeviceStatus;
}
