#![no_std]
#![no_main]
#![allow(unused_variables)]

mod reset;
mod boot2;

mod drivers;

use core::panic::PanicInfo;

use crate::drivers::led;
use crate::drivers::KerlDriver;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[no_mangle]
pub extern "C" fn _init() -> ! {
    let sysboard_led = led::Led::init();

    loop {
        sysboard_led.toggle();
        for _ in 0..100_000_000 {}
    }
}
