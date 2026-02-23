pub mod rtt;
pub mod led;
pub mod timer;

pub trait KerlDriver {
    fn init() -> Self;
}
