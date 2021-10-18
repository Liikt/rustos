run: boot
	qemu-system-x86_64 -enable-kvm stage0

boot: bootloader/src/stage0.S 
	nasm -f bin bootloader/src/stage0.S
	mv bootloader/src/stage0 .

debug: boot
	qemu-system-x86_64 -enable-kvm stage0 -S -s &

objdump: boot
	objdump -M intel -b binary -D ./stage0 -m i8086 --adjust-vma=0x7c00
