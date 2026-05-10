# Build and Flash

## Xteink X4 partition table

Vaachak OS flashes with the Xteink X4 / CrossPoint-compatible dual-OTA partition
table via `espflash.toml`:

```text
partitions/xteink_x4_standard.bin
```

The important compatibility boundaries are:

```text
app0   app/ota_0  offset 0x10000   size 0x640000
app1   app/ota_1  offset 0x650000  size 0x640000
spiffs data/spiffs offset 0xc90000 size 0x360000
```

Run flashing commands from the repository root, or from `target-xteink-x4/` or
`vendor/pulp-os/`, so the local espflash configuration is visible.

When migrating from an older Vaachak build that used an incompatible partition
table, run the one-time erase-and-flash helper first:

```bash
./scripts/flash_x4_standard_partition_table.sh /dev/cu.usbmodemXXXX
```

## Build validation

```bash
cargo fmt --all
./scripts/validate_x4_standard_partition_table_compatibility.sh
cargo build
```

## Flash / run

Use the repository's normal X4 flashing command from repo root:

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

Production hygiene check:

```bash
./scripts/check_repo_hygiene.sh
```

## X4 Vaachak app0 cable flashing

For normal Xteink X4 cable flashing, prefer:

```bash
./scripts/flash_x4_vaachak_app0.sh /dev/cu.usbmodemXXXX
```

This preserves the accepted X4/CrossPoint partition table and erases only the
`otadata` selector before flashing so the bootloader does not start a stale
`app1` image left by another firmware.

If a direct `espflash flash` log says `Loaded app from partition at offset
0x650000`, run:

```bash
./scripts/erase_x4_otadata_select_app0.sh /dev/cu.usbmodemXXXX
```

Then flash again.
