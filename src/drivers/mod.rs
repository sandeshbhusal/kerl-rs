pub mod rtt;
pub mod led;

pub trait KerlDriver {
    fn init() -> Self;
}
