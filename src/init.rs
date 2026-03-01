#![no_std]
#![no_main]
#![allow(unused_variables)]

mod boot;
mod kernel;

// TODO: Remove these mod decls later.
mod conf;
mod drivers;
mod hw;

use core::panic::PanicInfo;

use crate::kernel::log::init_logger;
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

/// _init() entrypoint with C ABI.
///
/// This sole task of this function is to init memory,
/// copy DATA, clear BSS and call kernel::main() to enter
/// a pure-Rust kernel.
#[no_mangle]
pub extern "C" fn _init() -> ! {
    unsafe {
        boot::init::meminit();
        init_logger();
    }

    kernel::main();

    loop {
        core::hint::spin_loop();
    }
}
