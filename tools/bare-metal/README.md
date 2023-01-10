Tools to build bare metal RISC-V executables used for testing purposes.

Build [`hello.c`](./hello.c):
```bash
nix-shell --pure --arg dev false --run 'make out/hello.h'
```

Run the resulting binary with `qemu` or `rvvm`:
```bash
qemu-system-riscv64 -nographic -machine virt -bios out/hello.elf
rvvm -nogui out/hello.bin
```
