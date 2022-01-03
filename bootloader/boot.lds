ENTRY(entry)

SECTIONS {
    . = 0x8000;

    .bootloader :
    {
        *(.text .text.*)
        *(.rodata .rodata.*)
    }
}