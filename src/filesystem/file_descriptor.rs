use crate::filesystem::stat::Stat;
use crate::syscall::error::Errno;
use alloc::vec::Vec;

pub enum Seek {
    Set,
    Cur,
    End,
}

pub trait FileDescriptor {
    fn is_readable(&self) -> bool;
    fn is_writable(&self) -> bool;

    fn seek(&self, _offset: isize, _whence: Seek) -> Result<usize, Errno>;

    fn read(&mut self, _buffer: Vec<u8>) -> Result<usize, Errno>;
    fn write(&mut self, _buffer: Vec<u8>) -> Result<usize, Errno>;
    fn stat(&self) -> Result<Stat, Errno>;

    fn absolute_path(&self) -> &str;
}
