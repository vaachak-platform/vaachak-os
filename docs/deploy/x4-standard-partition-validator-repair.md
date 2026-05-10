# X4 Standard Partition Validator Repair

This repair fixes the standard partition validator so it does not scan extracted overlay directories left in the repository root.

The original validator scanned the entire working tree. That caused a false failure when the extracted `x4_standard_partition_table_compatibility/` overlay folder remained in the repo root, because the overlay validator script itself intentionally contains legacy terms such as `app-factory` while checking that the real repo does not use that layout.

The repaired validator still checks:

- `partitions/xteink_x4_standard.csv`
- `partitions/xteink_x4_standard.bin`
- root `espflash.toml`
- `target-xteink-x4/espflash.toml`
- `vendor/pulp-os/espflash.toml`
- source/config files under `partitions/`, `scripts/`, `target-xteink-x4/`, and `vendor/pulp-os/`

It no longer treats extracted patch folders as active source/config.

Apply:

```bash
unzip -o x4_standard_partition_validator_repair.zip
chmod +x x4_standard_partition_validator_repair/scripts/*.sh x4_standard_partition_validator_repair/scripts/*.py
./x4_standard_partition_validator_repair/scripts/apply_x4_standard_partition_validator_repair.sh
```

Validate:

```bash
./scripts/validate_x4_standard_partition_validator_repair.sh
./scripts/validate_x4_standard_partition_table_compatibility.sh
```
