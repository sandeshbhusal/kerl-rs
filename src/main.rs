#![no_std]
#![no_main]
#![allow(unused_variables)]

use core::panic::PanicInfo;

use log::{error, info};

use crate::{boot::init::meminit, hw::clock, kernel::log::init_logger};

mod conf;
mod boot;
mod hw;
mod drivers;
mod kernel;

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
