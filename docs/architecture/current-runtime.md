# Current Runtime State

## Status

Vaachak OS is in a cleaned X4 runtime baseline.

```text
vaachak_hardware_runtime_final_acceptance=ok
hardware_physical_full_migration_consolidation=ok
vendor_pulp_os_scope_reduction=ok
```

The active X4 target is `target-xteink-x4`. Active firmware code is under `target-xteink-x4/src/vaachak_x4/**`.

## Active runtime areas

| Area | Current owner/path |
| --- | --- |
| Boot/runtime entrypoint | `target-xteink-x4/src/vaachak_x4/imported/x4_reader_runtime.rs` |
| Home/category dashboard | `target-xteink-x4/src/vaachak_x4/apps/home.rs` |
| App manager | `target-xteink-x4/src/vaachak_x4/apps/manager.rs` |
| Files app | `target-xteink-x4/src/vaachak_x4/x4_apps/apps/files.rs` |
| Reader app | `target-xteink-x4/src/vaachak_x4/x4_apps/apps/reader/**` |
| Settings app | `target-xteink-x4/src/vaachak_x4/x4_apps/apps/settings.rs` |
| Network / Wi-Fi Transfer | `target-xteink-x4/src/vaachak_x4/network/**` |
| Time continuity | `target-xteink-x4/src/vaachak_x4/time/**` |
| Lua app host/catalog | `target-xteink-x4/src/vaachak_x4/lua/**` |
| Reader state bridge | `target-xteink-x4/src/vaachak_x4/io/**` and `apps/reader_state.rs` |
| X4 kernel/runtime helpers | `target-xteink-x4/src/vaachak_x4/x4_kernel/**` |
| X4 UI/font helpers | `target-xteink-x4/src/vaachak_x4/x4_apps/**` |

## Hardware and storage expectations

- Preserve the accepted X4/CrossPoint partition table.
- Preserve X4 display/input/storage behavior unless a dedicated hardware-safe change is being made.
- Keep SD/FAT path assumptions compatible with uppercase 8.3 physical folders where the embedded path layer requires them.
- Keep `/VAACHAK/APPS` as the canonical optional Lua app root.

## Vendor scope

`vendor/pulp-os` may remain as compatibility/reference material. New Vaachak OS functionality should not be added there.

`vendor/smol-epub` remains the EPUB dependency source and is excluded from the workspace.

## Validation

```bash
cargo fmt --all
./scripts/check_repo_hygiene.sh
cargo test -p vaachak-core --all-targets
cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
```
