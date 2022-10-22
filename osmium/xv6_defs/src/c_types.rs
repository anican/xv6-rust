//! Rust type definitions for common C types for interfacing with C code.

// Using non-camel-cased types because they look nicer and that's also what the libc crate uses.
#![allow(non_camel_case_types)]

// Rust types for builtin C types.
pub type c_int = i32;
// The C standard allows the char type to be signed or unsigned. Most compilers use signed chars,
// but defining the c_int type as u8 shouldn't cause issues because this behavior isn't relied
// upon. The char type is almost always used as storage for ASCII, which only goes up to 127, or
// as raw bytes to be copied. In both cases using unsigned characters instead is acceptable. The u8
// type is used here for convenience with interfacing with Rust. Many Rust methods that deal with
// raw data use u8, not i8, so this definition reduces the number of type casts.
pub type c_char = u8;

// Aliases for xv6 specific types. For types such as uint64, the obvious Rust equivalents like u64
// should be used instead.
pub type c_uint = u32;
pub type c_ushort = u16;
pub type c_uchar = u8;
