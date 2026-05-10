# Vaachak Runtime Vendor Retirement

The active Xteink X4 firmware path no longer depends on `vendor/pulp-os` or the `x4-os` package.

Active runtime ownership now lives under `target-xteink-x4/src/vaachak_x4`:

- `x4_kernel/` owns board, display, input, SD/FAT storage helpers, scheduler, configuration, work queue, and sleep bitmap behavior.
- `x4_apps/` owns file browser, reader, settings, widgets, app-side UI, and bitmap font runtime.
- `apps/` owns the Vaachak category dashboard and app manager.
- `network/` owns Wi-Fi transfer, Wi-Fi setup/scan, and network time.
- `imported/x4_reader_runtime.rs` is the active X4 boot/runtime entrypoint.

The target crate no longer has these dependencies:

```text
pulp-os
x4-kernel
```

The accepted X4/CrossPoint-compatible partition table remains unchanged:

```text
app0    0x10000   0x640000
app1    0x650000  0x640000
spiffs  0xc90000  0x360000
```

Validation:

```bash
./scripts/validate_x4_standard_partition_table_compatibility.sh
./scripts/validate_vaachak_wifi_runtime_ownership.sh
./scripts/audit_remaining_pulp_runtime_dependencies.sh
./scripts/check_no_milestone_artifacts.sh
./scripts/check_repo_hygiene.sh
```
