use crate::filesystem::{FileSystem, FsId};

pub struct TestFs {
    fsid: FsId,
    init_called: usize,
}

impl TestFs {
    pub fn from(fsid: FsId) -> Self {
        TestFs {
            fsid,
            init_called: 0,
        }
    }

    pub fn init_called_count(&self) -> usize { self.init_called }
}

impl FileSystem for TestFs {
    fn fsid(&self) -> FsId { self.fsid }

    fn initialize(&mut self) -> bool {
        self.init_called += 1;
        true
    }

    fn is_read_only(&self) -> bool { false }

    fn total_block_count(&self) -> usize {
        todo!()
    }

    fn free_block_count(&self) -> usize {
        todo!()
    }

    fn total_inode_count(&self) -> usize {
        todo!()
    }

    fn free_inode_count(&self) -> usize {
        todo!()
    }

    fn flush(&self) {}
}