# Set the correct architecture for gdb
set architecture i386:x86-64

# This is so that we can jump over the inf loop breakpoint
define cont
set $pc=$pc+2
c
end

# Connect to qemu
gef-remote --qemu-mode localhost:1234

# Set a breakpoint at the start of the bootloader and continue qemu
br *0x7c04
c