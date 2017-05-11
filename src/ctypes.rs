#![allow(non_camel_case_types)]

pub type c_char = c_schar;
pub type c_int = i32;
pub type c_long = i32;
pub type c_longlong = i64;
pub type c_schar = i8;
pub type c_short = i16;
pub type c_uchar = u8;
pub type c_uint = u32;
pub type c_ulong = u32;
pub type c_ulonglong = u64;
pub type c_ushort = u16;

// NOTE copied from the libc crate
// Use repr(u8) as LLVM expects `void*` to be the same as `i8*` to help enable

// more optimization opportunities around it recognizing things like
// malloc/free.
#[repr(u8)]
pub enum c_void {
    // Two dummy variants so the #[repr] attribute can be used.
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}
