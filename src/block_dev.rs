use crate::{Device, DeviceError};
use core::fmt::Debug;

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum SectorSize {
    Lb512,
    Lb4096,
}

pub trait BlockDevice: Device + Sync {
    fn physical_block_size(&self) -> usize;
    fn read_block(&mut self, lba: usize, buf: &mut [u8]) -> Result<(), DeviceError>;
    fn write_block(&self, lba: usize, data: &[u8]) -> Result<(), DeviceError>;
    fn sector_size(&self) -> SectorSize;
}