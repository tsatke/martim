use core::convert::TryFrom;

pub mod ext2;
pub mod fd;
pub mod vfs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FsId(u32);

pub trait FileSystem {
    fn fsid(&self) -> FsId;
    fn initialize(&mut self) -> bool { true }

    fn is_read_only(&self) -> bool;

    fn total_block_count(&self) -> usize;
    fn free_block_count(&self) -> usize;
    fn total_inode_count(&self) -> usize;
    fn free_inode_count(&self) -> usize;

    fn flush(&self);
}

impl From<u32> for FsId {
    fn from(fsid: u32) -> Self {
        FsId(fsid)
    }
}