pub mod led;

pub trait KerlDriver {
    fn init() -> Self;
}
