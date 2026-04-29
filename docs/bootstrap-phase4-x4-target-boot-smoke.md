# VaachakOS Bootstrap Phase 4 — X4 Target Boot Smoke

## Purpose

Phase 4 makes `target-xteink-x4` a real ESP32-C3 firmware target while keeping the scope intentionally small.

This phase is not a reader migration and does not port the working display, SD, input task, or reader runtime from `x4-reader-os-rs`.

## What this phase proves

- `target-xteink-x4` can build for `riscv32imc-unknown-none-elf`.
- The ESP-IDF app descriptor is present for flashing compatibility.
- `no_std` / `no_main` firmware entry is wired for the X4 target.
- Serial logging works through `esp-println`.
- The Phase 3 input/power/display/storage model crates link into firmware.
- Host `cargo check/test/clippy --workspace` remains usable.

## What this phase intentionally does not do

- No real SSD1677 display initialization.
- No real SD/FAT mount.
- No real ADC sampling.
- No real Reader/Home/Files runtime.
- No AppManager/Kernel migration.
- No power sleep entry.

## Build and flash

Host checks:

```bash
cargo fmt --all
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
```

Embedded build:

```bash
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
```

Flash and monitor:

```bash
cargo run -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
```

The `.cargo/config.toml` runner maps that command to:

```bash
espflash flash --monitor --chip esp32c3
```

## Expected serial markers

```text
VaachakOS X4 boot smoke starting
phase=bootstrap-phase4-x4-target-boot-smoke
target=esp32c3 riscv32imc-unknown-none-elf
heap=16K boot-smoke only
display logical=480x800 native=800x480 rot=Deg270 strip_rows=40
bus shared_sd_epd=true probe=400kHz runtime=20MHz
storage state=Probed card_bytes=Some(15634268160)
power battery_mv=4100 pct=...
VaachakOS X4 boot smoke complete
```

## Next phase

Recommended next phase:

`VaachakOS Bootstrap Phase 5 — X4 Display HAL Boot Console Smoke`

Phase 5 should still avoid Reader migration. It should only prove real display init and a minimal boot-console render path.
