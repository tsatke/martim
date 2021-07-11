use crate::filesystem::fd::FileDescriptor;
use crate::filesystem::flags::{Mode, OpenFlags};
use crate::filesystem::{FileSystem, FsId};
use crate::syscall::error::Errno;
use alloc::collections::LinkedList;
use alloc::sync::Arc;
use core::borrow::BorrowMut;

pub struct TestFs {
    fsid: FsId,
    init_callback: Option<fn()>,
    open_callback: Option<fn(&str, Mode, OpenFlags)>,
}

impl TestFs {
    pub fn from(fsid: FsId) -> Self {
        TestFs {
            fsid,
            init_callback: None,
            open_callback: None,
        }
    }

    pub fn set_initialize_callback(&mut self, cb: fn()) {
        self.init_callback = Some(cb);
    }
}

impl FileSystem for TestFs {
    fn fsid(&self) -> FsId {
        self.fsid
    }

    fn initialize(&self) -> bool {
        true
    }

    fn is_read_only(&self) -> bool {
        false
    }

    fn open(&self, path: &str, mode: Mode, flags: OpenFlags) -> Result<FileDescriptor, Errno> {
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
