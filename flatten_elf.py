from struct import unpack
from sys import exit

with open("bin/bootloader", "rb") as f:
    data = f.read()

phoff = unpack("<Q", data[0x20:0x28])[0]
psize = unpack("<H", data[0x36:0x38])[0]
pnum  = unpack("<H", data[0x38:0x3a])[0]

print(f"Offset = {phoff}")
print(f"Size   = {psize}")
print(f"Num    = {pnum}")

for x in range(pnum):
    paddr = unpack("<Q", data[phoff+x*psize+0x18:phoff+x*psize+0x20])[0]
    print(f"{paddr:x}")
    if paddr == 0x7c00:
        offset = unpack("<Q", data[phoff+x*psize+0x8:phoff+x*psize+0x10])[0]
        size = unpack("<Q", data[phoff+x*psize+0x20:phoff+x*psize+0x28])[0]
        print(f"Paddr  = {paddr:x}")
        print(f"Size   = {size:x}")

        with open("bin/stage0", "wb") as f:
            f.write(data[offset:offset+size])
        exit(0)

print("No correct section was found")
exit(1)