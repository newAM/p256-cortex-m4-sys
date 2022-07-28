#![no_std]
#![no_main]

use defmt::unwrap;
use defmt_rtt as _; // global logger
use nucleo_wl55jc_bsp::hal::{
    cortex_m,
    pac::{self, DWT},
    rcc,
};
use panic_probe as _;

const FREQ: u32 = 48_000_000;
const CYC_PER_MICRO: u32 = FREQ / 1000 / 1000;

// WARNING will wrap-around eventually, use this for relative timing only
defmt::timestamp!("{=u32:us}", DWT::cycle_count() / CYC_PER_MICRO);

// Message hash
const HASH: [u32; 8] = [
    0x44acf6b7, 0xe36c1342, 0xc2c58972, 0x04fe0950, 0x4e1e2efb, 0x1a900377, 0xdbc4e7a6, 0xa133ec56,
];

const PRIVATE_KEY: [u32; 8] = [
    0x519b423d, 0x715f8b58, 0x1f4fa8ee, 0x59f4771a, 0x5b44c813, 0x0b4e3eac, 0xca54a56d, 0xda72b464,
];

// Note: in real-world use this should be a one-time random number (nonce).
// This fixed value is for testing purposes only.
const INTEGER: [u32; 8] = [
    0x94a1bbb1, 0x4b906a61, 0xa280f245, 0xf9e93c7f, 0x3b4a6247, 0x824f5d33, 0xb9670787, 0x642a68de,
];

const R_SIGN: [u32; 8] = [
    0xf3ac8061, 0xb514795b, 0x8843e3d6, 0x629527ed, 0x2afd6b1f, 0x6a555a7a, 0xcabb5e6f, 0x79c8c2ac,
];
const S_SIGN: [u32; 8] = [
    0x8bf77819, 0xca05a6b2, 0x786c7626, 0x2bf7371c, 0xef97b218, 0xe96f175a, 0x3ccdda2a, 0xcc058903,
];

const CURVE_PT_X: [u32; 8] = [
    0x1ccbe91c, 0x075fc7f4, 0xf033bfa2, 0x48db8fcc, 0xd3565de9, 0x4bbfb12f, 0x3c59ff46, 0xc271bf83,
];

const CURVE_PT_Y: [u32; 8] = [
    0xce4014c6, 0x8811f9a2, 0x1a1fdb2c, 0x0e6113e0, 0x6db7ca93, 0xb7404e78, 0xdc7ccd5c, 0xa89a4ca9,
];

const fn swap32(i: [u32; 8]) -> [u32; 8] {
    [
        i[0].swap_bytes(),
        i[1].swap_bytes(),
        i[2].swap_bytes(),
        i[3].swap_bytes(),
        i[4].swap_bytes(),
        i[5].swap_bytes(),
        i[6].swap_bytes(),
        i[7].swap_bytes(),
    ]
}

fn into_bytes(i: [u32; 8]) -> [u8; 32] {
    unsafe { core::mem::transmute::<[u32; 8], [u8; 32]>(i) }
}

// for use with rust-crypto
const HASH_SWAP: [u32; 8] = swap32(HASH);
const INTEGER_SWAP: [u32; 8] = swap32(INTEGER);
const PRIVATE_KEY_SWAP: [u32; 8] = swap32(PRIVATE_KEY);
const R_SIGN_SWAP: [u32; 8] = swap32(R_SIGN);
const S_SIGN_SWAP: [u32; 8] = swap32(S_SIGN);
const CURVE_PT_X_SWAP: [u32; 8] = swap32(CURVE_PT_X);
const CURVE_PT_Y_SWAP: [u32; 8] = swap32(CURVE_PT_Y);

fn convert(i: [u32; 8]) -> [u32; 8] {
    let mut ret = core::mem::MaybeUninit::<[u32; 8]>::uninit();
    unsafe {
        p256_cortex_m4_sys::p256_convert_endianness(
            ret.as_mut_ptr() as *mut _,
            i.as_ptr() as *const _,
            32,
        );
        ret.assume_init()
    }
}

#[defmt_test::tests]
mod tests {
    use super::*;

    const ZERO: [u32; 8] = [0; 8];
    const ONE: [u32; 8] = [0, 0, 0, 0, 0, 0, 0, 1];

    #[init]
    fn init() {
        let mut cp: pac::CorePeripherals = unwrap!(pac::CorePeripherals::take());
        let mut dp: pac::Peripherals = unwrap!(pac::Peripherals::take());

        cortex_m::interrupt::free(|cs| unsafe {
            rcc::set_sysclk_msi_max(&mut dp.FLASH, &mut dp.PWR, &mut dp.RCC, cs)
        });

        cp.DCB.enable_trace();
        cp.DWT.enable_cycle_counter();
        cp.DWT.set_cycle_count(0);
    }

    #[test]
    fn check_range_n() {
        use p256_cortex_m4_sys::P256_check_range_n;

        let valid: bool = unsafe { P256_check_range_n(ZERO.as_ptr()) };
        defmt::assert!(!valid, "0 is not in range");

        let valid: bool = unsafe { P256_check_range_n(ONE.as_ptr()) };
        defmt::assert!(valid, "1 is in range");

        // 2**256 - 2**224 + 2**192 - 0x4319055258e8617b0c46353d039cdaaf
        const N: [u32; 8] = [
            0xfc632551, 0xf3b9cac2, 0xa7179e84, 0xbce6faad, 0xffffffff, 0xffffffff, 0x00000000,
            0xffffffff,
        ];
        let valid: bool = unsafe { P256_check_range_n(N.as_ptr()) };
        defmt::assert!(!valid, "N is not within range");

        const N_MINUS_ONE: [u32; 8] = [
            0xfc632550, 0xf3b9cac2, 0xa7179e84, 0xbce6faad, 0xffffffff, 0xffffffff, 0x00000000,
            0xffffffff,
        ];
        let valid: bool = unsafe { P256_check_range_n(N_MINUS_ONE.as_ptr()) };
        defmt::assert!(valid, "N - 1 is within range");
    }

    #[test]
    fn check_range_p() {
        use p256_cortex_m4_sys::P256_check_range_p;

        let valid: bool = unsafe { P256_check_range_p(ZERO.as_ptr()) };
        defmt::assert!(valid, "0 is in range");

        let valid: bool = unsafe { P256_check_range_p(ONE.as_ptr()) };
        defmt::assert!(valid, "1 is in range");

        // 2**256 - 2**224 + 2**192 + 2**96 - 1
        const P: [u32; 8] = [
            0xffffffff, 0xffffffff, 0xffffffff, 0x00000000, 0x00000000, 0x00000000, 0x00000001,
            0xffffffff,
        ];
        let valid: bool = unsafe { P256_check_range_p(P.as_ptr()) };
        defmt::assert!(!valid, "P is not within range");

        const P_MINUS_ONE: [u32; 8] = [
            0xfffffffe, 0xffffffff, 0xffffffff, 0x00000000, 0x00000000, 0x00000000, 0x00000001,
            0xffffffff,
        ];
        let valid: bool = unsafe { P256_check_range_p(P_MINUS_ONE.as_ptr()) };
        defmt::assert!(valid, "P - 1 is within range");
    }

    #[test]
    fn convert_endianness() {
        use p256_cortex_m4_sys::p256_convert_endianness;

        const INPUT: [u8; 32] = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B,
            0x1C, 0x1D, 0x1E, 0x1F,
        ];
        let mut output: [u32; 8] = [0; 8];
        unsafe {
            p256_convert_endianness(
                output.as_mut_ptr() as *mut _,
                INPUT.as_ptr() as *const _,
                INPUT.len() as u32,
            )
        };
        defmt::assert_eq!(
            output,
            [
                0x1C1D1E1F, 0x18191A1B, 0x14151617, 0x10111213, 0x0C0D0E0F, 0x08090A0B, 0x04050607,
                0x00010203
            ]
        );
    }

    const X: [u32; 8] = [
        0x00112233, 0x44556677, 0x8899AABB, 0xCCDDEEFF, 0x00112233, 0x44556677, 0x8899AABB,
        0xCCDDEEFF,
    ];
    const Y: [u32; 8] = [
        0x01234567, 0x89ABCDEF, 0x12345678, 0x9ABCDEF0, 0x01234567, 0x89ABCDEF, 0x12345678,
        0x9ABCDEF0,
    ];

    #[test]
    fn point_to_octet_string_uncompressed() {
        use p256_cortex_m4_sys::p256_point_to_octet_string_uncompressed;

        let mut out: [u8; 65] = [0; 65];
        unsafe { p256_point_to_octet_string_uncompressed(out.as_mut_ptr(), X.as_ptr(), Y.as_ptr()) }
        defmt::assert_eq!(
            out,
            [
                0x04, 0xCC, 0xDD, 0xEE, 0xFF, 0x88, 0x99, 0xAA, 0xBB, 0x44, 0x55, 0x66, 0x77, 0x00,
                0x11, 0x22, 0x33, 0xCC, 0xDD, 0xEE, 0xFF, 0x88, 0x99, 0xAA, 0xBB, 0x44, 0x55, 0x66,
                0x77, 0x00, 0x11, 0x22, 0x33, 0x9A, 0xBC, 0xDE, 0xF0, 0x12, 0x34, 0x56, 0x78, 0x89,
                0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67, 0x9A, 0xBC, 0xDE, 0xF0, 0x12, 0x34, 0x56,
                0x78, 0x89, 0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67
            ]
        );
    }

    #[test]
    fn point_to_octet_string_compressed() {
        use p256_cortex_m4_sys::p256_point_to_octet_string_compressed;

        let mut out: [u8; 65] = [0; 65];
        unsafe { p256_point_to_octet_string_compressed(out.as_mut_ptr(), X.as_ptr(), Y.as_ptr()) }
        defmt::assert_eq!(
            out,
            [
                0x03, 0xCC, 0xDD, 0xEE, 0xFF, 0x88, 0x99, 0xAA, 0xBB, 0x44, 0x55, 0x66, 0x77, 0x00,
                0x11, 0x22, 0x33, 0xCC, 0xDD, 0xEE, 0xFF, 0x88, 0x99, 0xAA, 0xBB, 0x44, 0x55, 0x66,
                0x77, 0x00, 0x11, 0x22, 0x33, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
            ]
        );
    }

    #[test]
    fn point_to_octet_string_hybrid() {
        use p256_cortex_m4_sys::p256_point_to_octet_string_hybrid;

        let mut out: [u8; 65] = [0; 65];
        unsafe { p256_point_to_octet_string_hybrid(out.as_mut_ptr(), X.as_ptr(), Y.as_ptr()) }
        defmt::assert_eq!(
            out,
            [
                0x07, 0xCC, 0xDD, 0xEE, 0xFF, 0x88, 0x99, 0xAA, 0xBB, 0x44, 0x55, 0x66, 0x77, 0x00,
                0x11, 0x22, 0x33, 0xCC, 0xDD, 0xEE, 0xFF, 0x88, 0x99, 0xAA, 0xBB, 0x44, 0x55, 0x66,
                0x77, 0x00, 0x11, 0x22, 0x33, 0x9A, 0xBC, 0xDE, 0xF0, 0x12, 0x34, 0x56, 0x78, 0x89,
                0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67, 0x9A, 0xBC, 0xDE, 0xF0, 0x12, 0x34, 0x56,
                0x78, 0x89, 0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67
            ]
        );
    }

    #[test]
    fn octet_string_to_point() {
        use p256_cortex_m4_sys::p256_octet_string_to_point;

        const DER: [u8; 65] = [
            0x04, 0x57, 0x63, 0x64, 0xFF, 0xC3, 0x07, 0xBC, 0x8E, 0x7C, 0x2A, 0xB0, 0xB4, 0x91,
            0x0B, 0xB6, 0x70, 0xAE, 0x47, 0x29, 0x62, 0xFC, 0x7B, 0xE6, 0x41, 0x41, 0xA1, 0xF5,
            0x65, 0x5F, 0x2C, 0xC8, 0x56, 0xAB, 0xB2, 0xB2, 0x25, 0x73, 0x5F, 0x32, 0x77, 0x5B,
            0xDD, 0x82, 0x45, 0x98, 0x96, 0xFD, 0x3A, 0x92, 0x8C, 0x04, 0x0F, 0xB1, 0x33, 0x87,
            0x8E, 0xE9, 0xAC, 0x79, 0xE1, 0x72, 0x9E, 0x92, 0xE3,
        ];

        let mut x: [u32; 8] = [0; 8];
        let mut y: [u32; 8] = [0; 8];

        let is_ok: bool = unsafe {
            p256_octet_string_to_point(
                x.as_mut_ptr(),
                y.as_mut_ptr(),
                DER.as_ptr(),
                DER.len() as u32,
            )
        };
        defmt::assert!(is_ok, "An error occured");
        defmt::assert_eq!(
            x,
            [
                0x5F2CC856, 0x41A1F565, 0xFC7BE641, 0xAE472962, 0x910BB670, 0x7C2AB0B4, 0xC307BC8E,
                0x576364FF
            ]
        );
        defmt::assert_eq!(
            y,
            [
                0x729E92E3, 0xE9AC79E1, 0xB133878E, 0x928C040F, 0x9896FD3A, 0x5BDD8245, 0x735F3277,
                0xABB2B225
            ]
        );
    }

    #[test]
    fn verify() {
        use p256_cortex_m4_sys::{p256_octet_string_to_point, p256_verify};

        let mut key: [u8; 65] = [0; 65];
        key[0] = 0x04;
        key[1..33].copy_from_slice(&into_bytes(CURVE_PT_X_SWAP));
        key[33..65].copy_from_slice(&into_bytes(CURVE_PT_Y_SWAP));

        let mut x: [u32; 8] = [0; 8];
        let mut y: [u32; 8] = [0; 8];

        let is_ok: bool = unsafe {
            p256_octet_string_to_point(
                x.as_mut_ptr(),
                y.as_mut_ptr(),
                key.as_ptr(),
                key.len() as u32,
            )
        };
        assert!(is_ok, "p256_octet_string_to_point");

        let authentic: bool = unsafe {
            p256_verify(
                x.as_ptr(),
                y.as_ptr(),
                HASH_SWAP.as_ptr() as *const u8,
                32,
                convert(R_SIGN_SWAP).as_ptr(),
                convert(S_SIGN_SWAP).as_ptr(),
            )
        };
        defmt::assert!(authentic);
    }

    #[test]
    fn sign() {
        use p256_cortex_m4_sys::{p256_convert_endianness, p256_sign, P256_check_range_n};

        let mut private_key: [u32; 8] = [0; 8];
        unsafe {
            p256_convert_endianness(
                private_key.as_mut_ptr() as *mut _,
                into_bytes(PRIVATE_KEY_SWAP).as_ptr() as *const _,
                32,
            )
        };
        defmt::assert!(unsafe { P256_check_range_n(private_key.as_ptr()) });

        let mut integer: [u32; 8] = [0; 8];
        unsafe {
            p256_convert_endianness(
                integer.as_mut_ptr() as *mut _,
                into_bytes(INTEGER_SWAP).as_ptr() as *const _,
                32,
            )
        };

        let mut r_sign: [u32; 8] = [0; 8];
        let mut s_sign: [u32; 8] = [0; 8];

        let is_ok: bool = unsafe {
            p256_sign(
                r_sign.as_mut_ptr(),
                s_sign.as_mut_ptr(),
                HASH_SWAP.as_ptr() as *const u8,
                32,
                private_key.as_ptr(),
                integer.as_ptr(),
            )
        };

        defmt::assert!(is_ok, "An error occured");
        defmt::println!("r_sign={:08X}", r_sign);
        defmt::println!("R_SIGN_SWAP={:08X}", R_SIGN_SWAP);
        defmt::println!("s_sign={:08X}", r_sign);
        defmt::println!("S_SIGN_SWAP={:08X}", S_SIGN_SWAP);
        defmt::assert_eq!(convert(r_sign), R_SIGN_SWAP);
        defmt::assert_eq!(convert(s_sign), S_SIGN_SWAP);
    }
}
