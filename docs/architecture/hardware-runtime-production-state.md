# Vaachak OS Hardware Runtime Production State

Status: production cleanup baseline.

The X4 hardware runtime should be described in terms of production modules and product behavior, not in terms of migration slices, acceptance markers, or temporary deliverable validators.

## Production ownership

Vaachak owns the X4 hardware-runtime architecture and keeps hardware behavior behind production module boundaries:

- SPI bus policy and transport boundary
- SSD1677 display policy and refresh lifecycle
- SD/MMC storage physical lifecycle
- FAT/path/storage policy
- input physical sampling and semantic input pipeline

The source tree should not require marker-only acceptance modules, smoke-only contract modules, or transition-era validator scripts to compile.

## Pulp scope

`vendor/pulp-os` may remain in the repository where it is still needed as imported runtime/reference material or compatibility surface, but Pulp hardware migration scaffolding must not be part of the production compile path.

## Validation

Use:

```bash
cargo fmt --all
./scripts/check_repo_hygiene.sh
cargo test -p vaachak-core --all-targets
cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
```

Device smoke remains required after firmware changes.
