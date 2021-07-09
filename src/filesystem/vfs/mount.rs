use crate::filesystem::FileSystem;
use crate::filesystem::vfs::flags::MountFlags;

pub struct Mount<'a> {
    file_system: &'a dyn FileSystem,
    flags: MountFlags,
}

impl<'a> Mount<'a> {
    pub fn new(file_system: &'a dyn FileSystem, flags: MountFlags) -> Self {
        Mount {
            file_system,
            flags,
        }
    }
}