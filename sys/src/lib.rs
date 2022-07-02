#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#[cfg(all(cortex_m4, feature = "bindgen"))]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
#[cfg(all(cortex_m4, not(feature = "bindgen")))]
include!("bindings.rs");

use core::arch::global_asm;

// #[no_mangle]
// #[repr(C, align(2))]
// pub static P256_order: [u32; 9] = [
//     0xFC632551, 0xF3B9CAC2, 0xA7179E84, 0xBCE6FAAD, 0xFFFFFFFF, 0xFFFFFFFF, 0, 0xFFFFFFFF, 0,
// ];

global_asm!(include_str!("./asm.s"));

extern "C" {
    /// Checks that the argument, as little-endian integer, is a reduced non-zero
    /// element of the scalar field.
    ///
    /// In other words, that it is in the range `1..=n-1`,
    /// where `n = 2^256 - 2^224 + 2^192 - 0x4319055258e8617b0c46353d039cdaaf`.
    pub fn P256_check_range_n(a: *const u32) -> bool;
}
