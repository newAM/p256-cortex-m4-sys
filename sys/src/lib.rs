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

global_asm!(include_str!("./asm.s"));

extern "C" {
    // int P256_divsteps2_31(int delta, uint32_t f, uint32_t g, uint32_t res_matrix[4]);
    // void P256_matrix_mul_fg_9(uint32_t a, uint32_t b, const struct FGInteger fg[2], struct FGInteger *res);
    // void P256_matrix_mul_mod_n(uint32_t a, uint32_t b, const struct XYInteger xy[2], struct XYInteger *res);

    // void P256_to_montgomery(uint32_t aR[8], const uint32_t a[8]);
    fn P256_to_montgomery(aR: *mut u32, a: *const u32);
    // void P256_from_montgomery(uint32_t a[8], const uint32_t aR[8]);
    // bool P256_check_range_p(const uint32_t a[8]);

    // bool P256_check_range_n(const uint32_t a[8]);
    // void P256_mul_mod_n(uint32_t res[8], const uint32_t a[8], const uint32_t b[8]);
    // void P256_add_mod_n(uint32_t res[8], const uint32_t a[8], const uint32_t b[8]);
    // void P256_mod_n_inv_vartime(uint32_t res[8], const uint32_t a[8]);
    // void P256_reduce_mod_n_32bytes(uint32_t res[8], const uint32_t a[8]);

    // void P256_select_point(uint32_t (*output)[8], uint32_t* table, uint32_t num_coordinates, uint32_t index);

    // void P256_jacobian_to_affine(uint32_t affine_mont_x[8], uint32_t affine_mont_y[8], const uint32_t jacobian_mont[3][8]);
    // bool P256_point_is_on_curve(const uint32_t x_mont[8], const uint32_t y_mont[8]);
    fn P256_point_is_on_curve(x_mont: *const u32, y_mont: *const u32) -> bool;
    // bool P256_decompress_point(uint32_t y[8], const uint32_t x[8], uint32_t y_parity);
    fn P256_decompress_point(y: *mut u32, x: *const u32, y_parity: u32) -> bool;

    // void P256_double_j(uint32_t jacobian_point_out[3][8], const uint32_t jacobian_point_in[3][8]);
    // void P256_add_sub_j(uint32_t jacobian_point1[3][8], const uint32_t (*point2)[8], bool is_sub, bool p2_is_affine);
    // bool P256_verify_last_step(const uint32_t r[8], const uint32_t jacobian_point[3][8]);

    // void P256_negate_mod_p_if(uint32_t out[8], const uint32_t in[8], uint32_t should_negate);
    // void P256_negate_mod_n_if(uint32_t out[8], const uint32_t in[8], uint32_t should_negate);
}

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
/// The output and input pointers may NOT refer to the same location
/// and have no alignment requirements.
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

/// Uncompressed encoding
///
/// `04 || Px || Py`.
#[no_mangle]
pub unsafe extern "C" fn p256_point_to_octet_string_uncompressed(
    out: *mut u8,
    x: *const u32,
    y: *const u32,
) {
    // uint8_t out[65], const uint32_t x[8], const uint32_t y[8]
    let out_slice: &mut [u8] = slice::from_raw_parts_mut(out, 65);
    out_slice[0] = 4;
    p256_convert_endianness(out.offset(1) as *mut _, x as *const _, 32);
    p256_convert_endianness(out.offset(33) as *mut _, y as *const _, 32);
}
/// Compressed encoding
///
/// `02 || Px` if Py is even and `03 || Px` if Py is odd.
#[no_mangle]
pub unsafe extern "C" fn p256_point_to_octet_string_compressed(
    out: *mut u8,
    x: *const u32,
    y: *const u32,
) {
    // uint8_t out[33], const uint32_t x[8], const uint32_t y[8]
    let out_slice: &mut [u8] = slice::from_raw_parts_mut(out, 33);
    let y_slice: &[u32] = slice::from_raw_parts(y, 8);
    out_slice[0] = (2 + (y_slice[0] & 1)) as u8;
    p256_convert_endianness(out.offset(1) as *mut _, x as *const _, 32);
}

/// Hybrid encoding
///
/// `06 || Px || Py` if Py is even and `07 || Px || Py` if Py is odd
/// (a pretty useless encoding).
#[no_mangle]
pub unsafe extern "C" fn p256_point_to_octet_string_hybrid(
    out: *mut u8,
    x: *const u32,
    y: *const u32,
) {
    // uint8_t out[65], const uint32_t x[8], const uint32_t y[8]
    let out_slice: &mut [u8] = slice::from_raw_parts_mut(out, 65);
    let y_slice: &[u32] = slice::from_raw_parts(y, 8);
    out_slice[0] = (6 + (y_slice[0] & 1)) as u8;
    p256_convert_endianness(out.offset(1) as *mut _, x as *const _, 32);
    p256_convert_endianness(out.offset(33) as *mut _, y as *const _, 32);
}

/// Decodes a point according to the three encodings above.
///
/// include_p256_decode_point: first byte is "04", "06" or "07" and input length is 65 bytes
/// include_p256_decompress_point: first byte is "02" or "03" and input length is 33 bytes
///
/// Returns true if the input string confirms to a valid encoding and the point lies on the curve,
/// otherwise false.
///
/// NOTE: The return value MUST be checked in case the point is not guaranteed to lie on the curve (e.g. if it
/// is received from an untrusted party).
#[no_mangle]
pub unsafe extern "C" fn p256_octet_string_to_point(
    x: *mut u32,
    y: *mut u32,
    input: *const u8,
    input_len_in_bytes: u32,
) -> bool {
    // uint32_t x[8], uint32_t y[8], const uint8_t* input, uint32_t input_len_in_bytes
    if input_len_in_bytes < 33 {
        return false;
    }
    p256_convert_endianness(x as *mut _, input.offset(1) as *const _, 32);
    if !P256_check_range_p(x) {
        return false;
    }

    let in_slice: &[u8] = slice::from_raw_parts(input, input_len_in_bytes as usize);

    if (in_slice[0] == 4 || ((in_slice[0] >> 1) == 3)) && input_len_in_bytes == 65 {
        p256_convert_endianness(y as *mut _, input.offset(33) as *const _, 32);
        if !P256_check_range_p(y) {
            return false;
        }

        let y_slice: &[u32] = slice::from_raw_parts(y as *const u32, 8);
        // TODO: how
        if (in_slice[0] >> 1) == 3 && u32::from(in_slice[0] & 1) != (y_slice[0] & 1) {
            return false;
        }
        let mut x_mont: [u32; 8] = [0; 8];
        let mut y_mont: [u32; 8] = [0; 8];

        P256_to_montgomery(x_mont.as_mut_ptr(), x);
        P256_to_montgomery(y_mont.as_mut_ptr(), y);
        P256_point_is_on_curve(x_mont.as_ptr(), y_mont.as_ptr())
    } else if (in_slice[0] >> 1) == 1 && input_len_in_bytes == 33 {
        P256_decompress_point(y, x, u32::from(in_slice[0] & 1))
    } else {
        false
    }
}
