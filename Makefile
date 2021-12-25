run: boot
	qemu-system-x86_64 -enable-kvm bin/stage0

boot: bootloader/src/stage0.S 
	@@cd bootloader
	@$(MAKE) -C bootloader
	@@cd ../
	@@cp bootloader/target/x86-64-bootloader/debug/bootloader bin/
	@python3 flatten_elf.py

debug: boot
	qemu-system-x86_64 -enable-kvm bin/stage0 -S -s &
	sudo gdb --command ./debug.gdb
	killall qemu-system-x86

objdump: clean boot
	objdump -M intel -b binary -d bin/stage0 -m i8086 --adjust-vma=0x7c00

clean:
	@@rm bin/stage0 || true
	@@rm bin/bootloader || true