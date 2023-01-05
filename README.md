# rvvm [WIP]

Safe Rust bindings to the RVVM's public API.

Provides the Rust-idiomatic interface to the [RVVM](https://github.com/lekkit/rvvm) public API.

# Implemented

- [x] Virtual machine creation
- [x] RAM read/writes
- [x] Sound (in terms of safety) and type-safe api for devices
  - [ ] Complete API (needs rechecking)
- [x] Flattened device tree library bindings
- [x] Access to the virtual machine's FDT
- [x] `pause`/`start`/`powered_on` APIs

- [x] loading kernel/bootrom/dtb
- [x] dumping dtb to file
- [ ] Run virtual machine's event loop
- [ ] PLIC/i2c
- [ ] Userland API
- [ ] `DMA`
- [ ] Test `UART` device
- [ ] rvvm-sys features (like fdt, jit or smth)

Possibly I've lost something, so the list will be supplemented/updated. 
