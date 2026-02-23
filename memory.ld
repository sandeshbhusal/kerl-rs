MEMORY{
    BOOTLOADER(rx) : ORIGIN = 0x10000000,
    LENGTH = 256

    RAM(rwx) : ORIGIN = 0x20000000,
    LENGTH = 264K

    FLASH(rx) : ORIGIN = 0x10000100,
    LENGTH = 2M - 256
}

start_ram = ORIGIN(RAM);
estack = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS {
    .boot2 : {
        KEEP(*(.boot2*))
    } > RAM AT > BOOTLOADER

    .vector_table : {
        KEEP(*(.vector_table))
    } > FLASH

    .text : {
        KEEP(*(.text*))
        KEEP(*(.rodata*))
    } > FLASH

    .bss : {
        _bss_start = .;
        KEEP(*(.bss*))
        . = ALIGN(4);
        _bss_end = .;
    } > RAM

    .data : {
        _data_start = .;
        KEEP(*(.data*))
        . = ALIGN(4);
        _data_end = .;
    } > RAM AT > FLASH

    _data_loadaddr = LOADADDR(.data);

    /DISCARD/ : {
        *(.ARM.*)
    }
}
