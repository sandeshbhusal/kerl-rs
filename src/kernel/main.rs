//! Entrypoint for the kernel.

use log::info;

use crate::kernel::log::LOGGING_BACKEND;

pub fn main() {
    info!("Welcome to Kerl");
    info!("Logger is using the {} backend", LOGGING_BACKEND);
}
