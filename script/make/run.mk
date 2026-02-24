QEMU = qemu-system-riscv64
BIOS = default

GDB = /data/riscv64-unknown-linux-gnu/bin/riscv64-unknown-linux-gnu-gdb


run-qemu: $(rust_bin)
	$(QEMU) -m 2G -machine virt -bios $(BIOS) -kernel $(rust_bin) -nographic

run-qemu-debug: $(rust_bin)
	$(QEMU) -m 2G -machine virt -bios $(BIOS) -kernel $(rust_bin) -nographic -s -S

gdb:
	$(GDB) --ex "target remote localhost:1234" $(rust_elf)
