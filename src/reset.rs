//! Reset handling and vtor initialization code

#[repr(C)]
pub union Vector {
    handler: unsafe extern "C" fn(),
    pointer: *const u32,
}

// Safety: The vector table is immutable static data.
unsafe impl Sync for Vector {}

extern "C" {
    static estack: u32;
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

/// Initialize the system features
pub fn sysinit() {
}
