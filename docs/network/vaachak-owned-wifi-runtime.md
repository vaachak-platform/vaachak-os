# Vaachak-owned Wi-Fi runtime

This repository no longer uses `vendor/pulp-os/src/apps/*` as the active home/app-manager or Wi-Fi runtime path.

Active X4 Wi-Fi code now lives under:

```text
target-xteink-x4/src/vaachak_x4/network/
```

The active X4 app dispatch code now lives under:

```text
target-xteink-x4/src/vaachak_x4/apps/home.rs
target-xteink-x4/src/vaachak_x4/apps/manager.rs
```

The boot runtime imports Vaachak-owned `HomeApp` and `AppManager`, and the special-mode dispatch calls Vaachak-owned network modules for:

```text
Wi-Fi setup
Wi-Fi scan
Wi-Fi transfer
Network time sync
Time status cache rendering
```

`vendor/pulp-os` remains present only as a compatibility/reference dependency for the still-migrating kernel, board, display, input, storage, reader, settings, and widget code. Do not add new features to `vendor/pulp-os`.

The accepted X4/CrossPoint-compatible partition table remains fixed:

```text
app0    offset 0x10000   size 0x640000
app1    offset 0x650000  size 0x640000
spiffs  offset 0xc90000  size 0x360000
```

Validation:

```bash
./scripts/validate_x4_standard_partition_table_compatibility.sh
./scripts/validate_vaachak_wifi_runtime_ownership.sh
```
