//! Rust struct definitions for xv6 C structs. These structs can be used with the C version in FFI calls.

use crate::c_types::*;
use crate::param;
use core::ffi::c_void;

pub type PagetableT = *mut u64;

// Saved registers for kernel context switches.
#[repr(C)]
pub struct SpinLock {
    locked: c_uint,
    name: *const c_char,
    // Placeholder pointer type.
    cpu: *mut c_void,
    n: c_uint,
    nts: c_uint,
}

#[repr(C)]
pub struct Context {
    pub ra: u64,
    pub sp: u64,

    // callee-saved
    pub s0: u64,
    pub s1: u64,
    pub s2: u64,
    pub s3: u64,
    pub s4: u64,
    pub s5: u64,
    pub s6: u64,
    pub s7: u64,
    pub s8: u64,
    pub s9: u64,
    pub s10: u64,
    pub s11: u64,
}

#[repr(C)]
pub enum ProcState {
    UNUSED,
    SLEEPING,
    RUNNABLE,
    RUNNING,
    ZOMBIE,
}

#[repr(C)]
pub struct Proc {
    pub lock: SpinLock,
    pub state: ProcState,
    pub parent: *mut Proc,
    pub chan: *mut c_void,
    pub killed: c_int,
    pub xstate: c_int,
    pub pid: c_int,

    pub kstack: u64,
    pub sz: u64,
    pub pagetable: PagetableT,
    // The following contain placeholder pointer types.
    pub trapframe: *mut c_void,
    pub context: Context,
    pub ofile: [*mut c_void; param::NOFILE],
    pub cwd: *mut c_void,
    pub name: [c_char; 16],
}
