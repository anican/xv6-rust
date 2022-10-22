//! FFI bindings for xv6's C user library.

use xv6_defs::c_types::*;

extern "C" {
    pub fn exit(code: c_int) -> c_int;
    pub fn write(x: c_int, y: *const c_char, z: c_int) -> c_int;
    pub fn read(x: c_int, y: *const c_char, z: c_int) -> c_int;
    pub fn close(fd: c_int) -> c_int;
    pub fn open(file: *const c_char, flags: c_int) -> c_int;
    pub fn strlen(s: *const c_char) -> c_int;
    pub fn printf(fmt: *const c_char, args: ...);
    pub fn uptime() -> c_int;
}
