build_args := \
  -Z unstable-options \
  --target $(TARGET) \
  --target-dir $(TARGET_DIR) \
  $(build_args-$(MODE)) \


BUILD_DIR = $(PWD)/kernel/arch/riscv64/

rust_package := $(shell cat $(BUILD_DIR)/Cargo.toml | sed -n 's/^name = "\([a-z0-9A-Z_\-]*\)"/\1/p')
rust_elf = $(TARGET_DIR)/$(TARGET)/$(MODE)/$(rust_package)

rust_bin = $(TARGET_DIR)/$(TARGET)/$(MODE)/$(rust_package).bin

RUST_LINK_ARGS = -C link-arg=-T$(LINKER_SCRIPT) -C link-arg=-znostart-stop-gc

#  -C link-arg=-no-pie 

RUSTFLAGS = -A unsafe_op_in_unsafe_fn
RUSTFLAGS += $(RUST_LINK_ARGS)

export RUSTFLAGS

$(rust_bin): $(rust_elf)
	$(OBJCOPY) --strip-all -O binary $< $@


$(rust_elf): 
	cargo -C $(BUILD_DIR) build $(build_args)


kernel: $(rust_bin)