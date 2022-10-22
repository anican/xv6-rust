use xv6_defs::c_types::*;
use xv6_ulib::c_user::*;
use xv6_ulib::Args;
use xv6_ulib::CStr;

fn run(args: Args) -> i32 {
    let argc = args.argc();
    if argc < 2 {
        writeln!(xv6_ulib::stdout(), "sleep: insufficient arguments").unwrap();
        return exit(1);
    } 

    let ticks = match args.nth(1).data.parse() {
        Ok(t) => t,
        Err(_) 
    };
    sleep(time);
}

/// Entry point.
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *const *const c_char) -> c_int {
    xv6_ulib::run_prog(argc, argv, run)
}