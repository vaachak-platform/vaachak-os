# Vaachak Hardware Runtime Final Acceptance

This document records the accepted Xteink X4 hardware/runtime baseline.

## Accepted baseline

```text
vaachak_hardware_runtime_final_acceptance=ok
hardware_physical_full_migration_consolidation=ok
vendor_pulp_os_scope_reduction=ok
```

## Final ownership state

| Area | Current accepted owner |
| --- | --- |
| X4 boot/runtime entrypoint | Vaachak `target-xteink-x4` |
| X4 app/runtime path | Vaachak `target-xteink-x4` |
| Reader/files/settings/network path | Vaachak `target-xteink-x4` |
| Optional Lua app host/catalog | Vaachak `target-xteink-x4` |
| Remaining `vendor/pulp-os` scope | compatibility/reference only |

## Current validation

```bash
cargo fmt --all
./scripts/check_repo_hygiene.sh
./scripts/audit_remaining_pulp_runtime_dependencies.sh
cargo test -p vaachak-core --all-targets
cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
```

## Hardware smoke

After static/build checks pass, flash and validate on-device:

```bash
./scripts/flash_x4_vaachak_app0.sh /dev/cu.usbmodemXXXX
```

Expected smoke:

- boots normally
- display initializes
- full refresh works
- partial/list refresh works
- all buttons respond correctly
- SD card initializes
- storage availability state is correct
- file browser opens
- SD root listing works
- nested directory listing works if available
- long filename/title mapping still works
- TXT files open
- EPUB files open
- progress/state/cache files still work
- Back navigation works
- Wi-Fi Transfer and Date & Time paths do not lock input
- no FAT/path/cluster-chain errors
- no blank/stuck display
- no input regression
