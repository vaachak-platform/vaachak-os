# Build and Flash

## Xteink X4 partition table

Vaachak OS flashes with the Xteink X4 / CrossPoint-compatible dual-OTA partition table via `espflash.toml`:

```text
partitions/xteink_x4_standard.bin
```

The accepted compatibility boundaries are:

```text
app0   app/ota_0  offset 0x10000   size 0x640000
app1   app/ota_1  offset 0x650000  size 0x640000
spiffs data/spiffs offset 0xc90000 size 0x360000
```

Run flashing commands from the repository root so the local espflash configuration is visible.

## Build validation

```bash
cargo fmt --all
./scripts/check_repo_hygiene.sh
./scripts/validate_x4_standard_partition_table_compatibility.sh
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
```

## Normal app0 cable flashing

```bash
./scripts/flash_x4_vaachak_app0.sh /dev/cu.usbmodemXXXX
```

This preserves the accepted X4/CrossPoint partition table and erases only the `otadata` selector before flashing so the bootloader does not start a stale `app1` image left by another firmware.

## Partition-table recovery / migration

When migrating from an older Vaachak build that used an incompatible partition table, run:

```bash
./scripts/flash_x4_standard_partition_table.sh /dev/cu.usbmodemXXXX
```

If a direct `espflash flash` log says `Loaded app from partition at offset 0x650000`, run:

```bash
./scripts/erase_x4_otadata_select_app0.sh /dev/cu.usbmodemXXXX
```

Then flash again.

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
- Wi-Fi Transfer opens and returns safely
- Date & Time shows Live, Cached, or Unsynced without locking input
- no FAT/path/cluster-chain errors
```
