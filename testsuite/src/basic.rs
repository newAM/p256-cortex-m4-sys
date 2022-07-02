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
        assert_eq!(
            output,
            [
                0x1C1D1E1F, 0x18191A1B, 0x14151617, 0x10111213, 0x0C0D0E0F, 0x08090A0B, 0x04050607,
                0x00010203
            ]
        );
    }
}
