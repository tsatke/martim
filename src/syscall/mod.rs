use error::Result;

use crate::context::ContextId;
use crate::syscall::error::Errno;

pub mod error;

pub fn getpid() -> Result<ContextId> {
    Err(Errno::ENOSYS)
}