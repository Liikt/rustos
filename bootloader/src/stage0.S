.section .stage0, "awx"
.global _boot
.code16

jmp _boot

_boot:
    # Clear interupts and the direction flag
    cli
    cld

    # Set the A20 line
    in al, 0x92
    or al, 2
    out 0x92, al

	# Clear DS
    xor ax, ax
    mov ds, ax

    # Load the GDT
    lgdt [gdt]

    # Enable protected mode by setting the PE (Protection Enable) bit in CR0
    mov eax, cr0 
    or  eax, (1 << 0)
    mov cr0, eax

    mov sp, 0x7c00

    jmp pm_entry

.code32

pm_entry:
    mov bx, 0x10
    mov es, bx
    mov ds, bx
    mov fs, bx
    mov gs, bx
    mov ss, bx

    push ds

loop:
    jmp loop

    and al, 0xfe    # clear protected mode bit
    mov cr0, eax

    call entry

.align 8
gdt_base:
	.quad 0x0000000000000000 # 0x0000 | Null descriptor
	.quad 0x00cf9a000000ffff # 0x0008 | 32-bit, present, code, base 0
	.quad 0x00cf92000000ffff # 0x0010 | 32-bit, present, data, base 0

gdt:
	.word (gdt - gdt_base) - 1
	.long gdt_base
