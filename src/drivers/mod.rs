#![allow(unused)]

pub mod rtt;
pub mod led;
pub mod i2c;
pub mod timer;

pub trait KerlDriver {
    fn init() -> Self;
}
