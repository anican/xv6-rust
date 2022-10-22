#![no_std]

use core::fmt::Write;
use xv6_ulib::c_user::*;

/// Entry point.
#[no_mangle]
pub extern "C" fn main() -> i32 {
    writeln!(xv6_ulib::stdout(), "Hello World").unwrap();
    unsafe { exit(0) }
}
