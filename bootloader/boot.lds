ENTRY(entry)

SECTIONS {
    . = 0x7c00;

    .bootloader :
    {
        *(.text .text.*)
        *(.rodata .rodata.*)
    }
}