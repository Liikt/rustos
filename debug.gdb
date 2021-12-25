set architecture i386:x86-64

define cont
set $pc=$pc+2
c
end

gef-remote --qemu-mode localhost:1234

br *0x7c05
c