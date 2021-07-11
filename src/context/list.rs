use alloc::collections::BTreeMap;
use alloc::sync::Arc;

use spin::Mutex;

use super::{Context, ContextId};

pub struct ContextList {
    map: BTreeMap<ContextId, Arc<Mutex<Context>>>,
}

impl ContextList {
    pub fn new() -> Self {
        ContextList {
            map: BTreeMap::new(),
        }
    }

    pub fn new_context(&mut self) -> Result<&Arc<Mutex<Context>>, ()> {
        let id = ContextId::new();
        assert!(!self.map.contains_key(&id));
        assert!(self
            .map
            .insert(id, Arc::new(Mutex::new(Context::new(id))))
            .is_none());

        Ok(self.map.get(&id).expect("unable to insert context"))
    }
}
