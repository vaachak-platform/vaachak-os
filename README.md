# Vaachak OS

Vaachak OS is an X4-first reader operating system for ESP32-family e-paper devices. The current Xteink X4 branch has accepted the Vaachak-native hardware runtime and now uses Vaachak-owned hardware driver surfaces for SPI, SSD1677 display, SD/MMC, FAT, and input sampling.

## Current status

Accepted hardware gate:

```text
vaachak_hardware_runtime_final_acceptance=ok
hardware_physical_full_migration_consolidation=ok
vendor_pulp_os_scope_reduction=ok
```

The active X4 hardware ownership map is:

| Hardware surface | Active Vaachak owner |
| --- | --- |
| SPI physical driver | `VaachakNativeSpiPhysicalDriver` |
| SSD1677 display driver | `VaachakNativeSsd1677PhysicalDriver` |
| SD/MMC physical driver | `VaachakNativeSdMmcPhysicalDriver` |
| FAT/filesystem algorithm driver | `VaachakNativeFatAlgorithmDriver` |
| Input physical sampling | Vaachak native input physical sampling driver and native input event pipeline |

Pulp OS is not the active hardware runtime. `vendor/pulp-os` remains present only for scoped non-hardware compatibility, import, reference, or historical comparison surfaces until the remaining dependency audit says it can be reduced further.

## Product direction

The product remains reader-first:

1. Boot reliably on Xteink X4.
2. Show Home / Reader / Library / Settings / Transfer surfaces.
3. Open local reading content from storage.
4. Preserve progress, bookmarks, and per-book state.
5. Keep Wi-Fi transfer/import practical and bounded.
6. Prepare XTC compatibility and `.vchk` as the Vaachak-native package path.
7. Align local state with Vaachak sync only after local reader behavior is stable.

The architecture keeps the uploaded planning direction: X4-first, shared-core friendly, native Vaachak contracts before compatibility layers, and Waveshare/S3 as a later capability profile.

## Repository layout

```text
core/                 Shared Vaachak data models and contracts
hal-xteink-x4/        X4 HAL-facing seams and smoke helpers
target-xteink-x4/     X4 target integration, native hardware runtime, contracts, state, and imported compatibility boundaries
vendor/pulp-os/       Retained non-hardware compatibility/import/reference scope
src/apps/             Current first-party app/runtime path used by the X4 firmware
docs/                 Architecture, roadmap, validation, reader, state, and format planning
scripts/              Build/validation and cleanup helpers
```

## Validation

Run the current final gate before committing or flashing:

```bash
cargo fmt --all
cargo build
```

Run the device smoke after flashing:

```bash
cargo run --release
```

Hardware smoke checklist:

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

## Next roadmap focus

The next work should move away from hardware migration and toward the reader product path:

1. Reader Home + Continue Reading foundation.
2. Reader data model freeze.
3. Library index polish.
4. XTC compatibility path.
5. `.vchk` spec freeze.
6. `.vchk` read/open support.
7. `.vchk` mutable reading-state support.
8. Vaachak sync alignment.

Do not expand platform features ahead of the reader path.


Production hygiene check:

```bash
./scripts/check_repo_hygiene.sh
```
