use alloc::collections::LinkedList;
use alloc::sync::Arc;

use spin::Mutex;

use crate::filesystem::{FileSystem, FsId};
use crate::filesystem::fd::FileDescriptor;
use crate::filesystem::vfs::flags::{Mode, MountFlags, OpenFlags};
use crate::filesystem::vfs::mount::Mount;
use crate::syscall::error::{Errno, Result};

pub mod flags;
mod mount;
#[cfg(test)]
mod test_fs;

pub fn init() {}

pub struct Vfs<'a> {
    mounts: Mutex<LinkedList<Mount<'a>>>,
}

impl<'a> Vfs<'a> {
    pub fn new() -> Self {
        Vfs {
            mounts: Mutex::new(LinkedList::new()),
        }
    }

    pub fn mount(&mut self, file_system: &'a dyn FileSystem, flags: MountFlags) -> Result<(), Errno> {
        let mount = Mount::new(file_system, flags);
        self.mounts.lock().push_front(mount);
        Ok(())
    }

    pub fn mount_count(&self) -> usize { self.mounts.lock().len() }

    pub fn unmount(&mut self) -> Result<(), Errno> { // still missing an argument
        todo!()
    }

    pub fn open(_path: &str, _mode: Mode, _flags: OpenFlags) -> Result<&Arc<FileDescriptor>, Errno> {
        todo!()
    }

    pub fn mkdir(_path: &str, _mode: Mode) -> Result<(), Errno> {
        todo!()
    }

    pub fn rmdir(_path: &str) -> Result<(), Errno> {
        todo!()
    }
}

impl<'a> FileSystem for Vfs<'a> {
    fn fsid(&self) -> FsId {
        todo!()
    }

    fn is_read_only(&self) -> bool { false }

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

#[cfg(test)]
mod tests {
    use crate::filesystem::vfs::test_fs::TestFs;

    use super::*;

    #[test_case]
    fn test_initialize_called() {
        let mut fs = TestFs::from(7.into());
        assert!(fs.initialize());
        assert_eq!(1, fs.init_called_count());

        let mut vfs = Vfs::new();
        assert_eq!(0, vfs.mount_count());

        vfs.mount(&fs, MountFlags::NONE);

        assert_eq!(1, fs.init_called_count()); // vfs must not call initialize on mount
        assert_eq!(1, vfs.mount_count());
    }
}