[org 0x7c00]
[bits 16]

jmp entry

entry:
%ifdef dbg
loop:
    jmp loop
%endif

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
    or  eax, (1 << 0)
    mov cr0, eax

    ; Jump into protected mode
    jmp 0x08:pm_entry

[bits 32]

pm_entry:
    ; Set up the 32-bit segment registers
    mov ax, 0x10
    mov es, ax
    mov ds, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    ; Call into rust! `entry_point` is a macro that gets defined in the 
    ; `main.rs` in the main src folder.
    call entry_point

align 8
gdt_base:
	dq 0x0000000000000000 ; 0x0000 | Null descriptor
	dq 0x00cf9a000000ffff ; 0x0008 | 32-bit, present, code, base 0
	dq 0x00cf92000000ffff ; 0x0010 | 32-bit, present, data, base 0

gdt:
	dw (gdt - gdt_base) - 1
	dd gdt_base

times 510-($-$$) db 0
dw 0xaa55

times (0x8000-0x7c00)-($-$$) db 0
incbin "bin/boot.flat"
