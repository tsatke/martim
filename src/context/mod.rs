use core::sync::atomic::{AtomicU64, Ordering};

use lazy_static::lazy_static;
use spin::Mutex;

use list::ContextList;

mod list;

lazy_static! {
    static ref CONTEXTS: Mutex<ContextList> = Mutex::new(ContextList::new());
}

pub fn init() {
    // init first context
    assert!(CONTEXTS.lock().new_context().is_ok());
}

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
            status: Status::Runnable,
            running: false,
        }
    }
}