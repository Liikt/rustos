ENTRY(entry)

SECTIONS {
    . = 0x7c00;

    .bootloader :
    {
        /* stage 0 */
        *(.stage0)
        *(.text .text.*)
        *(.rodata .rodata.*)
    }
}