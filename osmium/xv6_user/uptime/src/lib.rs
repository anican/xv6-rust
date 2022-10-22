#![no_std]

use core::fmt::Write;
use xv6_defs::c_types::*;
use xv6_ulib::c_user::*;

#[no_mangle]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *const *const c_char) -> c_int {
    let ticks = uptime();
    writeln!(xv6_ulib::stdout(), "{}", ticks).unwrap();
    exit(0)
}
