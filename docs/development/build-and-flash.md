# Build and Flash

## Build validation

```bash
cargo fmt --all
./scripts/validate_vaachak_hardware_runtime_final_acceptance.sh
./scripts/validate_hardware_physical_full_migration_consolidation.sh
./scripts/validate_vendor_pulp_os_scope_reduction.sh
./scripts/validate_vaachak_docs_final_native_hardware_state.sh
cargo build
```

## Flash / run

Use the repository's normal X4 flashing command:

```bash
cargo run --release
```

## Hardware smoke

After flashing, validate:

```text
- device boots normally
- display initializes
- full refresh works
- partial/list refresh works
- all buttons respond correctly
- SD card initializes
- file browser opens
- SD root listing works
- TXT/EPUB files open
- progress/state/cache files work
- Back navigation works
- no FAT/path/cluster-chain errors
```
