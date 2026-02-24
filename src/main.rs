#![no_std]
#![no_main]
#![allow(unused_variables)]

mod boot2;
mod reset;

mod clock;
mod conf;
mod drivers;
mod log;

use ::log::{info, error};
use crate::log::init_logger;
use crate::reset::meminit;
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
        meminit();
        init_logger();
    };
    info!("");
    info!("Booting KERL");
    info!("-------------------");
    if let Err(e) = clock::init_clocks() {
        error!("Failed to initialize clocks: {:?}", e);
    }

    loop {
    }
}
