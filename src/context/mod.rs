use lazy_static::lazy_static;
use spin::Mutex;

use list::ContextList;

mod context;
mod list;

lazy_static! {
    static ref CONTEXTS: Mutex<ContextList> = Mutex::new(ContextList::new());
}

pub fn init() {
    // init first context
    assert!(CONTEXTS.lock().new_context().is_ok());
}
