use crate::filesystem::FileSystem;
use alloc::boxed::Box;
use alloc::rc::Rc;
use spin::Mutex;

pub struct Mount {
    pub path: &'static str,
    pub file_system: Rc<Mutex<Box<dyn FileSystem>>>,
}

impl Mount {
    pub fn new(path: &'static str, file_system: Box<dyn FileSystem>) -> Self {
        Mount {
            path,
            file_system: Rc::new(Mutex::new(file_system)),
        }
    }
}
