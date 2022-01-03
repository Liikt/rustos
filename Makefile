run:
	cargo run
	qemu-system-x86_64 -enable-kvm \
		-netdev user,net=192.168.22.0/24,id=net0,tftp=$(PWD)/bin/,bootfile=rustos.boot \
		-device virtio-net-pci,netdev=net0

debug:
	cargo run debug
	qemu-system-x86_64 -enable-kvm \
		-netdev user,net=192.168.22.0/24,id=net0,tftp=$(PWD)/bin/,bootfile=rustos.boot \
		-device virtio-net-pci,netdev=net0 \
		-s -S &
	sudo gdb --command ./debug.gdb
	killall qemu-system-x86

objdump: clean
	cargo run
	objdump -M intel -b binary -D bin/rustos.boot -m i386 --adjust-vma=0x7c00

clean:
	@@rm bin/* || true