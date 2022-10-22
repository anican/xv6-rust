use crate::c_defs::*;
use core::ffi::c_void;
use xv6_defs::c_structs::*;
use xv6_defs::c_types::*;

/// Returns the current process's PID.
#[no_mangle]
pub unsafe extern "C" fn sys_exit() -> i32 {
    let mut n: i32 = 0;
    if argint(0, &mut n) < 0 {
        -1
    } else {
        exit(n);
        0
    }
}

/// Returns the current process's PID.
#[no_mangle]
pub unsafe extern "C" fn sys_getpid() -> u64 {
    (*myproc()).pid as u64
}

/// Creates a process, returns child's PID.
#[no_mangle]
pub unsafe extern "C" fn sys_fork() -> u64 {
    fork() as u64
}

/// Waits for a child process to exit, returns child's PID.  Return -1 if this process has no
/// children.
#[no_mangle]
pub unsafe extern "C" fn sys_wait() -> i32 {
    let mut p: u64 = 0;
    if argaddr(0, &mut p) < 0 {
        -1
    } else {
        wait(p)
    }
}

/// Grow process' memory. Returns start of new memory, or -1 if error.
#[no_mangle]
pub unsafe extern "C" fn sys_sbrk() -> i32 {
    let mut n: i32 = 0;
    if argint(0, &mut n) < 0 {
        return -1;
    }
    let addr = (*myproc()).sz;
    if growproc(n) < 0 {
        -1
    } else {
        addr as i32
    }
}

/// Terminates process. Returns 0, or -1 if error.
#[no_mangle]
pub unsafe extern "C" fn sys_kill() -> i32 {
    let mut pid = 0;
    if argint(0, &mut pid) < 0 {
        -1
    } else {
        kill(pid)
    }
}

/// Pause for specified number of clock ticks. Returns 0, -1 if error.
#[no_mangle]
pub unsafe extern "C" fn sys_sleep() -> i32 {
    let mut n: i32 = 0;
    let ticks0: c_uint;

    if argint(0, &mut n) < 0 {
        return -1;
    }
    acquire(&mut tickslock);
    ticks0 = ticks;
    while ticks - ticks0 < n as u32 {
        if (*myproc()).killed != 0 {
            release(&mut tickslock);
            return -1;
        }
        sleep(&mut ticks as *mut u32 as *mut c_void, &mut tickslock);
    }
    release(&mut tickslock);
    0
}

/// Returns how many clock tick interrupts have occurred since start.
#[no_mangle]
pub unsafe extern "C" fn sys_uptime() -> u64 {
    let xticks;
    acquire(&mut tickslock as *mut SpinLock);
    xticks = ticks;
    release(&mut tickslock as *mut SpinLock);
    xticks as u64
}
