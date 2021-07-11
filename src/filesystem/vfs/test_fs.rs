// use crate::filesystem::fd::FileDescriptor;
// use crate::filesystem::flags::{Mode, OpenFlags};
// use crate::filesystem::{FileSystem, FsId};
// use crate::syscall::error::Errno;
// use alloc::collections::LinkedList;
// use alloc::sync::Arc;
// use core::borrow::BorrowMut;
//
// pub struct TestFs<'a> {
//     fsid: FsId,
//     init_called: usize,
//     open_calls: LinkedList<(&'a str, Mode, OpenFlags)>,
// }
//
// impl<'a> TestFs<'a> {
//     pub fn from(fsid: FsId) -> Self {
//         TestFs {
//             fsid,
//             init_called: 0,
//             open_calls: LinkedList::new(),
//         }
//     }
//
//     pub fn init_called_count(&self) -> usize {
//         self.init_called
//     }
// }
//
// impl<'a> FileSystem for TestFs<'a> {
//     fn fsid(&self) -> FsId {
//         self.fsid
//     }
//
//     fn initialize(&mut self) -> bool {
//         self.init_called += 1;
//         true
//     }
//
//     fn is_read_only(&self) -> bool {
//         false
//     }
//
//     fn open(
//         &mut self,
//         path: &'a str,
//         mode: Mode,
//         flags: OpenFlags,
//     ) -> Result<&Arc<FileDescriptor>, Errno> {
//         self.open_calls.push_back((path, mode, flags));
//         todo!()
//     }
//
//     fn mkdir(&mut self, path: &str, mode: Mode) -> Result<(), Errno> {
//         todo!()
//     }
//
//     fn rmdir(&mut self, path: &str) -> Result<(), Errno> {
//         todo!()
//     }
//
//     fn flush(&mut self) {}
// }
