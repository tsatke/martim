use crate::filesystem::flags::Mode;

pub struct Stat {
    pub device_id: usize,
    pub inode_number: usize,
    pub access_mode: Mode,
    pub num_hard_links: usize,
    pub owner_uid: usize,
    pub owner_gid: usize,
    pub special: bool,
    pub size: usize,
    pub block_size: usize,
    pub block_count: usize,
}
