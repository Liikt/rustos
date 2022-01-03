main:
	cargo run

run: main
	qemu-system-x86_64 -enable-kvm \
		-netdev user,net=192.168.22.0/24,id=net0,tftp=$(PWD)/bin/,bootfile=rustos.boot \
		-device virtio-net-pci,netdev=net0

debug: main
	qemu-system-x86_64 -enable-kvm \
		-netdev user,net=192.168.22.0/24,id=net0,tftp=$(PWD)/bin/,bootfile=rustos.boot \
		-device virtio-net-pci,netdev=net0 \
		-s -S &
	sudo gdb --command ./debug.gdb
	killall qemu-system-x86

objdump: clean main
	objdump -M intel -b binary -d bin/rustos.boot -m i8086 --adjust-vma=0x7c00

clean:
	@@rm bin/* || true