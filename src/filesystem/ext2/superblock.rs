#[repr(u16)]
pub enum FileSystemState {
    Clean = 1,
    Error = 2,
}

#[repr(u16)]
pub enum ErrorStrategy {
    IgnoreAndContinue = 1,
    RemountReadOnly = 2,
    KernelPanic = 3,
}

#[repr(C)]
pub struct SuperBlock {
    total_inode_count: u32,
    total_block_count: u32,
    reserved_blocks: u32,
    total_free_blocks: u32,
    total_free_inodes: u32,
    superblock_block_id: u32,
    block_size: u32,
    fragment_size: u32,
    blocks_per_group: u32,
    fragments_per_group: u32,
    inodes_per_group: u32,
    last_mount_time: u32,
    last_write_time: u32,
    mount_count_since_fsck: u16,
    fsck_threshold: u16,
    /// 0xef53 to validate that this is indeed an ext2 fs.
    magic_number: u16,
    state: FileSystemState,
    error_strategy: ErrorStrategy,
    minor: u16,
    last_fsck_time: u32,
    fsck_force_interval: u32,
    os_id: u32,
    major: u32,
    user_id_reserve_blocks: u16,
    group_id_reserve_blocks: u16,
}
