# rv-plic

Rust support for RISC-V Platform Level Interrupt Controller

Based on [HUST-OS/plic](https://github.com/HUST-OS/plic)

REF: [RISC-V Platform-Level Interrupt Controller Specification](https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc)

## Usage

Add to `Cargo.toml`:

```toml
rv-plic = { git = "https://github.com/duskmoon314/rv-plic" }
```

Set qemu virt machine uart:

```rust
// context 1: hart 0, S mode
type PLIC = rv_plic::PLIC<PLIC_BASE, 3>;
PLIC::set_threshold(1, Priority::any());
PLIC::enable(1, 10);
PLIC::set_priority(10, Priority::lowest());
```
