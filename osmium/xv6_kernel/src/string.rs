//! Rust implementation of kernel/string.c.

use xv6_defs::c_types::*;

// Note, this doesn't define every function that was previously defined in string.c in xv6. This is
// because functions such as memcpy are builtin by LLVM.

#[no_mangle]
pub unsafe extern "C" fn strncmp(p: *const c_char, q: *const c_char, n: c_uint) -> c_int {
    let mut p1 = p;
    let mut p2 = q;
    let mut i = 0;
    while i < n && *p1 != 0 && *p1 == *p2 {
        p1 = p1.add(1);
        p2 = p2.add(1);
        i += 1;
    }
    if i == n {
        return 0;
    }
    *p1 as c_int - *p2 as c_int
}

#[no_mangle]
pub unsafe extern "C" fn strncpy(s: *mut c_char, t: *const c_char, n: c_int) -> *mut c_char {
    let mut i = 0;
    while i < n {
        let b = *t.add(i as usize);
        *s.add(i as usize) = b;
        i += 1;
        if b == 0 {
            break;
        }
    }
    while i < n {
        *s.add(i as usize) = 0;
        i += 1;
    }
    s
}

#[no_mangle]
pub unsafe extern "C" fn safestrcpy(s: *mut c_char, t: *const c_char, n: c_int) -> *mut c_char {
    let mut i = 0;
    while i < n {
        let b = *t.add(i as usize);
        *s.add(i as usize) = b;
        i += 1;
        if b == 0 {
            break;
        }
    }
    *s.add(i as usize) = 0;
    s
}

#[no_mangle]
pub unsafe extern "C" fn strlen(s: *const c_char) -> c_int {
    let mut len: c_int = 0;
    let mut c = *s.add(len as usize);
    while c != 0 {
        len += 1;
        c = *s.add(len as usize);
    }
    len
}
