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

        const ZERO: [u32; 8] = [0; 8];
        let valid: bool = unsafe { P256_check_range_n(ZERO.as_ptr()) };
        defmt::assert!(!valid, "zero is not in range");

        const ONE: [u32; 8] = [0, 0, 0, 0, 0, 0, 0, 1];
        let valid: bool = unsafe { P256_check_range_n(ONE.as_ptr()) };
        defmt::assert!(valid, "1 is in range");

        // n = 2**256 - 2**224 + 2**192 - int("4319055258e8617b0c46353d039cdaaf", 16)
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
        defmt::assert!(valid, "N_MINUS_ONE is within range");
    }
}
