use crate::filesystem::{FileSystem, FsId};

pub struct Ext2Fs {}


impl FileSystem for Ext2Fs {
    fn fsid(&self) -> FsId {
        todo!()
    }

    fn is_read_only(&self) -> bool {
        todo!()
    }

    fn total_block_count(&self) -> usize {
        todo!()
    }

    fn free_block_count(&self) -> usize {
        todo!()
    }

    fn total_inode_count(&self) -> usize {
        todo!()
    }

    fn free_inode_count(&self) -> usize {
        todo!()
    }

    fn flush(&self) {
        todo!()
    }
}