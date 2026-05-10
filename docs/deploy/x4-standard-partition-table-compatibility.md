# X4 standard partition table compatibility

Vaachak OS now uses the standard Xteink X4 OTA-compatible 16 MB partition
layout. This avoids leaving the device in the old Vaachak single-factory layout,
which external X4 tooling reports as an unexpected partition configuration.

## Standard layout

| Label | Type | Subtype | Offset | Size |
|---|---:|---:|---:|---:|
| `nvs` | data | nvs | `0x9000` | `0x5000` |
| `otadata` | data | ota | `0xe000` | `0x2000` |
| `ota_0` | app | ota_0 | `0x10000` | `0x770000` |
| `ota_1` | app | ota_1 | `0x780000` | `0x770000` |
| `spiffs` | data | spiffs | `0xef0000` | `0x100000` |
| `coredump` | data | coredump | `0xff0000` | `0x10000` |

Source files:

- `partitions/xteink_x4_standard.csv`
- `partitions/xteink_x4_standard.bin`
- `espflash.toml`
- `target-xteink-x4/espflash.toml`
- `vendor/pulp-os/espflash.toml`

The local `espflash.toml` files point espflash to the generated binary partition
table and set flash size to `16MB`.

## Migration from previous Vaachak layout

The previous Vaachak layout used a single large factory app partition. Moving
from that layout to the X4 standard layout changes the partition table, so do a
one-time full flash erase before flashing this build.

From repository root:

```bash
cargo fmt --all
./scripts/validate_x4_standard_partition_table_compatibility.sh
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
```

Then migrate and flash:

```bash
./scripts/flash_x4_standard_partition_table.sh /dev/cu.usbmodemXXXX
```

The script asks for explicit confirmation before erasing flash.

## Normal flashing after migration

After the one-time migration, use the normal root-level flash command. Because
`espflash.toml` lives in the repo, espflash will use the standard partition table.

```bash
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
espflash flash \
  --monitor \
  --chip esp32c3 \
  --port /dev/cu.usbmodemXXXX \
  target/riscv32imc-unknown-none-elf/release/target-xteink-x4
```

## External firmware compatibility check

After flashing this layout, CrossPoint or official Xteink tooling should see the
standard OTA partition set instead of the old Vaachak factory-only layout.

Expected logical partition list:

```text
data-nvs       offset 36864     size 20480
data-ota       offset 57344     size 8192
app-ota_0      offset 65536     size 7798784
app-ota_1      offset 7864320   size 7798784
data-spiffs    offset 15663104  size 1048576
data-coredump  offset 16711680  size 65536
```

## Guardrail

Run this validation before committing partition-related changes:

```bash
./scripts/validate_x4_standard_partition_table_compatibility.sh
```

It fails if:

- the standard CSV rows change unexpectedly;
- the binary partition table does not match the CSV;
- the MD5 marker is invalid;
- `espflash.toml` no longer points to the standard binary;
- a factory app partition or data/phy partition is reintroduced.
