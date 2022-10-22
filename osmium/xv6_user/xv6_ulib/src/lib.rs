//! # xv6_ulib
//! Library for xv6 user programs. Contains FFI bindings for xv6's C user library and native Rust
//! functions.
//!
//! To create a new user program `<uprog_name>` in Rust,
//! 1. Run `cargo new --lib <uprog_name>` in the directory `rustcomps/xv6_uprogs`.
//! 2. Add the new crate to the `rustcomps` workspace by editing `rustcomps/Cargo.toml`.
//! 3. In the new crate, copy the `Cargo.toml` file from `echo` and change the name to the new
//!    executable. This sets up the dependencies.
//! 4. In `rustcomps/xv6_uprogs/<uprog_name>/src/lib.rs`, optionally add the following code to the
//!    top of the file to bring useful types and functions into scope.
//! ```
//! use xv6_defs::c_types::*;
//! use xv6_ulib::c_user::*;
//! ```
//! 5. Copy the main function definition from an existing user program (like `echo`) and replace it
//!    with your code.
//! 6. Add the new user program to the `RUST_UPROGS` variable in `Makefile` and remove it from the
//!    `UPROGS` variable.

#![no_std]

extern crate panic_halt;

pub mod c_user;

use core::fmt;
use core::fmt::Write;
use core::slice;
use core::str;
use xv6_defs::c_types::*;

/// Wrapper around a `str` that is guaranteed to represent a valid C string.
pub struct CStr<'a> {
    pub data: &'a str,
}

impl<'a> CStr<'a> {
    /// Constructs a new `CStr` and checks its validity. Verifies it contains only ASCII characters
    /// and it has exactly one null terminator as the last bytes. Exits the program if an error is
    /// detected.
    pub fn new(data: &'a str) -> CStr<'a> {
        let len = data.len();
        if data.is_empty()
            || !data.is_ascii()
            || !data.ends_with('\0')
            || data[0..len - 1].contains('\0')
        {
            exit(1);
        }
        CStr { data }
    }

    /// Returns the length of the string not including the null terminator.
    pub fn len(&self) -> usize {
        self.data.len() - 1
    }

    /// Returns whether the string is empty or not
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a slice representing the data in the string including the null terminator.
    pub fn as_bytes(&self) -> &[u8] {
        self.data.as_bytes()
    }

    /// Returns a slice representing the data in the string excluding the null terminator.
    pub fn chars_as_bytes(&self) -> &[u8] {
        &self.data.as_bytes()[0..self.len()]
    }

    pub fn as_str(&self) -> &str {
        self.data
    }
}

/// Terminates the program with exit code `code`.
pub fn exit(code: i32) -> ! {
    unsafe {
        c_user::exit(code);
    }
    loop {}
}

/// Writes the data in `buf` to the file descriptor `fildes`. Returns the number of bytes written.
///
/// # Errors
/// Returns an error if the system call failed.
pub fn write(fildes: i32, buf: &[u8]) -> Result<usize, ()> {
    let result;
    unsafe {
        result = c_user::write(fildes, buf.as_ptr(), buf.len() as c_int);
    }
    if result >= 0 {
        Ok(result as usize)
    } else {
        Err(())
    }
}

/// Reads the data in `buf` from the file descriptor `fildes`. Returns the number of bytes read.
///
/// # Errors
/// Returns an error if the system call failed.
pub fn read(fildes: i32, buf: &[u8]) -> Result<usize, ()> {
    let result;
    unsafe {
        result = c_user::read(fildes, buf.as_ptr(), buf.len() as c_int);
    }
    if result >= 0 {
        Ok(result as usize)
    } else {
        Err(())
    }
}

/// Release open file descriptor `fildes`. Returns true if successful, else false.
pub fn close(fildes: i32) -> Result<(), ()> {
    let result: i32;
    unsafe {
        result = c_user::close(fildes);
    }
    if result >= 0 {
        Ok(())
    } else {
        Err(())
    }
}

/// Opens `file` with corresponding read/write `flags`. Returns a file descriptor.
///
/// # Errors
/// Returns an error if the system call failed.
pub fn open(file: &[u8], flags: i32) -> Result<i32, ()> {
    let result: i32;
    unsafe {
        result = c_user::open(file.as_ptr(), flags);
    }
    if result >= 0 {
        Ok(result)
    } else {
        Err(())
    }
}

/// Iterator over command line arguments.
pub struct Args {
    current: usize,
    num_args: usize,
    args: *const *const c_char,
}

impl Args {
    /// Returns an iterator over the given arguments.
    ///
    /// # Safety
    /// The returned value must not live longer than `argc` and `argv`.
    unsafe fn new(argc: c_int, argv: *const *const c_char) -> Self {
        Args {
            current: 0,
            num_args: argc as usize,
            // num_args: argc as usize,
            args: argv,
        }
    }

    /// Returns the number of arguments, including the first one representing the program name.
    pub fn argc(&self) -> usize {
        self.num_args
    }
}

impl Iterator for Args {
    type Item = CStr<'static>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.num_args {
            None
        } else {
            let as_str;
            unsafe {
                let arg = *(self.args.add(self.current));
                let len = c_user::strlen(arg);
                let bytes = slice::from_raw_parts(arg, (len + 1) as usize);
                as_str = str::from_utf8(bytes).unwrap();
            }
            self.current += 1;
            Some(CStr::new(as_str))
        }
    }
}

/// Given a safe main function `main`, executes the function passing in the arguments represented
/// by `argc` and `argv` wrapped in a safe `Args` type. This function calls `exit` with the exit
/// status that the main function returns, so there's no need to call it at the end of the main
/// function.
///
/// # Safety
/// The `argv` pointer must refer to a valid array of C strings of length `argc`.
pub unsafe fn run_prog(
    argc: c_int,
    argv: *const *const c_char,
    main: impl FnOnce(Args) -> i32,
) -> c_int {
    exit(main(Args::new(argc, argv)));
}

/// Dummy structure representing a `core::fmt::Write` implementation that can write to stdout.
struct StdOut;

impl Write for StdOut {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();
        let len = bytes.len();
        let mut num_written = 0;
        while num_written < len {
            match write(1, &bytes[num_written..len]) {
                Ok(n) => num_written += n,
                Err(_) => return Err(core::fmt::Error),
            }
        }
        Ok(())
    }
}

/// Returns a structure suitable for writing to stdout with the `write!` macro.
pub fn stdout() -> impl Write {
    StdOut
}
