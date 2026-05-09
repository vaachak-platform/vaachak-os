# Hardware Migration Readiness

## Current status

The uploaded repository is not yet a fully Vaachak-native hardware runtime. The active device path imports `vendor/pulp-os` for board, display, input, SD/FAT, kernel, and app runtime behavior.

This means hardware migration should not be described as complete in docs. The current state is:

```text
hardware runtime behavior: imported Pulp-derived runtime
Vaachak-owned code: contracts, helpers, models, preflight probes, and migration seams
```

## Readiness gate before the next hardware behavior move

Run from repository root:

```bash
cargo fmt --all
cargo build
./scripts/validate_documentation_refresh.sh
```

If doing embedded validation on the X4 toolchain, also run the target-specific checks documented in `docs/development/build-and-flash.md`.

## Device smoke baseline

Before moving any hardware behavior, verify the current baseline:

- device boots repeatedly
- Home/category dashboard appears
- Files/Library opens
- SD card lists files
- TXT opens
- EPUB/EPU smoke path opens
- prepared cache path still reports clean success/failure status
- Back navigation works
- Settings persists expected values
- Wi-Fi Transfer remains usable
- Date & Time screen remains cancellable and recoverable
- sleep-image mode still works

## Recommended next hardware migration order

Hardware behavior should only move after Reader Home / data model stabilization unless a bug requires it.

Recommended order if/when hardware migration resumes:

1. input physical sampling interpretation, with Pulp physical reads available as comparison baseline
2. SPI transaction ownership and arbitration, because display and SD share the bus
3. display command sequencing, keeping rendering artifacts visible in smoke tests
4. SD/MMC lifecycle, with FAT behavior protected by tests and smoke checks
5. FAT algorithms, after library and reader state models are frozen

## What not to do

- Do not combine input, SPI, display, and SD/FAT changes in one unvalidated slice.
- Do not delete `vendor/pulp-os` while the uploaded code still imports it.
- Do not claim native hardware ownership in docs unless the active code path proves it.
- Do not add feature work that destabilizes the reader baseline.
