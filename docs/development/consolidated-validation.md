# Consolidated Validation

## Repository validation

Use production checks instead of historical patch/slice validators:

```bash
cargo fmt --all
./scripts/check_repo_hygiene.sh
./scripts/validate_ui_shell_foundation.sh
cargo test -p vaachak-core --all-targets
cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
```

## Current expected markers

Accepted runtime/product markers include:

```text
vaachak_hardware_runtime_final_acceptance=ok
hardware_physical_full_migration_consolidation=ok
vendor_pulp_os_scope_reduction=ok
reader-bionic=x4-reader-bionic-reading-ok
reader-guide-dots=x4-reader-guide-dots-ok
reader-sunlight=x4-reader-sunlight-fading-fix-ok
ui-shell-foundation-vaachak-ok
```

## Flash-related validation

Use the retained production partition checks when flashing or changing partition policy:

```bash
./scripts/validate_x4_standard_partition_table_compatibility.sh
./scripts/validate_x4_flash_ota_slot_policy.sh
```

## Device smoke

After flashing, validate boot, display, input, SD, Files, Reader, Settings, Wi-Fi Transfer, Date & Time, and Back navigation on device.
