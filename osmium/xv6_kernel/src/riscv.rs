//! Constants and macros from `kernel/riscv.h`

pub const PGSIZE: u64 = 4096;

pub fn pgroundup(sz: u64) -> u64 {
    (sz + PGSIZE - 1) & !(PGSIZE - 1)
}
