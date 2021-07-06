use core::sync::atomic::{AtomicU64, Ordering};

use crate::context::context::Status::Runnable;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContextId(u64);

impl ContextId {
    pub(crate) fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        ContextId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Status {
    Runnable,
    Blocked,
    Stopped(usize),
    Exited(usize),
}

pub struct Context {
    pub id: ContextId,
    pub status: Status,
    pub running: bool,
}

impl Context {
    pub(crate) fn new(id: ContextId) -> Self {
        Context {
            id,
            status: Runnable,
            running: false,
        }
    }
}