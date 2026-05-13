# Vaachak Runtime Vendor Retirement

The active Xteink X4 firmware path no longer depends on a `pulp-os`/`x4-os` workspace package. Active runtime ownership lives under `target-xteink-x4/src/vaachak_x4`.

Active runtime areas:

- `x4_kernel/` owns the current X4 board/runtime helpers.
- `x4_apps/` owns file browser, reader, settings, widgets, app-side UI, and bitmap font runtime.
- `apps/` owns the Vaachak category dashboard and app manager.
- `network/` owns Wi-Fi transfer, Wi-Fi setup/scan, and network time.
- `lua/` owns the optional SD-loaded app catalog/host path.
- `imported/x4_reader_runtime.rs` is the active X4 boot/runtime entrypoint.

The target crate no longer has these package dependencies:

```text
pulp-os
x4-kernel
```

`vendor/pulp-os` may remain in the repository as scoped compatibility/reference material. Do not add new Vaachak OS functionality there.

The accepted X4/CrossPoint-compatible partition table remains unchanged:

```text
app0    0x10000   0x640000
app1    0x650000  0x640000
spiffs  0xc90000  0x360000
```

Validation:

```bash
./scripts/check_repo_hygiene.sh
./scripts/audit_remaining_pulp_runtime_dependencies.sh
./scripts/validate_x4_standard_partition_table_compatibility.sh
./scripts/validate_x4_flash_ota_slot_policy.sh
```
