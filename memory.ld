MEMORY{
    BOOTLOADER(rx) : ORIGIN = 0x10000000,
    LENGTH = 256

    RAM(rwx) : ORIGIN = 0x20000000,
    LENGTH = 2K

    FLASH(rx) : ORIGIN = 0x10000100,
    LENGTH = 2M - 256
}

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
    } > FLASH

    /DISCARD/ : {
        *(.ARM.*)
    }
}
