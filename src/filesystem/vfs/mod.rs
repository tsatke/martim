use alloc::collections::{BTreeMap, LinkedList};
use alloc::sync::Arc;

use spin::Mutex;

use crate::filesystem::fd::FileDescriptor;
use crate::filesystem::flags::{Mode, MountFlags, OpenFlags};
use crate::filesystem::{FileSystem, FsId};
use crate::syscall::error::{Errno, Result};
use alloc::boxed::Box;
use alloc::string::String;
use core::borrow::BorrowMut;

#[cfg(test)]
mod test_fs;

pub fn init() {}

pub struct Vfs {
    fsid: FsId,
    mounts: Mutex<LinkedList<Mount>>, // TODO: a prefix tree would probably be smart here
}

struct Mount {
    path: &'static str,
    file_system: Box<dyn FileSystem>,
}

impl Mount {
    fn new(path: &'static str, file_system: Box<dyn FileSystem>) -> Self {
        Mount { path, file_system }
    }
}

impl Vfs {
    pub fn new(fsid: FsId) -> Self {
        Vfs {
            fsid,
            mounts: Mutex::new(LinkedList::new()),
        }
    }

    pub fn mount(
        &self,
        path: &'static str,
        file_system: Box<dyn FileSystem>,
        _flags: MountFlags,
    ) -> Result<(), Errno> {
        self.mounts.lock().push_back(Mount::new(path, file_system));
        Ok(())
    }

    pub fn mount_count(&self) -> usize {
        self.mounts.lock().len()
    }

    pub fn unmount(&self, path: &str) -> Result<(), Errno> {
        // match self.mounts.lock().remove(path) {
        //     None => Err(Errno::EINVAL), // not a mount point
        //     Some(_) => Ok(()),
        // }
        Ok(())
    }

    fn find_file_system_for_path(&self, path: &str) -> Option<Box<dyn FileSystem>> {
        // TODO: prefix tree would make this way more efficient
        self.mounts
            .lock()
            .iter()
            .find(|&mount| path.starts_with(mount.path))
            .take()
            .map(|m| m.file_system)
            .take()
    }
}

impl FileSystem for Vfs {
    fn fsid(&self) -> FsId {
        self.fsid
    }

    fn is_read_only(&self) -> bool {
        false
    }

    fn open(&self, path: &str, mode: Mode, flags: OpenFlags) -> Result<FileDescriptor, Errno> {
        match self.find_file_system_for_path(path) {
            Some(fs) => fs.open(path, mode, flags),
            None => Err(Errno::ENOENT),
        }
    }

    fn mkdir(&self, path: &str, mode: Mode) -> Result<(), Errno> {
        match self.find_file_system_for_path(path) {
            Some(fs) => fs.mkdir(path, mode),
            None => Err(Errno::ENOENT),
        }
    }

    fn rmdir(&self, path: &str) -> Result<(), Errno> {
        match self.find_file_system_for_path(path) {
            Some(fs) => fs.rmdir(path),
            None => Err(Errno::ENOENT),
        }
    }

    fn flush(&self) {
        todo!()
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::filesystem::vfs::test_fs::TestFs;
//
//     use super::*;
//
//     #[test_case]
//     fn test_initialize_called() {
//         let mut fs = TestFs::from(7.into());
//         assert!(fs.initialize());
//         assert_eq!(1, fs.init_called_count());
//
//         let mut vfs = Vfs::new(FsId::from(0));
//         assert_eq!(0, vfs.mount_count());
//
//         assert!(vfs.mount("/", &fs, MountFlags::NONE).is_ok());
//
//         assert_eq!(1, fs.init_called_count()); // vfs must not call initialize on mount
//         assert_eq!(1, vfs.mount_count());
//     }
//
//     #[test_case]
//     fn test_mount_unmount() {
//         let fs = TestFs::from(19.into());
//
//         let mut vfs = Vfs::new(FsId::from(0));
//         assert_eq!(0, vfs.mount_count());
//
//         assert!(vfs.mount("/", &fs, MountFlags::NONE).is_ok());
//         assert_eq!(1, vfs.mount_count());
//
//         assert!(vfs.unmount("/").is_ok());
//         assert_eq!(0, vfs.mount_count());
//
//         assert_eq!(Err(Errno::EINVAL), vfs.unmount("/"));
//         assert_eq!(0, vfs.mount_count());
//     }
//
//     #[test_case]
//     fn test_unmount_wrong_dir() {
//         let fs = TestFs::from(19.into());
//
//         let mut vfs = Vfs::new(FsId::from(0));
//         assert_eq!(0, vfs.mount_count());
//
//         assert!(vfs.mount("/", &fs, MountFlags::NONE).is_ok());
//         assert_eq!(1, vfs.mount_count());
//
//         assert_eq!(Err(Errno::EINVAL), vfs.unmount("/foobar"));
//         assert_eq!(1, vfs.mount_count());
//
//         assert!(vfs.unmount("/").is_ok());
//         assert_eq!(0, vfs.mount_count());
//
//         assert_eq!(Err(Errno::EINVAL), vfs.unmount("/foobar"));
//         assert_eq!(0, vfs.mount_count());
//     }
// }
