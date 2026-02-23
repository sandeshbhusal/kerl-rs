#![no_std]
#![no_main]
#![allow(unused_variables)]

mod boot2;
mod reset;

mod conf;
mod drivers;

use crate::drivers::rtt::Rtt;
use crate::reset::sysinit;
use core::fmt::Write;
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
    };

    let mut rtt = Rtt;
    write!(rtt, "KERL: Welcome to RTT!\n").unwrap();
    loop {
    }
}
