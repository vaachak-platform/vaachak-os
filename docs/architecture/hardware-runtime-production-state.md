# Vaachak OS Hardware Runtime Production State

Status: cleaned production baseline.

The X4 runtime should now be described in terms of production modules and product behavior, not migration slices, temporary acceptance markers, or generated validator scripts.

## Production ownership

Vaachak owns the active X4 target code under `target-xteink-x4/src/vaachak_x4/**`.

Production areas:

- boot/runtime entrypoint
- Home/category dashboard and app manager
- Files, Reader, Settings, and Network apps
- X4 kernel/runtime helpers
- display refresh lifecycle and drawing helpers
- input handling and app navigation
- SD/FAT/path behavior used by the active X4 target
- reader state, title cache, prepared cache metadata, and settings
- optional Lua app catalog/host path

## Pulp scope

`vendor/pulp-os` may remain in the repository for scoped compatibility/reference use, but it should not receive new Vaachak OS functionality.

## Validation

Use production validation instead of slice validators:

```bash
cargo fmt --all
./scripts/check_repo_hygiene.sh
cargo test -p vaachak-core --all-targets
cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
```

Device smoke remains required after firmware changes.
