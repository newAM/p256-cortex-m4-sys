#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::missing_safety_doc)]

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

    // void P256_mul_mod_n(uint32_t res[8], const uint32_t a[8], const uint32_t b[8]);
    fn P256_mul_mod_n(res: *mut u32, a: *const u32, b: *const u32);
    // void P256_add_mod_n(uint32_t res[8], const uint32_t a[8], const uint32_t b[8]);
    fn P256_add_mod_n(res: *mut u32, a: *const u32, b: *const u32);
    // void P256_mod_n_inv_vartime(uint32_t res[8], const uint32_t a[8]);
    fn P256_mod_n_inv(res: *mut u32, a: *const u32);
    // void P256_reduce_mod_n_32bytes(uint32_t res[8], const uint32_t a[8]);
    fn P256_reduce_mod_n_32bytes(res: *mut u32, a: *const u32);

    // void P256_select_point(uint32_t (*output)[8], uint32_t* table, uint32_t num_coordinates, uint32_t index);

    // void P256_jacobian_to_affine(uint32_t affine_mont_x[8], uint32_t affine_mont_y[8], const uint32_t jacobian_mont[3][8]);
    fn P256_jacobian_to_affine(
        affine_mont_x: *mut u32,
        affine_mont_y: *mut u32,
        jacobian_mont: *const [u32; 8],
    );
    // bool P256_point_is_on_curve(const uint32_t x_mont[8], const uint32_t y_mont[8]);
    fn P256_point_is_on_curve(x_mont: *const u32, y_mont: *const u32) -> bool;
    // bool P256_decompress_point(uint32_t y[8], const uint32_t x[8], uint32_t y_parity);
    fn P256_decompress_point(y: *mut u32, x: *const u32, y_parity: u32) -> bool;

    // void P256_double_j(uint32_t jacobian_point_out[3][8], const uint32_t jacobian_point_in[3][8]);
    fn P256_double_j(jacobian_point_out: *mut [u32; 8], jacobian_point_in: *const [u32; 8]);
    // void P256_add_sub_j(uint32_t jacobian_point1[3][8], const uint32_t (*point2)[8], bool is_sub, bool p2_is_affine);
    fn P256_add_sub_j(
        jacobian_point1: *mut [u32; 8],
        point2: *const [u32; 8],
        is_sub: bool,
        p2_is_affine: bool,
    );
    // bool P256_verify_last_step(const uint32_t r[8], const uint32_t jacobian_point[3][8]);
    fn P256_verify_last_step(r: *const u32, jacobian_point: *const [u32; 8]) -> bool;

    // void P256_negate_mod_p_if(uint32_t out[8], const uint32_t in[8], uint32_t should_negate);
    fn P256_negate_mod_p_if(out: *mut u32, inn: *const u32, should_negate: u32);
    // void P256_negate_mod_n_if(uint32_t out[8], const uint32_t in[8], uint32_t should_negate);
    fn P256_negate_mod_n_if(out: *mut u32, inn: *const u32, should_negate: u32);

    // TODO: remove this, was a C private function
    // void scalarmult_fixed_base(uint32_t output_mont_x[8], uint32_t output_mont_y[8], const uint32_t scalar[8]);
    fn scalarmult_fixed_base(output_mont_x: *mut u32, output_mont_y: *mut u32, scalar: *const u32);

    // TODO: remove this, was a C private function
    // void slide_257(signed char r[257], const uint8_t a[32]);
    fn slide_257(r: *mut i8, a: *const u8);
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

// This table contains 1G, 3G, 5G, ... 15G in affine coordinates in montgomery form
#[rustfmt::skip]
const P256_BASEPOINT_PRECOMP: [[[u32; 8]; 2]; 8]= [
[[0x18a9143c, 0x79e730d4, 0x5fedb601, 0x75ba95fc, 0x77622510, 0x79fb732b, 0xa53755c6, 0x18905f76],
[0xce95560a, 0xddf25357, 0xba19e45c, 0x8b4ab8e4, 0xdd21f325, 0xd2e88688, 0x25885d85, 0x8571ff18]],
[[0x4eebc127, 0xffac3f90, 0x87d81fb, 0xb027f84a, 0x87cbbc98, 0x66ad77dd, 0xb6ff747e, 0x26936a3f],
[0xc983a7eb, 0xb04c5c1f, 0x861fe1a, 0x583e47ad, 0x1a2ee98e, 0x78820831, 0xe587cc07, 0xd5f06a29]],
[[0xc45c61f5, 0xbe1b8aae, 0x94b9537d, 0x90ec649a, 0xd076c20c, 0x941cb5aa, 0x890523c8, 0xc9079605],
[0xe7ba4f10, 0xeb309b4a, 0xe5eb882b, 0x73c568ef, 0x7e7a1f68, 0x3540a987, 0x2dd1e916, 0x73a076bb]],
[[0xa0173b4f, 0x746354e, 0xd23c00f7, 0x2bd20213, 0xc23bb08, 0xf43eaab5, 0xc3123e03, 0x13ba5119],
[0x3f5b9d4d, 0x2847d030, 0x5da67bdd, 0x6742f2f2, 0x77c94195, 0xef933bdc, 0x6e240867, 0xeaedd915]],
[[0x264e20e8, 0x75c96e8f, 0x59a7a841, 0xabe6bfed, 0x44c8eb00, 0x2cc09c04, 0xf0c4e16b, 0xe05b3080],
[0xa45f3314, 0x1eb7777a, 0xce5d45e3, 0x56af7bed, 0x88b12f1a, 0x2b6e019a, 0xfd835f9b, 0x86659cd]],
[[0x6245e404, 0xea7d260a, 0x6e7fdfe0, 0x9de40795, 0x8dac1ab5, 0x1ff3a415, 0x649c9073, 0x3e7090f1],
[0x2b944e88, 0x1a768561, 0xe57f61c8, 0x250f939e, 0x1ead643d, 0xc0daa89, 0xe125b88e, 0x68930023]],
[[0x4b2ed709, 0xccc42563, 0x856fd30d, 0xe356769, 0x559e9811, 0xbcbcd43f, 0x5395b759, 0x738477ac],
[0xc00ee17f, 0x35752b90, 0x742ed2e3, 0x68748390, 0xbd1f5bc1, 0x7cd06422, 0xc9e7b797, 0xfbc08769]],
[[0xbc60055b, 0x72bcd8b7, 0x56e27e4b, 0x3cc23ee, 0xe4819370, 0xee337424, 0xad3da09, 0xe2aa0e43],
[0x6383c45d, 0x40b8524f, 0x42a41b25, 0xd7663554, 0x778a4797, 0x64efa6de, 0x7079adf4, 0x2042170a]]
];

// This contains two tables, 8 points each in affine coordinates in montgomery form
// The first table contains these points:
// (2^192 - 2^128 - 2^64 - 1)G
// (2^192 - 2^128 - 2^64 + 1)G
// (2^192 - 2^128 + 2^64 - 1)G
// (2^192 - 2^128 + 2^64 + 1)G
// (2^192 + 2^128 - 2^64 - 1)G
// (2^192 + 2^128 - 2^64 + 1)G
// (2^192 + 2^128 + 2^64 - 1)G
// (2^192 + 2^128 + 2^64 + 1)G
// The second table contains the same points multiplied by 2^32
#[rustfmt::skip]
const P256_BASEPOINT_PRECOMP2: [[[[u32; 8]; 2]; 8]; 2] =
[
[
[[0x670844e0, 0x52d8a7c9, 0xef68a29d, 0xe33bdc, 0x4bdb7361, 0xf3d2848, 0x91c5304d, 0x5222c821],
[0xdf73fc25, 0xea6d2944, 0x255c81b, 0xa04c0f55, 0xefe488a8, 0x29acdc97, 0x80a560de, 0xbe2e158f]],
[[0x2b13e673, 0xfc8511ee, 0xd103ed24, 0xffc58dee, 0xea7e99b8, 0x1022523a, 0x4afc8a17, 0x8f43ea39],
[0xc5f33d0b, 0x8f4e2dbc, 0xd0aa1681, 0x3bc099fa, 0x79ff9df1, 0xffbb7b41, 0xd58b57c4, 0x180de09d]],
[[0x8bd1cda5, 0x56430752, 0x8e05eda5, 0x1807577f, 0x956896e9, 0x99c699b, 0xf1f0efb5, 0x83d6093d],
[0xed97061c, 0xef5af17e, 0x30d4c3c, 0x35b977b8, 0x49229439, 0x81fa75a2, 0xa0b6d35d, 0xf5a22070]],
[[0x74f81cf1, 0x814c5365, 0x120065b, 0xe30baff7, 0x15132621, 0x80ae1256, 0x36a80788, 0x16d2b8cb],
[0xecc50bca, 0x33d14697, 0x17aedd21, 0x19a9dfb0, 0xedc3f766, 0x523fbcc7, 0xb2cf5afd, 0x9c4de6dd]],
[[0xcf0d9f6d, 0x5305a9e6, 0x81a9b021, 0x5839172f, 0x75c687cf, 0xcca7a4dd, 0x844be22f, 0x36d59b3e],
[0x111a53e9, 0xcace7e62, 0xf063f3a1, 0x91c843d4, 0xda812da, 0xbf77e5f0, 0x437f3176, 0xe64af9c]],
[[0xcf07517d, 0xdbd568bb, 0xba6830b9, 0x2f1afba2, 0xe6c4c2a6, 0x15b6807c, 0xe4966aef, 0x91c7eabc],
[0xd6b2b6e6, 0x716dea1b, 0x19f85b4b, 0x248c43d1, 0x4a315e2a, 0x16dcfd60, 0xc72b3d0b, 0x15fdd303]],
[[0x42b7dfd5, 0xe40bf9f4, 0x2d934f2a, 0x673689f3, 0x30a6f50b, 0x8314beb4, 0x976ec64e, 0xd17af2bc],
[0x1ee7ddf1, 0x39f66c4f, 0x68ea373c, 0x7f68e18b, 0x53d0b186, 0x5166c1f2, 0x7be58f14, 0x95dda601]],
[[0x42913074, 0xd5ae356, 0x48a542b1, 0x55491b27, 0xb310732a, 0x469ca665, 0x5f1a4cc1, 0x29591d52],
[0xb84f983f, 0xe76f5b6b, 0x9f5f84e1, 0xbe7eef41, 0x80baa189, 0x1200d496, 0x18ef332c, 0x6376551f]]
],
[
[[0x7c4e54f5, 0xb9e5cbc0, 0xe1410e34, 0xc53a1a17, 0xec454425, 0x3e199130, 0x1700902e, 0xb029c97e],
[0x786423b6, 0x2de66e11, 0xb41a95be, 0x262dc914, 0x451b683, 0x51766abd, 0x85bb6fb1, 0x55ad5f34]],
[[0x9066cb79, 0x74f4f1c, 0x30c8b94e, 0x1ab31bd6, 0xd74275b3, 0x6d3f012f, 0x9ddcce40, 0xa214d0b1],
[0xd165050a, 0x24aedf74, 0xe0e5dc3e, 0x95f17ece, 0xd9224456, 0x6ada9cda, 0x2dd60eea, 0x1fadb2d1]],
[[0xe20cfb9b, 0xa3d83091, 0xba76e0cb, 0xae79c975, 0xc8858a6e, 0xa5f2a588, 0x874a3168, 0xe897a5f4],
[0x7d48f096, 0xf6c1ef40, 0xc35b132c, 0x1f9c516b, 0x53c479fd, 0xe1040f91, 0x9df06743, 0x60e881f]],
[[0x52a90e51, 0x9e0ad72, 0x38c50a96, 0xb7e66ea3, 0x7d997770, 0xab32ad05, 0x445671cb, 0xceaffe2],
[0x5d37cc99, 0xdfbe753c, 0xe0fea2d5, 0x95d068cc, 0x4dd77cb6, 0x1e37cdda, 0x55530688, 0x88c5a4bb]],
[[0xc7744f1, 0x3413f033, 0xbc816702, 0x23c05c89, 0x1192b5ac, 0x2322ee9a, 0x373180bb, 0xc1636a0],
[0xbdde0207, 0xfe2f3d4, 0xc23578d8, 0xe1a093a, 0xc888ead, 0x6e5f0d1, 0x52a2b660, 0x9ca285a5]],
[[0xce923964, 0xdae76995, 0xa34c7993, 0xcc96493a, 0xea73d9e7, 0xd19b5144, 0x311e6e34, 0x4a5c263],
[0xd9a2a443, 0x7db5b32b, 0x2cfd960c, 0x3754bd33, 0xa430f15, 0xc5bcc98, 0xd9a94574, 0x5651201f]],
[[0xfc0418fe, 0xebdd8921, 0x34e20036, 0x37015b39, 0xdf03a353, 0xcf4fcd8f, 0xf12cab16, 0xdc2de6e1],
[0xd071df14, 0x9c17cc1a, 0x63415530, 0xd7c5e6a3, 0x68f3fb1e, 0xb5301660, 0x18269301, 0xb5f70bc9]],
[[0x79ec1a0f, 0x2d8daefd, 0xceb39c97, 0x3bbcd6fd, 0x58f61a95, 0xf5575ffc, 0xadf7b420, 0xdbd986c4],
[0x15f39eb7, 0x81aa8814, 0xb98d976c, 0x6ee2fcf5, 0xcf2f717d, 0x5465475d, 0x6860bbd0, 0x8e24d3c4]]
]
];

// Constant time abs
// but not really abs, only works for +/-15
#[inline(always)]
fn abs_int(a: i8) -> u32 {
    let a_u: u32 = a as i32 as u32;
    let mut mask: u32 = a_u >> 31;
    mask |= mask << 1;
    mask |= mask << 2;
    let mut result: u32 = ((-a) as u32) & mask;
    result |= (a as u32) & (mask ^ 0xF);
    result
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

    let input_mont_x: &[u32] = slice::from_raw_parts(input_mont_x, 8);
    let input_mont_y: &[u32] = slice::from_raw_parts(input_mont_y, 8);

    // The algorithm used requires the scalar to be odd. If even, negate the scalar modulo p to make it odd, and later negate the end result.
    let even: u32 = ((*scalar) & 1) ^ 1;
    let mut scalar2: [u32; 8] = [0; 8];
    P256_negate_mod_n_if(scalar2.as_mut_ptr(), scalar, even);

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
    P256_double_j(table[7].as_mut_ptr(), table[0].as_ptr());
    (1..8).for_each(|i| {
        table.copy_within(7..8, i);
        P256_add_sub_j(table[i].as_mut_ptr(), table[i - 1].as_ptr(), false, false);
    });

    // Calculate the result as (((((((((e[63]*G)*2^4)+e[62])*2^4)+e[61])*2^4)...)+e[1])*2^4)+e[0] = (2^252*e[63] + 2^248*e[62] + ... + e[0])*G.
    let mut current_point: [[u32; 8]; 3] = [[0; 8]; 3];

    // e[63] is never negative
    current_point.copy_from_slice(&table[usize::try_from(e[63] >> 1).unwrap()]);

    let mut i: usize = 62;
    loop {
        (0..4).for_each(|_| {
            P256_double_j(current_point.as_mut_ptr(), current_point.as_ptr());
        });

        let mut selected_point: [[u32; 8]; 3] = [[0; 8]; 3];
        selected_point.copy_from_slice(&table[(abs_int(e[i]) >> 1) as usize]);
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
            current_point.as_mut_ptr(),
            selected_point.as_ptr(),
            false,
            false,
        );

        i = match i.checked_sub(1) {
            Some(i) => i,
            None => break,
        }
    }

    P256_jacobian_to_affine(output_mont_x, output_mont_y, current_point.as_ptr());

    // If the scalar was initially even, we now negate the result to get the correct result, since -(scalar*G) = (-scalar*G).
    // This is done by negating y, since -(x,y) = (x,-y).
    P256_negate_mod_p_if(output_mont_y, output_mont_y, even);
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

/// Generates the shared secret according to the ECDH standard.
///
/// The shared secret parameter will contain the big endian encoding for the x coordinate of the scalar
/// multiplication of the private key and the input point (other's public key), if the function succeeds.
///
/// If the other's public key point does not lie on the curve, this function fails and false is returned.
/// Otherwise, shared secret is calculated and true is returned.
///
/// NOTE: The return value MUST be checked since the other's public key point cannot generally be trusted.
#[no_mangle]
pub unsafe extern "C" fn p256_ecdh_calc_shared_secret(
    shared_secret: *mut u8,
    private_key: *const u32,
    others_public_key_x: *const u32,
    others_public_key_y: *const u32,
) -> bool {
    // uint8_t shared_secret[32], const uint32_t private_key[8], const uint32_t others_public_key_x[8], const uint32_t others_public_key_y[8]
    let mut result_x: [u32; 8] = [0; 8];
    let mut result_y: [u32; 8] = [0; 8];
    if !p256_scalarmult_generic_no_scalar_check(
        result_x.as_mut_ptr(),
        result_y.as_mut_ptr(),
        private_key,
        others_public_key_x,
        others_public_key_y,
    ) {
        false
    } else {
        P256_from_montgomery(result_x.as_mut_ptr(), result_x.as_mut_ptr());
        p256_convert_endianness(shared_secret as *mut _, result_x.as_mut_ptr() as *mut _, 32);
        true
    }
}

/// Calculates the public key from a given private key for use by either ECDSA or ECDH.
///
/// The private key shall be taken from a random value that MUST have been generated by a cryptographically
/// secure random number generator that generates 256 random bits. This function validates that the private key
/// lies in the accepted range 1 to n-1, where n is the order of the elliptic curve, and returns true only if
/// this validation succeeds. If random value is out of that range, false is returned and in this case a new
/// random value needs to be generated and this function MUST be called again until true is returned.
///
/// The public key is created by performing a scalar multiplication of the private key and the base point of
/// the curve.
///
/// Only use a keypair for either ECDSA or ECDH, not both, and don't use the private key for any other purposes.
#[no_mangle]
pub unsafe extern "C" fn p256_keygen(
    public_key_x: *mut u32,
    public_key_y: *mut u32,
    private_key: *const u32,
) -> bool {
    // uint32_t public_key_x[8], uint32_t public_key_y[8], const uint32_t private_key[8]
    p256_scalarmult_base(public_key_x, public_key_y, private_key)
}

/*
macro_rules! get_bit {
    ($arr:ident, $i:expr) => {
        (($arr[$i / 32] >> ($i % 32)) & 1)
    };
}

// Calculates scalar*G in constant time
#[no_mangle]
unsafe extern "C" fn scalarmult_fixed_base(
    output_mont_x: *mut u32,
    output_mont_y: *mut u32,
    scalar: *const u32,
) {
    // u32 output_mont_x[8], u32 output_mont_y[8], const u32 scalar[8]

    // Just as with the algorithm used in variable base scalar multiplication, this algorithm requires the scalar to be odd.
    let even: u32 = ((*scalar) & 1) ^ 1;
    let mut scalar2: [u32; 8] = [0; 8];
    P256_negate_mod_n_if(scalar2.as_mut_ptr(), scalar, even);

    // This algorithm conceptually rewrites the odd scalar as s[0] + 2^1*s[1] + 2^2*s[2] + ... + 2^255*s[255], where each s[i] is -1 or 1.
    // By initially setting s[i] to the corresponding bit S[i] in the original odd scalar S, we go from lsb to msb, and whenever a value s[i] is 0,
    // increase s[i] by 1 and decrease s[i-1] by 2.
    // This will result in that s[i] = S[i+1] == 1 ? 1 : -1 for i < 255, and s[255] = 1.

    // We then form the scalars abs(s[j] + s[j+64]*2^64 + s[j+128]*2^128 + s[j+192]*2^192)*(2^32 * floor(j / 32)) for different 0 <= j < 64.
    // Each scalar times G has already been precomputed in p256_basepoint_precomp2.
    // That way we only need 31 point doublings and 63 point additions.

    let mut current_point: [[u32; 8]; 3] = [[0; 8]; 3];
    let mut selected_point: [[u32; 8]; 2] = [[0; 8]; 2];

    let mut i: usize = 31;
    loop {
        {
            let mut mask: u32 = get_bit!(scalar2, i + 32 + 1)
                | (get_bit!(scalar2, i + 64 + 32 + 1) << 1)
                | (get_bit!(scalar2, i + 2 * 64 + 32 + 1) << 2);
            if i == 31 {
                current_point[..2].copy_from_slice(&P256_BASEPOINT_PRECOMP2[1][mask as usize]);
                current_point[2].copy_from_slice(&ONE_MONTGOMERY);
            } else {
                P256_double_j(
                    current_point.as_mut_ptr() as *mut *mut u32,
                    current_point.as_ptr() as *const *const u32,
                );

                let sign: u32 = get_bit!(scalar2, i + 3 * 64 + 1).wrapping_sub(1); // positive: 0, negative: -1
                mask = (mask ^ sign) & 7;
                selected_point.copy_from_slice(&P256_BASEPOINT_PRECOMP2[1][mask as usize]);
                P256_negate_mod_p_if(
                    selected_point[1].as_mut_ptr(),
                    selected_point[1].as_ptr(),
                    sign & 1,
                );
                P256_add_sub_j(
                    current_point.as_mut_ptr() as *mut *mut u32,
                    selected_point.as_mut_ptr() as *const *mut u32,
                    false,
                    true,
                );
            }
        }
        {
            let mut mask: u32 = get_bit!(scalar2, i + 1)
                | (get_bit!(scalar2, i + 64 + 1) << 1)
                | (get_bit!(scalar2, i + 2 * 64 + 1) << 2);
            let sign: u32 = get_bit!(scalar2, i + 3 * 64 + 1).wrapping_sub(1); // positive: 0, negative: -1
            mask = (mask ^ sign) & 7;
            selected_point.copy_from_slice(&P256_BASEPOINT_PRECOMP2[0][mask as usize]);
            P256_negate_mod_p_if(
                selected_point[1].as_mut_ptr(),
                selected_point[1].as_ptr(),
                sign & 1,
            );
            P256_add_sub_j(
                current_point.as_mut_ptr() as *mut *mut u32,
                selected_point.as_mut_ptr() as *const *mut u32,
                false,
                true,
            );
        }

        i = match i.checked_sub(1) {
            Some(i) => i,
            None => break,
        }
    }

    P256_jacobian_to_affine(
        output_mont_x,
        output_mont_y,
        current_point.as_ptr() as *const *mut u32,
    );

    // Negate final result if the scalar was initially even.
    P256_negate_mod_p_if(output_mont_y, output_mont_y, even);
}
*/

/// Raw scalar multiplication by the base point of the elliptic curve.
///
/// This function can be used to implement custom algorithms using the P-256 curve.
///
/// This function validates that the scalar lies in the accepted range 1 to n-1, where n is the order of the
/// elliptic curve, and returns true only if this validation succeeds. Otherwise false is returned.
#[no_mangle]
pub unsafe extern "C" fn p256_scalarmult_base(
    result_x: *mut u32,
    result_y: *mut u32,
    scalar: *const u32,
) -> bool {
    // u32 result_x[8], u32 result_y[8], const u32 scalar[8]
    if !P256_check_range_n(scalar) {
        false
    } else {
        scalarmult_fixed_base(result_x, result_y, scalar);
        P256_from_montgomery(result_x, result_x);
        P256_from_montgomery(result_y, result_y);
        true
    }
}

/// Sign precomputation state.
///
/// The content shall be treated as opaque to the API user and shall not be inspected or modified.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct SignPrecomp {
    pub r: [u32; 8],
    pub k_inv: [u32; 8],
}

/// Creates an ECDSA signature.
///
/// The parameter "k" shall consist of a 256-bit random integer value. This random value MUST be generated from
/// a cryptographically secure random number generator, and MUST be unique for every pair of message hash and
/// private key.
///
/// With a small probability (~ 2^-32), this function will fail and return false for the given "k" and this
/// function MUST in that case be called again with a new random "k", until true is returned. This is in line
/// with the ECDSA standard.
///
/// As an alternative to using a random "k", "k" might be derived deterministically from the input, using a
/// sophisticated hash construction such as RFC 6979, or e.g. by hashing the private key, message hash and a
/// retry counter, using a secure hash function such as SHA-256.
#[no_mangle]
pub unsafe extern "C" fn p256_sign(
    r: *mut u32,
    s: *mut u32,
    hash: *const u8,
    hashlen_in_bytes: u32,
    private_key: *const u32,
    k: *const u32,
) -> bool {
    // uint32_t r[8], uint32_t s[8], const uint8_t* hash, uint32_t hashlen_in_bytes, const uint32_t private_key[8], const uint32_t k[8]
    let mut t: SignPrecomp = Default::default();
    if !p256_sign_step1(&mut t, k) {
        (0..8).for_each(|offset| {
            *r.offset(offset) = 0;
            *s.offset(offset) = 0;
        });
        false
    } else {
        p256_sign_step2(r, s, hash, hashlen_in_bytes, private_key, &mut t)
    }
}

/// Creates an ECDSA signature, using a two-step procedure.
///
/// This function performs the first of two steps, and accounts for 99% of the time spent for generating an
/// ECDSA signature.
///
/// By splitting up into two steps, most of the work could be spent before deciding what message to sign, or
/// which private key to use.
///
/// The parameter "k" shall consist of a 256-bit random integer value. This random value MUST be generated from
/// a cryptographically secure random number generator, and MUST be unique for every pair of message hash and
/// private key.
///
/// With a small probability (~ 2^-32), this function will fail and return false for the given "k" and this
/// function MUST in that case be called again with a new random "k", until true is returned. This is in line
/// with the ECDSA standard.
///
/// As an alternative to using a random "k", "k" might be derived deterministically from the input, using a
/// sophisticated hash construction such as RFC 6979, or e.g. by hashing the private key, message hash and a
/// retry counter, using a secure hash function such as SHA-256.
///
/// The "result" parameter will contain the computed state, that is later to be passed to p256_sign_step2.
/// A result state MUST NOT be reused for generating multiple signatures.
#[no_mangle]
pub unsafe extern "C" fn p256_sign_step1(result: &mut SignPrecomp, k: *const u32) -> bool {
    // p256_sign_step1(struct SignPrecomp *result, const uint32_t k[8])

    #[allow(clippy::never_loop)]
    loop {
        let mut point_res: [[u32; 8]; 2] = [[0; 8]; 2];
        if !P256_check_range_n(k) {
            break;
        }
        scalarmult_fixed_base(point_res[0].as_mut_ptr(), point_res[1].as_mut_ptr(), k);
        P256_mod_n_inv(result.k_inv.as_mut_ptr(), k);
        P256_from_montgomery(result.r.as_mut_ptr(), point_res[0].as_ptr());
        P256_reduce_mod_n_32bytes(result.r.as_mut_ptr(), result.r.as_ptr());

        let r_sum: u32 = (0..8).fold(0, |r_sum, i| r_sum | result.r[i]);
        if r_sum == 0 {
            break;
        }
        return true;
    }

    result.r.fill(0);
    result.k_inv.fill(0);
    false
}

// Takes the leftmost 256 bits in hash (treated as big endian),
// and converts to little endian integer z.
fn hash_to_z(z: &mut [u8], hash: &[u8]) {
    let hashlen: usize = core::cmp::min(hash.len(), 32);
    (0..hashlen).for_each(|i| {
        z[i] = hash[hashlen - 1 - i];
    });
    z[hashlen..].fill(0);
}

/// Second step of creating an ECDSA signature, using a two-step procedure.
///
/// This function performs the second of two steps, and accounts for the last 1% of the time spent for generating
/// an ECDSA signature.
///
/// The "sign_precomp" parameter shall contain a pointer to a state generated by p256_sign_step1.
///
/// With a small probability (~ 2^-256), this function will fail, due to the given "k" from the first step is
/// not compatible with the rest of the input, and return false. In this case, the procedure MUST be started
/// over from step 1 with a new random "k".  This is in line with the ECDSA standard. Otherwise true is returned
/// and the signature is placed in "r" and "s".
///
/// When this function returns, "sign_precomp" is also zeroed out and may hence not be reused.
#[no_mangle]
pub unsafe extern "C" fn p256_sign_step2(
    r: *mut u32,
    s: *mut u32,
    hash: *const u8,
    hashlen_in_bytes: u32,
    private_key: *const u32,
    sign_precomp: &mut SignPrecomp,
) -> bool {
    // p256_sign_step2(uint32_t r[8], uint32_t s[8], const uint8_t* hash, uint32_t hashlen_in_bytes,
    //     const uint32_t private_key[8], struct SignPrecomp *sign_precomp)
    //     __attribute__((warn_unused_result));
    #[allow(clippy::never_loop)]
    loop {
        // just make sure user did not input an obviously invalid precomp
        if !P256_check_range_n(sign_precomp.k_inv.as_ptr())
            || !P256_check_range_n(sign_precomp.r.as_ptr())
        {
            break;
        }
        let z: &mut [u8] = slice::from_raw_parts_mut(r as *mut u8, hashlen_in_bytes as usize);
        hash_to_z(z, slice::from_raw_parts(hash, hashlen_in_bytes as usize));
        P256_mul_mod_n(s, sign_precomp.r.as_ptr(), private_key);
        P256_add_mod_n(s, r as *const u32, s);
        P256_mul_mod_n(s, sign_precomp.k_inv.as_ptr(), s);

        r.copy_from(sign_precomp.r.as_ptr(), 8);

        let s_sum: u32 = (0..8).fold(0, |s_sum, i| s_sum | *s.offset(i));
        if s_sum == 0 {
            break;
        }
        sign_precomp.r.fill(0);
        sign_precomp.k_inv.fill(0);
        return true;
    }

    (0..8).for_each(|offset| {
        *r.offset(offset) = 0;
        *s.offset(offset) = 0;
    });
    false
}

extern "C" {
    /// Verifies an ECDSA signature.
    ///
    /// Returns true if the signature is valid for the given input, otherwise false.
    pub fn p256_verify(
        public_key_x: *const u32,
        public_key_y: *const u32,
        hash: *const u8,
        hashlen_in_bytes: u32,
        r: *const u32,
        s: *const u32,
    ) -> bool;
}
/*
/// Verifies an ECDSA signature.
///
/// Returns true if the signature is valid for the given input, otherwise false.
#[no_mangle]
pub unsafe extern "C" fn p256_verify(
    public_key_x: *const u32,
    public_key_y: *const u32,
    hash: *const u8,
    hashlen_in_bytes: u32,
    r: *const u32,
    s: *const u32,
) -> bool {
    // const uint32_t public_key_x[8], const uint32_t public_key_y[8], const uint8_t* hash, uint32_t hashlen_in_bytes, const uint32_t r[8], const uint32_t s[8]
    if !P256_check_range_n(r) || !P256_check_range_n(s) {
        return false;
    }

    if !P256_check_range_p(public_key_x) || !P256_check_range_p(public_key_y) {
        return false;
    }

    let mut pk_table: [[[u32; 8]; 3]; 8] = [[[0; 8]; 3]; 8];
    P256_to_montgomery(pk_table[0][0].as_mut_ptr(), public_key_x);
    P256_to_montgomery(pk_table[0][1].as_mut_ptr(), public_key_y);
    pk_table[0][2].copy_from_slice(&ONE_MONTGOMERY);

    if !P256_point_is_on_curve(pk_table[0][0].as_ptr(), pk_table[0][1].as_ptr()) {
        return false;
    }

    // Create a table of P, 3P, 5P, ..., 15P, where P is the public key.
    P256_double_j(
        pk_table[7].as_mut_ptr() as *mut *mut u32,
        pk_table[0].as_ptr() as *const *const u32,
    );
    (1..8).for_each(|i| {
        pk_table.copy_within(i..(i + 1), 7);
        P256_add_sub_j(
            pk_table[i].as_mut_ptr() as *mut *mut u32,
            pk_table[i - 1].as_ptr() as *const *const u32,
            false,
            false,
        );
    });

    let mut w: [u32; 8] = [0; 8];
    let mut u1: [u32; 8] = [0; 8];
    let mut u2: [u32; 8] = [0; 8];

    let z: &mut [u8] = slice::from_raw_parts_mut(r as *mut u8, hashlen_in_bytes as usize);
    hash_to_z(z, slice::from_raw_parts(hash, hashlen_in_bytes as usize));

    P256_mod_n_inv(w.as_mut_ptr(), s);

    P256_mul_mod_n(u1.as_mut_ptr(), r as *const u32, w.as_ptr());
    P256_mul_mod_n(u2.as_mut_ptr(), r, w.as_ptr());

    // Each value in these arrays will be an odd integer v, so that -15 <= v <= 15.
    // Around 1/5.5 of them will be non-zero.

    let mut slide_bp: [i8; 257] = [0; 257];
    let mut slide_pk: [i8; 257] = [0; 257];

    slide_257(slide_bp.as_mut_ptr(), u1.as_ptr() as *const u8);
    slide_257(slide_pk.as_mut_ptr(), u2.as_ptr() as *const u8);

    let mut cp: [[u32; 3]; 8] = [[0; 3]; 8];

    let mut i: usize = 255;
    #[allow(clippy::comparison_chain)]
    loop {
        P256_double_j(
            cp.as_mut_ptr() as *mut *mut u32,
            cp.as_ptr() as *const *const u32,
        );
        if slide_bp[i] > 0 {
            P256_add_sub_j(
                cp.as_mut_ptr() as *mut *mut u32,
                P256_BASEPOINT_PRECOMP[(slide_bp[i] / 2) as usize].as_ptr() as *const *const u32,
                false,
                true,
            );
        } else if slide_bp[i] < 0 {
            P256_add_sub_j(
                cp.as_mut_ptr() as *mut *mut u32,
                P256_BASEPOINT_PRECOMP[((-slide_bp[i]) / 2) as usize].as_ptr() as *const *const u32,
                true,
                true,
            );
        }
        if slide_pk[i] > 0 {
            P256_add_sub_j(
                cp.as_mut_ptr() as *mut *mut u32,
                pk_table[(slide_pk[i] / 2) as usize].as_ptr() as *const *const u32,
                false,
                false,
            );
        } else if slide_pk[i] < 0 {
            P256_add_sub_j(
                cp.as_mut_ptr() as *mut *mut u32,
                pk_table[((-slide_pk[i]) / 2) as usize].as_ptr() as *const *const u32,
                true,
                false,
            );
        }

        i = match i.checked_sub(1) {
            Some(i) => i,
            None => break,
        }
    }

    P256_verify_last_step(r, cp.as_ptr() as *const *const u32)
}
*/
