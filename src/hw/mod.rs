//! The intended use of this module is to provide
//! some sort of a HAL layer to the system services (not periphs)
//! like clocks and gpio that is used everywhere.
//!
//! The files in this module provide essential components
//! for the peripherals and drivers, for e.g. the LED driver can
//! use the gpio and clock modules to create a blinking output on a
//! GPIO pin.

pub mod clock;
pub mod cpuid;
pub mod gpio;
pub mod interrupts;
pub mod pll;
pub mod rosc;
pub mod xosc;
