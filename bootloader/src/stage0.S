[org 0x7c00]
[bits 16]

jmp boot

boot:
    ; Clear interupts and the direction flag
    cli
    cld

    ; Set the A20 line
    in al, 0x92
    or al, 2
    out 0x92, al

	; Clear DS
    xor ax, ax
    mov ds, ax

    ; Load the GDT
    lgdt [ds:gdt]

    ; Enable protected mode by setting the PE (Protection Enable) bit in CR0
    mov eax, cr0 
    or al, 1
    mov cr0, eax

    jmp 0x18:pm_entry

[bits 32]
pm_entry:
    jmp $

times 510-($-$$) db 0
dw 0xaa55

align 8
gdt_base:
	dq 0x0000000000000000 ; 0x0000 | Null descriptor
	dq 0x00009a007c00ffff ; 0x0008 | 16-bit, present, code, base 0x7c00
	dq 0x000092000000ffff ; 0x0010 | 16-bit, present, data, base 0
	dq 0x00cf9a000000ffff ; 0x0018 | 32-bit, present, code, base 0
	dq 0x00cf92000000ffff ; 0x0020 | 32-bit, present, data, base 0
	dq 0x00209a0000000000 ; 0x0028 | 64-bit, present, code, base 0
	dq 0x0000920000000000 ; 0x0030 | 64-bit, present, data, base 0

gdt:
	dw (gdt - gdt_base) - 1
	dd gdt_base

boot_end: