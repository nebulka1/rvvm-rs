CROSS_COMPILE        := riscv64-none-elf-
CC                   := $(CROSS_COMPILE)gcc
OBJCOPY              := $(CROSS_COMPILE)objcopy
NIX_HARDENING_ENABLE := # disable nix quirks for gcc

.DELETE_ON_ERROR:
.PRECIOUS: out/%.elf out/%.bin

.PHONY: all
all: out/hello.h

out/%.elf: %.c entry.S linker.ld
	mkdir -p out
	$(CC) -o $@ -T linker.ld $< entry.S \
		-ffreestanding -nostdlib -mcmodel=medany -Qn -O3

out/%.bin: out/%.elf
	$(OBJCOPY) --only-section=.text -O binary $< $@

out/%.h: out/%.bin
	xxd -i -a < $< > $@

.PHONY: clean
clean:
	rm -rf out

.PHONY: format
format:
	clang-format -i *.c
	nixpkgs-fmt *.nix
