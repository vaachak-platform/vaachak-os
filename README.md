# Vaachak OS

Vaachak OS is an Xteink X4-first reader firmware for ESP32-C3 e-paper devices. The current repository state is a commit-clean product/runtime baseline: old patch zip files, extracted deliverable folders, and temporary patch validator scripts are not part of the working tree.

## Current accepted state

The active target is `target-xteink-x4`. The X4 path is reader-first and uses Vaachak-owned runtime code under `target-xteink-x4/src/vaachak_x4/**`.

Accepted baseline:

```text
vaachak_hardware_runtime_final_acceptance=ok
hardware_physical_full_migration_consolidation=ok
vendor_pulp_os_scope_reduction=ok
reader-bionic=x4-reader-bionic-reading-ok
reader-guide-dots=x4-reader-guide-dots-ok
reader-sunlight=x4-reader-sunlight-fading-fix-ok
```

Current product capabilities:

- Xteink X4 / ESP32-C3 boot with the accepted X4/CrossPoint-compatible partition table.
- Vaachak-owned X4 app/runtime path for Home, Files, Reader, Settings, Network, Wi-Fi Transfer, and optional Lua apps.
- Reader support for local TXT and EPUB paths, progress/state files, bookmarks, title cache, prepared cache metadata, reader settings, Bionic Reading, Guide Dots, sunlight-fading mitigation, and SD/static font work.
- Category dashboard with Network, Productivity, Games, Reader, System, and Tools.
- Vaachak-owned internal UI shell foundation for larger tabbed pages such as Settings, Files, Library, Fonts, Network, and Tools.
- Wi-Fi setup/scan, Wi-Fi Transfer, and network time integration through Vaachak-owned X4 target code.
- Optional Lua app deployment from `/VAACHAK/APPS` using uppercase 8.3-safe physical folders.
- Sample Lua app pack for Calendar, Panchang, Daily Mantra, Dictionary, Unit Converter, and Games.
- Sleep image mode helpers for Daily, Fast Daily, Static, Cached, Text, and No Redraw flows.

## Vendor scope

`vendor/pulp-os` may remain in the repository as compatibility/reference material, but new functionality should not be added there. Active Vaachak OS work belongs under Vaachak-owned paths such as:

```text
target-xteink-x4/src/vaachak_x4/**
core/**
hal-xteink-x4/**
support/**
examples/sd-card/**
docs/**
tools/**
```

`vendor/smol-epub` remains the EPUB dependency source and is excluded from the workspace.

## Repository layout

```text
core/                 Shared Vaachak data models and contracts
hal-xteink-x4/        X4 HAL-facing seams and smoke helpers
target-xteink-x4/     X4 firmware target, runtime, apps, reader, network, state, and UI
support/              Optional support crates such as the Lua VM wrapper
examples/sd-card/     SD-card sample assets and optional Lua apps
tools/                Host-side asset/font/dictionary/prepared-cache tooling
partitions/           Accepted X4/CrossPoint partition table assets
docs/                 Current architecture, development, reader, Lua, network, state, and operations docs
scripts/              Production helper scripts only
vendor/               Retained reference/dependency sources
```

## Validation

Run these before committing:

```bash
cargo fmt --all
./scripts/check_repo_hygiene.sh
./scripts/validate_ui_shell_foundation.sh
cargo test -p vaachak-core --all-targets
cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
```

For flashing helpers that depend on the X4 partition table, keep:

```bash
./scripts/validate_x4_standard_partition_table_compatibility.sh
./scripts/validate_x4_flash_ota_slot_policy.sh
```

## Flashing

Normal app0 cable flashing:

```bash
./scripts/flash_x4_vaachak_app0.sh /dev/cu.usbmodemXXXX
```

One-time partition-table migration / recovery helper:

```bash
./scripts/flash_x4_standard_partition_table.sh /dev/cu.usbmodemXXXX
```

## Hardware smoke checklist

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

## Next roadmap focus

The hardware cleanup track is no longer the main path. The next work should stay focused on the reader product path:

1. Reader Home and Continue Reading polish.
2. Reader data model freeze.
3. Library index and title-cache polish.
4. XTC compatibility path.
5. `.vchk` package spec freeze.
6. `.vchk` read/open support.
7. Mutable `.vchk` reading-state support.
8. Vaachak sync alignment.

Do not add broad platform features ahead of the reader path.
