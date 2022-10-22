#![no_std]

use core::fmt::Write;
use xv6_defs::c_types::*;
use xv6_ulib::Args;

fn cat(fd: i32, buf: &[u8]) {
    loop {
        let n = match xv6_ulib::read(fd, buf) {
            Ok(0) => return,
            Ok(n) => n,
            Err(_) => {
                writeln!(xv6_ulib::stdout(), "cat: read error").unwrap();
                return;
            }
        };

        if xv6_ulib::write(1, &buf[0..n]) != Ok(n) {
            writeln!(xv6_ulib::stdout(), "cat: write error").unwrap();
            return;
        }
    }
}

fn run(args: Args) -> i32 {
    let argc = args.argc();
    let buf: [u8; 512] = [0; 512];
    if argc <= 1 {
        cat(0, &buf);
        return 1;
    }

    for arg in args.skip(1) {
        let fd = match xv6_ulib::open(arg.as_bytes(), 0) {
            Ok(fd) => fd,
            Err(_) => {
                writeln!(xv6_ulib::stdout(), "cat: cannot open {}", arg.as_str()).unwrap();
                return 1;
            }
        };
        cat(fd, &buf);
        xv6_ulib::close(fd).unwrap();
    }
    0
}

/// Entry point.
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *const *const c_char) -> c_int {
    xv6_ulib::run_prog(argc, argv, run)
}
