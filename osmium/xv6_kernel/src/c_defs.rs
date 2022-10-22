//! FFI declarations for functions declared in kernel/defs.h.

#![allow(dead_code)]

use core::ffi::c_void;
use xv6_defs::c_structs::*;
use xv6_defs::c_types::*;

extern "C" {
    pub fn exit(code: c_int);
    pub fn write(x: c_int, y: *const c_void, z: c_int) -> c_int;
    pub fn strlen(s: *const c_char) -> c_int;
    pub fn printf(fmt: *const c_char, args: ...);

    // kalloc.c
    pub fn kalloc() -> *mut c_void;
    pub fn kfree(pa: *mut c_void);
    pub fn kinit();

    // printf.c
    pub fn panic(s: *const c_char);

    // proc.c
    pub fn growproc(n: c_int) -> c_int;
    pub fn myproc() -> *mut Proc;
    pub fn fork() -> c_int;
    pub fn kill(pid: c_int) -> c_int;
    pub fn sleep(chan: *const c_void, lk: *mut SpinLock);
    pub fn wait(addr: u64) -> c_int;

    // spinlock.c
    pub fn acquire(lk: *mut SpinLock);
    pub fn release(lk: *mut SpinLock);
    pub fn initlock(lk: *mut SpinLock, name: *const c_char);

    // string.c
    pub fn memset(dst: *mut c_void, c: c_int, n: c_uint);

    // syscall.c
    pub fn argint(n: c_int, ip: *mut c_int) -> c_int;
    pub fn argaddr(n: c_int, ip: *mut u64) -> c_int;

    // trap.c
    pub static mut ticks: c_uint;
    pub static mut tickslock: SpinLock;
}
