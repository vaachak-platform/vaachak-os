# X4 Vaachak app0 flashing policy

Vaachak OS keeps the Xteink X4 / CrossPoint-compatible partition table:

```text
nvs       0x9000    0x5000
otadata   0xe000    0x2000
app0      0x10000   0x640000
app1      0x650000  0x640000
spiffs    0xc90000  0x360000
coredump  0xff0000  0x10000
```

## Why normal `espflash flash` can appear to fail

With this dual-OTA partition table, ESP-IDF stores the selected boot slot in
`otadata`. If another firmware or an OTA test selected `app1`, a direct
`espflash flash target/.../target-xteink-x4` may update `app0` while the
bootloader still starts `app1` at `0x650000`.

The symptom looks like this:

```text
Flashing has completed!
...
boot: Loaded app from partition at offset 0x650000
Error: Broken pipe
```

The flash did complete. The problem is stale OTA selection, not the partition
table itself.

## Normal Vaachak cable flash command

Use this helper for Vaachak OS cable flashing:

```bash
./scripts/flash_x4_vaachak_app0.sh /dev/cu.usbmodemXXXX
```

The helper:

1. validates the X4 partition table,
2. builds the X4 target,
3. erases only `otadata` at `0xe000..0x10000`, and
4. flashes/monitors Vaachak OS.

Erasing `otadata` is intentional for this workflow. It makes app0 the safe
cable-flash default and prevents booting a stale app1 image.

## One-time recovery when app1 is already selected

```bash
./scripts/erase_x4_otadata_select_app0.sh /dev/cu.usbmodemXXXX
espflash flash \
  --chip esp32c3 \
  --baud 115200 \
  --monitor \
  --before default-reset \
  --port /dev/cu.usbmodemXXXX \
  target/riscv32imc-unknown-none-elf/release/target-xteink-x4
```

## Validation

```bash
./scripts/validate_x4_standard_partition_table_compatibility.sh
./scripts/validate_x4_flash_ota_slot_policy.sh
```
