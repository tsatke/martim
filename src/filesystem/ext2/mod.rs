use crate::filesystem::{FileDescriptor, FileSystem, FsId, Mode, OpenFlags};
use crate::syscall::error::Errno;
use alloc::boxed::Box;

pub mod superblock;

pub struct Ext2FileSystem {
    fsid: FsId,
}

impl FileSystem for Ext2FileSystem {
    fn fsid(&self) -> FsId {
        self.fsid
    }

    fn initialize(&self) -> bool {
        true
    }

    fn is_read_only(&self) -> bool {
        todo!()
    }

    fn open(
        &self,
        path: &'static str,
        mode: Mode,
        flags: OpenFlags,
    ) -> Result<Box<dyn FileDescriptor>, Errno> {
        todo!()
    }

    fn mkdir(&self, path: &str, mode: Mode) -> Result<(), Errno> {
        todo!()
    }

    fn rmdir(&self, path: &str) -> Result<(), Errno> {
        todo!()
    }

    fn flush(&self) {
        todo!()
    }
}
