use rp2040_pac::{I2c0, IoBank0, PadsBank0, Resets, Sio, sio};

use crate::drivers::KerlDriver;

pub struct i2c;

enum I2CAddress {
    _8_Bit(u8),
    _10_Bit(u16)
}

impl KerlDriver for i2c {
    fn init() -> Self {
        // Always take GPIO Pins .. and ..
        let sio = unsafe { Sio::steal() };
        let pads = unsafe { PadsBank0::steal() };
        let io_bank0 = unsafe { IoBank0::steal() };
        let resets = unsafe { Resets::steal()};

        io_bank0.gpio0_ctrl().modify(|_, w| w.funcsel().i2c0_sda());
        io_bank0.gpio1_ctrl().modify(|_, w| w.funcsel().i2c0_scl());

        pads.gpio0().write(|w| w.ie().set_bit().od().clear_bit());
        pads.gpio1().write(|w| w.ie().set_bit().od().clear_bit());

        resets.reset().write(|w| w.i2c0().clear_bit());
        while (resets.reset_done().read().i2c0().bit() != true) {}

        Self
    }
}

impl i2c {
    fn set_master_mode() {
       todo!()
    }

    fn find_device(address: u16) {

    }
}
