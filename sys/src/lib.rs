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
    fn P256_from_montgomery(a: *mut u32, aR: *const u32);
    // bool P256_check_range_p(const uint32_t a[8]);

    // bool P256_check_range_n(const uint32_t a[8]);
    // void P256_mul_mod_n(uint32_t res[8], const uint32_t a[8], const uint32_t b[8]);
    // void P256_add_mod_n(uint32_t res[8], const uint32_t a[8], const uint32_t b[8]);
    // void P256_mod_n_inv_vartime(uint32_t res[8], const uint32_t a[8]);
    // void P256_reduce_mod_n_32bytes(uint32_t res[8], const uint32_t a[8]);

    // void P256_select_point(uint32_t (*output)[8], uint32_t* table, uint32_t num_coordinates, uint32_t index);

    // void P256_jacobian_to_affine(uint32_t affine_mont_x[8], uint32_t affine_mont_y[8], const uint32_t jacobian_mont[3][8]);
    fn P256_jacobian_to_affine(
        affine_mont_x: *mut u32,
        affine_mont_y: *mut u32,
        jacobian_mont: *const *const u32,
    );
    // bool P256_point_is_on_curve(const uint32_t x_mont[8], const uint32_t y_mont[8]);
    fn P256_point_is_on_curve(x_mont: *const u32, y_mont: *const u32) -> bool;
    // bool P256_decompress_point(uint32_t y[8], const uint32_t x[8], uint32_t y_parity);
    fn P256_decompress_point(y: *mut u32, x: *const u32, y_parity: u32) -> bool;

    // void P256_double_j(uint32_t jacobian_point_out[3][8], const uint32_t jacobian_point_in[3][8]);
    fn P256_double_j(jacobian_point_out: *mut *mut u32, jacobian_point_in: *const *const u32);
    // void P256_add_sub_j(uint32_t jacobian_point1[3][8], const uint32_t (*point2)[8], bool is_sub, bool p2_is_affine);
    fn P256_add_sub_j(
        jacobian_point1: *mut *mut u32,
        point2: *const *mut u32,
        is_sub: bool,
        p2_is_affine: bool,
    );
    // bool P256_verify_last_step(const uint32_t r[8], const uint32_t jacobian_point[3][8]);

    // void P256_negate_mod_p_if(uint32_t out[8], const uint32_t in[8], uint32_t should_negate);
    fn P256_negate_mod_p_if(out: *mut u32, inn: *const u32, should_negate: u32);
    // void P256_negate_mod_n_if(uint32_t out[8], const uint32_t in[8], uint32_t should_negate);
    fn P256_negate_mod_n_if(out: *mut u32, inn: *const u32, should_negate: u32);
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

const ONE_MONTGOMERY: [u32; 8] = [1, 0, 0, 0xffffffff, 0xffffffff, 0xffffffff, 0xfffffffe, 0];

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
    out_slice[0] = 2_u8.wrapping_add((y_slice[0] & 1) as u8);
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
    out_slice[0] = 6_u8.wrapping_add((y_slice[0] & 1) as u8);
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

// Calculates scalar*P in constant time (except for the scalars 2 and n-2, for which the results take a few extra cycles to compute)
#[no_mangle]
unsafe extern "C" fn scalarmult_variable_base(
    output_mont_x: *mut u32,
    output_mont_y: *mut u32,
    input_mont_x: *const u32,
    input_mont_y: *const u32,
    scalar: *const u32,
) {
    // uint32_t output_mont_x[8], uint32_t output_mont_y[8], const uint32_t input_mont_x[8], const uint32_t input_mont_y[8], const uint32_t scalar[8]

    // Based on https://eprint.iacr.org/2014/130.pdf, Algorithm 1.

    let output_mont_x: &mut [u32] = slice::from_raw_parts_mut(output_mont_x, 8);
    let output_mont_y: &mut [u32] = slice::from_raw_parts_mut(output_mont_y, 8);
    let input_mont_x: &[u32] = slice::from_raw_parts(input_mont_x, 8);
    let input_mont_y: &[u32] = slice::from_raw_parts(input_mont_y, 8);
    let scalar: &[u32] = slice::from_raw_parts(scalar, 8);

    // The algorithm used requires the scalar to be odd. If even, negate the scalar modulo p to make it odd, and later negate the end result.
    let even: u32 = (scalar[0] & 1) ^ 1;
    let mut scalar2: [u32; 8] = [0; 8];
    P256_negate_mod_n_if(scalar2.as_mut_ptr(), scalar.as_ptr(), even);

    // Rewrite the scalar as e[0] + 2^4*e[1] + 2^8*e[2] + ... + 2^252*e[63], where each e[i] is an odd number and -15 <= e[i] <= 15.
    let mut e: [i8; 64] = [0; 64];
    (1..64).for_each(|i| {
        // Extract 4 bits
        e[i] = ((scalar2[i / 8] >> ((i % 8) * 4)) & 0xf) as i8;
        // If even, subtract 2^4 from e[i - 1] and add 1 to e[i]
        e[i - 1] -= ((e[i] & 1) ^ 1) << 4;
        e[i] |= 1;
    });

    // Create a table of P, 3P, 5P, ... 15P.
    let mut table: [[[u32; 8]; 3]; 8] = [[[0; 8]; 3]; 8];
    table[0][0].copy_from_slice(input_mont_x);
    table[0][1].copy_from_slice(input_mont_y);
    table[0][2].copy_from_slice(ONE_MONTGOMERY.as_slice());
    P256_double_j(
        table[7].as_ptr() as *mut *mut u32,
        table[0].as_ptr() as *const *const u32,
    );
    (1..8).for_each(|i| {
        // TODO: is this eqivalent to this?
        // memcpy(table[i], table[7], 96);
        table.copy_within(7..8, i);
        P256_add_sub_j(
            table[i].as_mut_ptr() as *mut *mut u32,
            table[i - 1].as_mut_ptr() as *const *mut u32,
            false,
            false,
        );
    });

    // Calculate the result as (((((((((e[63]*G)*2^4)+e[62])*2^4)+e[61])*2^4)...)+e[1])*2^4)+e[0] = (2^252*e[63] + 2^248*e[62] + ... + e[0])*G.
    let mut current_point: [[u32; 8]; 3] = [[0; 8]; 3];

    // e[63] is never negative
    current_point.copy_from_slice(&table[usize::try_from(e[63] >> 1).unwrap()]);

    let mut i = 63;
    while i > 0 {
        (0..3).for_each(|_| {
            P256_double_j(
                current_point.as_mut_ptr() as *mut *mut u32,
                current_point.as_ptr() as *const *const u32,
            );
        });

        let mut selected_point: [[u32; 8]; 3] = [[0; 8]; 3];
        // TODO: this needs to be a constant-time abs_diff, see the original C
        selected_point.copy_from_slice(&table[usize::from(e[i].abs_diff(0) >> 1)]);
        P256_negate_mod_p_if(
            selected_point[1].as_mut_ptr(),
            selected_point[1].as_ptr(),
            (e[i] >> 7) as u32,
        );

        // There is (only) one odd input scalar that leads to an exception when i == 0: n-2,
        // in that case current_point will be equal to selected_point and hence a doubling
        // will occur instead. We don't bother fixing the same constant time for that case since
        // the probability of that random value to be generated is around 1/2^255 and an
        // attacker could easily test this case anyway.
        P256_add_sub_j(
            current_point.as_mut_ptr() as *mut *mut u32,
            selected_point.as_mut_ptr() as *const *mut u32,
            false,
            false,
        );
        i -= 1;
    }

    P256_jacobian_to_affine(
        output_mont_x.as_mut_ptr(),
        output_mont_y.as_mut_ptr(),
        current_point.as_ptr() as *const *const u32,
    );

    // If the scalar was initially even, we now negate the result to get the correct result, since -(scalar*G) = (-scalar*G).
    // This is done by negating y, since -(x,y) = (x,-y).
    P256_negate_mod_p_if(output_mont_y.as_mut_ptr(), output_mont_y.as_ptr(), even);
}

#[no_mangle]
unsafe extern "C" fn p256_scalarmult_generic_no_scalar_check(
    output_mont_x: *mut u32,
    output_mont_y: *mut u32,
    scalar: *const u32,
    in_x: *const u32,
    in_y: *const u32,
) -> bool {
    // uint32_t output_mont_x[8], uint32_t output_mont_y[8], const uint32_t scalar[8], const uint32_t in_x[8], const uint32_t in_y[8]

    if !P256_check_range_p(in_x) || !P256_check_range_p(in_y) {
        false
    } else {
        P256_to_montgomery(output_mont_x, in_x);
        P256_to_montgomery(output_mont_y, in_y);

        if !P256_point_is_on_curve(output_mont_x, output_mont_y) {
            false
        } else {
            scalarmult_variable_base(
                output_mont_x,
                output_mont_y,
                output_mont_x,
                output_mont_y,
                scalar,
            );
            true
        }
    }
}

/// Raw scalar multiplication by any point on the elliptic curve.
///
/// This function can be used to implement custom algorithms using the P-256 curve.
///
/// This function validates all inputs and proceeds only if the scalar is within the range 1 to n-1, where n
/// is the order of the elliptic curve, and the input point's coordinates are each less than the order of
/// the prime field. If validation succeeds, true is returned. Otherwise false is returned.
#[no_mangle]
pub unsafe extern "C" fn p256_scalarmult_generic(
    result_x: *mut u32,
    result_y: *mut u32,
    scalar: *const u32,
    in_x: *const u32,
    in_y: *const u32,
) -> bool {
    // uint32_t result_x[8], uint32_t result_y[8], const uint32_t scalar[8], const uint32_t in_x[8], const uint32_t in_y[8]
    if !P256_check_range_n(scalar)
        || !p256_scalarmult_generic_no_scalar_check(result_x, result_y, scalar, in_x, in_y)
    {
        false
    } else {
        P256_from_montgomery(result_x, result_x);
        P256_from_montgomery(result_y, result_y);
        true
    }
}
