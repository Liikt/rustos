run: boot
	qemu-system-x86_64 -enable-kvm stage0

boot: bootloader/src/stage0.S 
	cd bootloader
	$(MAKE) -C bootloader
	cd ../
	cp bootloader/target/x86-64-bootloader/debug/bootloader stage0


debug: boot
	qemu-system-x86_64 -enable-kvm stage0 -S -s &

objdump: clean boot
	objdump -M intel -b binary -d ./stage0 -m i8086 --adjust-vma=0x7c00

clean:
	@@rm stage0 || true