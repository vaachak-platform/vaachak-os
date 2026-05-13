# Vaachak OS Hardware Runtime Architecture

## Accepted X4 state

```text
vaachak_hardware_runtime_final_acceptance=ok
hardware_physical_full_migration_consolidation=ok
vendor_pulp_os_scope_reduction=ok
```

The active X4 runtime lives in the Vaachak target tree:

```text
target-xteink-x4/src/vaachak_x4
```

## Active runtime surfaces

| Surface | Active path |
| --- | --- |
| Boot/runtime entrypoint | `imported/x4_reader_runtime.rs` |
| Board/runtime helpers | `x4_kernel/**` |
| Display/UI helpers | `x4_kernel/drivers/**`, `x4_apps/**`, `display/**`, `ui/**` |
| Input/navigation | `x4_kernel/**`, `input/**`, `apps/manager.rs` |
| SD/FAT/path behavior | `x4_kernel/**`, `io/**`, `state/**` |
| Files app | `x4_apps/apps/files.rs` |
| Reader app | `x4_apps/apps/reader/**` |
| Settings app | `x4_apps/apps/settings.rs` |
| Network/Wi-Fi Transfer | `network/**` |
| Lua app host/catalog | `lua/**` |

## Vendor scope

`vendor/pulp-os` is retained only as scoped compatibility/reference material. New work should not be added there.

`vendor/smol-epub` remains the EPUB dependency source.

## Partition table rule

Keep the accepted X4/CrossPoint-compatible partition table:

```text
app0    0x10000   0x640000
app1    0x650000  0x640000
spiffs  0xc90000  0x360000
```

## Validation

```bash
cargo fmt --all
./scripts/check_repo_hygiene.sh
./scripts/validate_x4_standard_partition_table_compatibility.sh
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
```
