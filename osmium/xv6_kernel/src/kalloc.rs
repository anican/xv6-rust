//! Rust implementation of kernel/kalloc.c.
//!
//! Physical memory allocator, for user processes, kernel stacks, page-table pages, and pipe
//! buffers. Allocates whole 4096-byte pages.

use crate::c_defs::*;
use crate::memlayout;
use crate::riscv;
use core::ffi::c_void;
use core::mem::MaybeUninit;
use xv6_defs::c_structs::*;
use xv6_defs::c_types::*;

extern "C" {
    // First address after the kernel. Defined by kernel.ld.
    static mut end: c_char;
}

struct Run {
    next: *mut Run,
}

struct KMem {
    lock: SpinLock,
    freelist: *mut Run,
    nfree: u64,
}

static mut KMEM: MaybeUninit<KMem> = MaybeUninit::uninit();

#[no_mangle]
pub unsafe extern "C" fn kinit() {
    initlock(&mut KMEM.assume_init_mut().lock, "kmem\0".as_ptr());
    freerange(
        &mut end as *mut u8 as *mut c_void,
        memlayout::PHYSTOP as *mut c_void,
    );
}

unsafe fn freerange(pa_start: *mut c_void, pa_end: *mut c_void) {
    let mut p = riscv::pgroundup(pa_start as u64);
    while p + riscv::PGSIZE <= pa_end as u64 {
        kfree(p as *mut c_void);
        p += riscv::PGSIZE;
    }
}

/// Free the page of physical memory pointed at by pa, which normally should have been returned by a
/// call to kalloc().  (The exception is when initializing the allocator; see kinit above.)
#[no_mangle]
pub unsafe extern "C" fn kfree(pa: *mut c_void) {
    if (pa as u64) % riscv::PGSIZE != 0
        || pa < end as *mut c_void
        || pa as u64 >= memlayout::PHYSTOP
    {
        panic("kfree".as_ptr());
    }

    // Fill with junk to catch dangling refs.
    memset(pa, 1, riscv::PGSIZE as u32);

    let r = pa as *mut Run;

    let kmem = KMEM.assume_init_mut();
    acquire(&mut kmem.lock);
    (*r).next = kmem.freelist;
    kmem.freelist = r;
    kmem.nfree += 1;
    release(&mut kmem.lock);
}

/// Allocate one 4096-byte page of physical memory. Returns a pointer that the kernel can use.
/// Returns 0 if the memory cannot be allocated.
#[no_mangle]
pub unsafe extern "C" fn kalloc() -> *mut c_void {
    let kmem = KMEM.assume_init_mut();
    acquire(&mut kmem.lock);
    let r = kmem.freelist;
    if !r.is_null() {
        kmem.freelist = (*r).next;
        kmem.nfree -= 1;
    }
    release(&mut kmem.lock);

    if !r.is_null() {
        // fill with junk
        memset(r as *mut c_void, 5, riscv::PGSIZE as u32);
    }

    r as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn sys_nfree() -> u64 {
    KMEM.assume_init_ref().nfree
}
