use core::sync::atomic::{AtomicU64, Ordering};

use lazy_static::lazy_static;
use spin::Mutex;

use list::ContextList;

mod list;

lazy_static! {
    static ref CONTEXTS: Mutex<ContextList> = Mutex::new(ContextList::new());
}

lazy_static! {
    #[thread_local]
    pub static ref CURRENT_CONTEXT_ID: AtomicU64 = AtomicU64::default();
}

pub fn init() {
    // init first context
    let id = CONTEXTS
        .lock()
        .new_context()
        .expect("unable to initialize first context")
        .lock()
        .id;
    CURRENT_CONTEXT_ID.store(id.0, Ordering::SeqCst);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContextId(u64);

impl ContextId {
    pub(crate) fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        ContextId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }

    pub(crate) fn from(id: u64) -> Self {
        ContextId(id)
    }
}

impl From<ContextId> for u64 {
    fn from(c: ContextId) -> Self {
        c.0
    }
}

impl From<ContextId> for usize {
    fn from(c: ContextId) -> Self {
        c.0 as usize
    }
}

impl From<ContextId> for i32 {
    fn from(c: ContextId) -> Self {
        c.0 as i32
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
