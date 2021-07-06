use core::sync::atomic::Ordering;

use error::Result;

use crate::context::{ContextId, CURRENT_CONTEXT_ID};

pub mod error;

pub fn getpid() -> Result<ContextId> {
    Ok(ContextId::from(CURRENT_CONTEXT_ID.load(Ordering::SeqCst)))
}