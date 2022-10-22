#![no_std]

use xv6_defs::c_types::*;
use xv6_ulib::Args;
use xv6_ulib::CStr;

fn run(args: Args) -> i32 {
    let argc = args.argc();
    for (i, arg) in args.enumerate().skip(1) {
        xv6_ulib::write(1, arg.chars_as_bytes()).unwrap();
        if i + 1 < argc {
            xv6_ulib::write(1, CStr::new(" \0").as_bytes()).unwrap();
        } else {
            xv6_ulib::write(1, CStr::new("\n\0").as_bytes()).unwrap();
        }
    }
    0
}

/// Entry point.
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *const *const c_char) -> c_int {
    xv6_ulib::run_prog(argc, argv, run)
}
