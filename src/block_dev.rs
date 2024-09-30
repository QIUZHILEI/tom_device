use crate::{Device, DeviceError};
use core::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlockSize {
    Lb512 = 512,
    Lb4096 = 4096,
}

pub trait BlockDevice: Device + Sync {
    fn block_size(&self) -> BlockSize;
    fn read_block(&mut self, lba: usize, buf: &mut [u8]) -> Result<(), DeviceError>;
    fn write_block(&self, lba: usize, data: &[u8]) -> Result<(), DeviceError>;
    fn information(&self) -> &dyn BlkDevInfo;
}

pub trait BlkDevInfo: Debug {}
