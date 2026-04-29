# VaachakOS Bootstrap Phase 4.1 — X4 Linker Script Fix

## Purpose

Phase 4 converted `target-xteink-x4` into a minimal embedded ESP32-C3 boot-smoke target, but the first embedded build failed during linking with undefined interrupt/peripheral symbols such as:

- `WIFI_MAC`
- `BT_MAC`
- `I2C_MASTER`
- `GPIO`
- `SPI1`
- `SPI2`

Those symbols are provided by the ESP HAL linker script set. The target-specific Cargo config must pass the ESP HAL linker script entry point to the linker.

## Fix

Add this target rustflag:

```toml
"-C", "link-arg=-Tlinkall.x"
```

The resulting target config is:

```toml
[target.riscv32imc-unknown-none-elf]
runner = "espflash flash --monitor --chip esp32c3"
rustflags = [
  "-C", "link-arg=-Tlinkall.x",
  "-C", "force-frame-pointers",
]
```

## Scope

This phase does not change app/runtime behavior. It only fixes embedded linking for the X4 boot-smoke target.

## Validate

```bash
cargo fmt --all
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings

. "$HOME/export-esp.sh"
cargo build -p target-xteink-x4 \
  --release \
  --target riscv32imc-unknown-none-elf
```

If build succeeds, flash with:

```bash
cargo run -p target-xteink-x4 \
  --release \
  --target riscv32imc-unknown-none-elf
```
