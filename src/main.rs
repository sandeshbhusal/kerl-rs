#![no_std]
#![no_main]
#![allow(unused_variables)]

mod boot2;
mod reset;

mod conf;
mod drivers;
mod log;

use ::log::{trace, info};
use crate::log::init_logger;
use crate::reset::sysinit;
use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[no_mangle]
pub extern "C" fn _init() -> ! {
    unsafe {
        sysinit();
        init_logger();
    };
    info!("");
    info!("Booting KERL");
    info!("-------------------");
    info!("So far the following things have been done:");
    info!("1. Vector table init");
    info!("2. Initialize memory");
    info!("3. RTT protocol init (we are here)");

    loop {
    }
}
