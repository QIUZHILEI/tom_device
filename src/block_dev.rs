use crate::{Device, DeviceError};
use core::fmt::Debug;

pub trait BlockDevice: Device + Sync {
    fn block_size(&self) -> usize;
    fn read_block(&mut self, lba: usize, buf: &mut [u8]) -> Result<(), DeviceError>;
    fn write_block(&self, lba: usize, data: &[u8]) -> Result<(), DeviceError>;
    fn information(&self) -> &dyn BlkDevInfo;
}

pub trait BlkDevInfo: Debug {}
