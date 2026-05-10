# X4 CrossPoint-compatible partition table

Vaachak OS uses the Xteink X4 / CrossPoint-compatible 16 MB dual-OTA
partition layout so that CrossPoint-style flash tools and official-firmware
restore workflows do not reject the device after Vaachak has been flashed.

The earlier Vaachak OTA layout used larger app slots and a smaller SPIFFS
partition. Flash tools reported that layout as:

```text
app-ota_0      offset 65536     size 7798784
app-ota_1      offset 7864320   size 7798784
data-spiffs    offset 15663104  size 1048576
```

That is valid ESP-IDF partitioning, but it does not match the partition sizes
used by the current X4 CrossPoint/Biscuit firmware layout. Vaachak now uses this
layout instead:

| Name | Type | Subtype | Offset | Size |
| --- | --- | --- | ---: | ---: |
| `nvs` | data | nvs | `0x9000` | `0x5000` |
| `otadata` | data | ota | `0xe000` | `0x2000` |
| `app0` | app | ota_0 | `0x10000` | `0x640000` |
| `app1` | app | ota_1 | `0x650000` | `0x640000` |
| `spiffs` | data | spiffs | `0xc90000` | `0x360000` |
| `coredump` | data | coredump | `0xff0000` | `0x10000` |

Expected logical partition list after flashing Vaachak OS:

```text
data-nvs       offset 36864     size 20480
data-ota       offset 57344     size 8192
app-ota_0      offset 65536     size 6553600
app-ota_1      offset 6619136   size 6553600
data-spiffs    offset 13172736  size 3538944
data-coredump  offset 16711680  size 65536
```

Files that enforce this layout:

- `partitions/xteink_x4_standard.csv`
- `partitions/xteink_x4_standard.bin`
- `espflash.toml`
- `target-xteink-x4/espflash.toml`
- `vendor/pulp-os/espflash.toml`
- `scripts/validate_x4_standard_partition_table_compatibility.py`

## Validation

Run this before flashing or committing partition-related changes:

```bash
./scripts/validate_x4_standard_partition_table_compatibility.sh
```

Expected output:

```text
x4-crosspoint-partition-table-compatibility-ok
```

The validator fails if:

- the CSV does not match the CrossPoint-compatible X4 offsets and sizes;
- the binary partition table does not match the CSV;
- the binary partition table MD5 marker is invalid;
- `espflash.toml` no longer points to the checked partition table;
- a legacy factory app partition or data/phy partition is reintroduced.

## Migration from an incompatible Vaachak partition table

If an X4 already has the earlier Vaachak large-slot table, do a one-time
erase-and-flash so the partition table stored at `0x8000` is replaced:

```bash
./scripts/flash_x4_standard_partition_table.sh /dev/cu.usbmodemXXXX
```

After this migration, normal Vaachak flashes can use the standard command from
repo root:

```bash
cargo run --release
```

To restore another firmware after this, use that firmware's normal X4 flashing
instructions. Vaachak should no longer leave the device with the larger app-slot
layout that caused the `Got [...]` partition mismatch.
