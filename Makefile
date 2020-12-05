arch ?= x86_64
build_type ?= debug
kernel := build/kernel-$(arch).bin
iso := build/serex-$(arch).iso

linker_script := src/arch/$(arch)/unique/linker.ld
grub_cfg := src/arch/$(arch)/unique/grub.cfg
assembly_source_files := $(wildcard src/arch/$(arch)/unique/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/%.asm, build/arch/$(arch)/%.o, $(assembly_source_files))

target ?= $(arch)
serex: target/$(target)/$(build_type)/libserex.a

.PHONY: all clean run iso kernel doc disk

all: $(kernel)
release: $(release_kernel)
clean:
	rm -r build
	cargo clean
run: $(iso)
	qemu-system-x86_64 -cdrom $(iso) -boot menu=on -vga std -s -serial file:serial.log

iso: $(iso)
	@echo "ISO made."

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/iso/boot/grub
	cp $(kernel) build/iso/boot/kernel.bin
	cp $(grub_cfg) build/iso/boot/grub
	grub-mkrescue -o $(iso) build/iso --modules="part_msdos boot cat configfile echo elf fat gfxmenu gfxterm iso9660 ls msdospart multiboot2 normal search_fs_file search video_fb video" #2> /dev/null
	@rm -r build/iso

$(kernel): kernel $(serex) $(assembly_object_files) $(linker_script)
	ld -n --gc-sections -T $(linker_script) -o $(kernel) $(assembly_object_files) target/$(target)/$(build_type)/libserex.a

kernel:
	@RUST_TARGET_PATH=$(32shell pwd) cargo build --target $(arch).json

build/arch/$(arch)/unique/%.o: src/arch/$(arch)/unique/%.asm
	@mkdir -p $(shell dirname $@)
	nasm -felf64 $< -o $@
