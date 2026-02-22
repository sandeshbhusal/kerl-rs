#![no_std]
#![no_main]
#![allow(unused_variables)]

mod reset;
mod boot2;

use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[no_mangle]
pub extern "C" fn _init() -> ! {
    loop {
        core::hint::spin_loop();
    }
}
