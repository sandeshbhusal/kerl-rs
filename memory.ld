MEMORY{
    BOOTLOADER(rx) : ORIGIN = 0x10000000,
    LENGTH = 256

    RAM(rwx) : ORIGIN = 0x20000000,
    LENGTH = 2K

    FLASH(rx) : ORIGIN = 0x10000100,
    LENGTH = 2M - 256
}

SECTIONS {
    .boot2 : {
        KEEP(*(.boot2*))
    } > RAM AT > BOOTLOADER

    .text : {
        KEEP(*(.text*))
    } > FLASH

    /DISCARD/ : {
        *(.ARM.*)
    }
}
