run: boot
	qemu-system-x86_64 -enable-kvm bootloader/boot

boot: bootloader/boot.S 
	nasm -f bin bootloader/boot.S