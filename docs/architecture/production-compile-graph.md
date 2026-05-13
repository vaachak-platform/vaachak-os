# Vaachak OS Production Compile Graph

Status: cleaned production baseline.

The active target compile graph keeps runtime-facing Vaachak modules and removes generated patch/deliverable artifacts from the repository root and scripts folder.

## Production-facing target modules

- `boot.rs` and `imported/x4_reader_runtime.rs`
- `apps/**` for Home/category dashboard and app manager
- `x4_apps/**` for Files, Reader, Settings, widgets, and UI/font helpers
- `x4_kernel/**` for X4 runtime helpers
- `network/**` for Wi-Fi setup, Wi-Fi Transfer, and network time
- `lua/**` for optional SD-loaded app hosting/catalog support
- `io/**`, `state/**`, `text/**`, `sleep/**`, and `time/**` for active runtime support
- `contracts/**` and `runtime_adapter_contracts.rs` for current explicit contracts

## Removed from repository hygiene baseline

- root zip files
- extracted deliverable folders
- generated apply/patch scripts
- one-off repair/cleanup/feature validator scripts
- Python bytecode/cache folders
- macOS metadata folders

## Validation

```bash
cargo fmt --all
./scripts/check_repo_hygiene.sh
cargo test -p vaachak-core --all-targets
cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
```
