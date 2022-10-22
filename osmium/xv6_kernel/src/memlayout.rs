//! Constants defined in `kernel/memlayout.h`

pub const KERNBASE: u64 = 0x80000000;
pub const PHYSTOP: u64 = KERNBASE + 128 * 1024 * 1024;
