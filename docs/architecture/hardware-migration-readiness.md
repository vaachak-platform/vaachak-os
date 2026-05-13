# Hardware Runtime Readiness

## Current status

The repository is in a cleaned X4 runtime baseline. Active X4 work should be described through Vaachak-owned target paths under:

```text
target-xteink-x4/src/vaachak_x4/**
```

`vendor/pulp-os` may remain as scoped compatibility/reference material, but new Vaachak OS functionality should not be added there.

## Readiness gate before hardware-sensitive changes

Run from repository root:

```bash
cargo fmt --all
./scripts/check_repo_hygiene.sh
cargo test -p vaachak-core --all-targets
cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
```

## Device smoke baseline

Before changing hardware-sensitive behavior, verify:

- device boots repeatedly
- Home/category dashboard appears
- Files/Library opens
- SD card lists files
- TXT opens
- EPUB/EPU smoke path opens
- prepared cache path still reports clean success/failure status
- Back navigation works
- Settings persists expected values
- Wi-Fi Transfer remains usable
- Date & Time screen remains cancellable and recoverable
- sleep-image mode still works

## What not to do

- Do not combine input, SPI, display, and SD/FAT changes in one unvalidated change.
- Do not delete `vendor/pulp-os` without a separate dependency-removal audit.
- Do not add feature work that destabilizes the reader baseline.
- Do not reintroduce root patch zips/folders or one-off validators.
