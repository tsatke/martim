use alloc::collections::BTreeMap;
use alloc::rc::Rc;

use spin::Mutex;

use crate::filesystem::file_descriptor::FileDescriptor;
use crate::filesystem::flags::{Mode, MountFlags, OpenFlags};
use crate::filesystem::{FileSystem, FsId};
use crate::syscall::error::{Errno, Result};

use alloc::boxed::Box;
use mount::Mount;

mod mount;
#[cfg(test)]
mod test_fs;

pub fn init() {}

pub struct Vfs {
    fsid: FsId,
    mounts: Mutex<BTreeMap<&'static str, Mount>>, // TODO: a prefix tree would probably be smart here
}

impl Vfs {
    pub fn new(fsid: FsId) -> Self {
        Vfs {
            fsid,
            mounts: Mutex::new(BTreeMap::<&str, Mount>::new()),
        }
    }

    pub fn mount(
        &self,
        path: &'static str,
        file_system: Box<dyn FileSystem>,
        _flags: MountFlags,
    ) -> Result<(), Errno> {
        match self
            .mounts
            .lock()
            .insert(path, Mount::new(path, file_system))
        {
            None => Ok(()),
            Some(_) => Err(Errno::EINVAL),
        }
    }

    pub fn mount_count(&self) -> usize {
        self.mounts.lock().len()
    }

    pub fn unmount(&self, path: &str) -> Result<(), Errno> {
        match self.mounts.lock().remove(path) {
            None => Err(Errno::EINVAL), // not a mount point
            Some(_) => Ok(()),
        }
    }

    fn find_file_system_for_path(&self, path: &str) -> Option<Rc<Mutex<Box<dyn FileSystem>>>> {
        // TODO: prefix tree would make this way more efficient
        self.mounts
            .lock()
            .iter()
            .find(|&(p, _)| path.starts_with(p))
            .map(|(_, m)| m.file_system.clone())
    }
}

impl FileSystem for Vfs {
    fn fsid(&self) -> FsId {
        self.fsid
    }

    fn is_read_only(&self) -> bool {
        false
    }

    fn initialize(&self) -> bool {
        true
    }

    fn open(
        &self,
        path: &'static str,
        mode: Mode,
        flags: OpenFlags,
    ) -> Result<Box<dyn FileDescriptor>, Errno> {
        match self.find_file_system_for_path(path) {
            Some(fs) => fs.clone().lock().open(path, mode, flags),
            None => Err(Errno::ENOENT),
        }
    }

    fn mkdir(&self, path: &str, mode: Mode) -> Result<(), Errno> {
        match self.find_file_system_for_path(path) {
            Some(fs) => fs.clone().lock().mkdir(path, mode),
            None => Err(Errno::ENOENT),
        }
    }

    fn rmdir(&self, path: &str) -> Result<(), Errno> {
        match self.find_file_system_for_path(path) {
            Some(fs) => fs.clone().lock().rmdir(path),
            None => Err(Errno::ENOENT),
        }
    }

    fn flush(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::filesystem::vfs::test_fs::EmptyFileSystem;

    use super::*;

    #[test_case]
    fn test_mount_unmount() {
        let fs = Box::new(EmptyFileSystem::from(19.into()));

        let vfs = Vfs::new(FsId::from(0));
        assert_eq!(0, vfs.mount_count());

        assert!(vfs.mount("/", fs, MountFlags::NONE).is_ok());
        assert_eq!(1, vfs.mount_count());

        assert!(vfs.unmount("/").is_ok());
        assert_eq!(0, vfs.mount_count());

        assert_eq!(Err(Errno::EINVAL), vfs.unmount("/"));
        assert_eq!(0, vfs.mount_count());
    }

    #[test_case]
    fn test_unmount_wrong_dir() {
        let fs = Box::new(EmptyFileSystem::from(19.into()));

        let vfs = Vfs::new(FsId::from(0));
        assert_eq!(0, vfs.mount_count());

        assert!(vfs.mount("/", fs, MountFlags::NONE).is_ok());
        assert_eq!(1, vfs.mount_count());

        assert_eq!(Err(Errno::EINVAL), vfs.unmount("/foobar"));
        assert_eq!(1, vfs.mount_count());

        assert!(vfs.unmount("/").is_ok());
        assert_eq!(0, vfs.mount_count());

        assert_eq!(Err(Errno::EINVAL), vfs.unmount("/foobar"));
        assert_eq!(0, vfs.mount_count());
    }

    #[test_case]
    fn test_mount_and_open() {
        let fs = Box::new(EmptyFileSystem::from(2.into()));

        let vfs = Vfs::new(FsId::from(11));

        assert!(vfs.mount("/", fs, MountFlags::NONE).is_ok());

        let f_res = vfs.open("/foo/bar.file", Mode::from(0), OpenFlags::O_RDWR);
        assert!(f_res.is_ok());

        let f = f_res.unwrap();
        assert_eq!("/foo/bar.file", f.absolute_path());
    }
}
