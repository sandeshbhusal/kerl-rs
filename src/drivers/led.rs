//! Very very basic LED driver for the rp2040 raspipico board

use rp2040_pac::{IoBank0, PadsBank0, Resets, Sio};

use crate::drivers::KerlDriver;

const GPIO_PIN: u32 = 25;

pub struct Led {
    sio: Sio,
}

impl KerlDriver for Led {
    fn init() -> Self {
        let resets = unsafe { Resets::steal() };
        let sio = unsafe { Sio::steal() };
        let pad = unsafe { PadsBank0::steal() };
        let io = unsafe { IoBank0::steal() };

        resets
            .reset()
            .modify(|_, w| w.pads_bank0().clear_bit().io_bank0().clear_bit());

        while !resets.reset_done().read().pads_bank0().bit_is_set() {}
        while !resets.reset_done().read().io_bank0().bit_is_set() {}

        // Configure the pad
        pad.gpio25().write(|w| {
            w.ie()
                .clear_bit()
                .od()
                .clear_bit()
                .pde()
                .clear_bit()
                .pue()
                .clear_bit()
                .drive()
                ._4m_a()
        });

        // Configure the pin to SIO mode
        io.gpio25_ctrl().write(|w| w.funcsel().sio_25());

        // Enable output on GPIO25
        sio.gpio_oe_set()
            .write(|w| unsafe { w.bits(1 << GPIO_PIN) });

        Self { sio }
    }
}

impl Led {
    /// Toggle the LED
    pub fn toggle(&self) {
        self.sio
            .gpio_out_xor()
            .write(|w| unsafe { w.bits(1 << GPIO_PIN) });
    }
}
