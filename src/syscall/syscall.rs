use core::sync::atomic::Ordering;

use crate::context::{ContextId, CURRENT_CONTEXT_ID};
use crate::syscall::error::Result;

pub fn getpid() -> Result<ContextId> {
    Ok(ContextId::from(CURRENT_CONTEXT_ID.load(Ordering::SeqCst)))
}