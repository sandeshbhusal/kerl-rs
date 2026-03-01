use core::arch::asm;

/// Enables all interrupts.
pub(crate) fn enable_all_interrupts() {
    unsafe { asm!("cpsie i") }
}

/// Disables all interrupts.
pub(crate) fn disable_all_interrupts() {
    unsafe { asm!("cpsid i") }
}

#[link_section = ".vector_ext"]
#[no_mangle]
pub static IRQ_TABLE: [u32; 0] = [];
