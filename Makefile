PWD := $(shell pwd)
LINKER_SCRIPT := $(PWD)/linker/kernel.lds

TARGET := riscv64gc-unknown-none-elf
TARGET_DIR := $(PWD)/target

ARCH ?= riscv64
MODE ?= release


OBJCOPY = rust-objcopy --binary-architecture $(TARGET)



.DEFAULT_GOAL := all

include script/make/compile.mk
include script/make/run.mk


all: kernel
	@echo "Building project..."


clean:
	@echo "Cleaning project..."
	cargo clean


.PHONY: all