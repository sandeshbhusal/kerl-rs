//! Reset handling and vtor initialization code

use core::ptr::{read_volatile, write_volatile};

#[repr(C)]
pub union Vector {
    handler: unsafe extern "C" fn(),
    pointer: *const u32,
}

// Safety: The vector table is immutable static data.
unsafe impl Sync for Vector {}

extern "C" {
    // Symbols from the linker script
    static estack: u32;
    static mut _data_start: u32;
    static mut _data_end: u32;
    static mut _data_loadaddr: u32;
    static mut _bss_start: u32;
    static mut _bss_end: u32;

    fn _init();
}

#[link_section = ".vector_table"]
#[used]
#[no_mangle]
pub static VECTOR_TABLE: [Vector; 2] = [
    Vector {
        pointer: unsafe { &estack },
    },
    Vector { handler: _init },
];

/// Initialize the system
pub unsafe fn meminit() {
    // Zero out the BSS, copy the data to the RAM
    let mut data_start_ptr = &raw mut _data_start;
    let mut data_load_ptr = &raw mut _data_loadaddr;
    let data_end_ptr = &raw mut _data_end;

    while data_start_ptr < data_end_ptr {
        write_volatile(data_start_ptr, read_volatile(data_load_ptr));
        data_load_ptr = data_load_ptr.add(1);
        data_start_ptr = data_start_ptr.add(1);
    }

    let mut bss_start = &raw mut _bss_start;
    let bss_end = &raw mut _bss_end;
    while bss_start < bss_end {
        core::ptr::write_volatile(bss_start, 0);
        bss_start = bss_start.add(1);
    }
}
