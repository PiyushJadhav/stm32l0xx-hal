//! Interface to the USB peripheral
//!
//! As USB seems to work mostly the same across all STM32 MCUs, there is a
//! single crate that supports USB for these MCUs:
//! [`stm32-usbd`](https://crates.io/crates/stm32-usbd)
//!
//! This module implements some bits needed for `stm32-usbd` to work and exports
//! `UsbBus` from `stm32-usbd`.
//!
//! Please check out the USB examples in the `examples/` directory to see how it
//! fits together.

use crate::stm32::{RCC, USB};
use stm32_usbd::UsbPeripheral;

use crate::gpio::gpioa::{PA11, PA12};
use crate::gpio::{Floating, Input};
pub use stm32_usbd::UsbBus;

pub struct Peripheral {
    pub usb: USB,
    pub pin_dm: PA11<Input<Floating>>,
    pub pin_dp: PA12<Input<Floating>>,
}

unsafe impl Sync for Peripheral {}

unsafe impl UsbPeripheral for Peripheral {
    const REGISTERS: *const () = USB::ptr() as *const ();
    const DP_PULL_UP_FEATURE: bool = true;
    const EP_MEMORY: *const () = 0x4000_6000 as _;
    const EP_MEMORY_SIZE: usize = 1024;

    fn enable() {
        let rcc = unsafe { (&*RCC::ptr()) };

        cortex_m::interrupt::free(|_| {
            // Enable USB peripheral
            rcc.apb1enr.modify(|_, w| w.usben().set_bit());

            // Reset USB peripheral
            rcc.apb1rstr.modify(|_, w| w.usbrst().set_bit());
            rcc.apb1rstr.modify(|_, w| w.usbrst().clear_bit());
        });
    }

    fn startup_delay() {
        // There is a chip specific startup delay. For STM32F103xx it's 1µs and this should wait for
        // at least that long.
        cortex_m::asm::delay(72);
    }
}

pub type UsbBusType = UsbBus<Peripheral>;
