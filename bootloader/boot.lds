ENTRY(_boot)

SECTIONS {
    . = 0x7c00;

    .bootloader :
    {
        /* stage 0 */
        *(.stage0)
    }
}