// use crate::filesystem::fd::FileDescriptor;
// use crate::filesystem::flags::{Mode, OpenFlags};
// use crate::filesystem::{FileSystem, FsId};
// use crate::syscall::error::Errno;
// use alloc::sync::Arc;
//
// pub struct Ext2Fs {}
//
// impl FileSystem for Ext2Fs {
//     fn fsid(&self) -> FsId {
//         todo!()
//     }
//
//     fn is_read_only(&self) -> bool {
//         todo!()
//     }
//
//     fn open(
//         &mut self,
//         _path: &str,
//         _mode: Mode,
//         _flags: OpenFlags,
//     ) -> Result<&Arc<FileDescriptor>, Errno> {
//         todo!()
//     }
//
//     fn mkdir(&mut self, _path: &str, _mode: Mode) -> Result<(), Errno> {
//         todo!()
//     }
//
//     fn rmdir(&mut self, _path: &str) -> Result<(), Errno> {
//         todo!()
//     }
//
//     fn flush(&mut self) {
//         todo!()
//     }
// }
