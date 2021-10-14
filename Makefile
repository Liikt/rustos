run: boot
	qemu-system-x86_64 -enable-kvm stage0

boot: bootloader/src/stage0.S 
	nasm -f bin bootloader/src/stage0.S
	mv bootloader/src/stage0 .