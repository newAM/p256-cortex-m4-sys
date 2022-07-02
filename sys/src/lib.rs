#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::missing_safety_doc)]
#[cfg(all(cortex_m4, feature = "bindgen"))]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
#[cfg(all(cortex_m4, not(feature = "bindgen")))]
include!("bindings.rs");

use core::arch::global_asm;
use core::ffi::c_void;
use core::slice;

// #[no_mangle]
// #[repr(C, align(2))]
// pub static P256_order: [u32; 9] = [
//     0xFC632551, 0xF3B9CAC2, 0xA7179E84, 0xBCE6FAAD, 0xFFFFFFFF, 0xFFFFFFFF, 0, 0xFFFFFFFF, 0,
// ];

global_asm!(include_str!("./asm.s"));

extern "C" {
    /// Checks that the argument, as little-endian integer,
    /// is a reduced non-zero element of the scalar field.
    ///
    /// In other words, that it is in the range `1..=n-1`,
    /// where `n = 2^256 - 2^224 + 2^192 - 0x4319055258e8617b0c46353d039cdaaf`.
    pub fn P256_check_range_n(a: *const u32) -> bool;

    /// Checks that the argument, as little-endian integer,
    /// is a reduced element of the base field.
    ///
    /// In other words, that it is in the range `0..=p-1`,
    /// where `p = 2^256 - 2^224 + 2^192 + 2^96 - 1`.
    pub fn P256_check_range_p(a: *const u32) -> bool;
}

/// Converts endianness by reversing the input value.
///
/// The output and input pointers may refer to the same location
/// and have no alignment requirements.
// TODO: is the above about same location correct?  Rust alias and all.
#[no_mangle]
pub unsafe extern "C" fn p256_convert_endianness(
    output: *mut c_void,
    input: *const c_void,
    byte_len: u32,
) {
    let len: usize = byte_len as usize;
    let out_slice: &mut [u8] = slice::from_raw_parts_mut(output as *mut u8, len);
    let in_slice: &[u8] = slice::from_raw_parts(input as *const u8, len);
    for i in 0..len / 2 {
        let t: u8 = in_slice[len - 1 - i];
        out_slice[len - 1 - i] = in_slice[i];
        out_slice[i] = t;
    }
}
