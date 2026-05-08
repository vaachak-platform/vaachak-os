# Current Runtime

## Active firmware path

The active Xteink X4 firmware path is:

```text
vendor/pulp-os
```

This is the code path to build, flash, and validate on the device today.

## Why this is the source of truth

The Pulp-derived runtime already drives the exact X4 hardware path that has been validated on the device:

- ESP32-C3 startup
- SSD1677 display refresh
- button ladder input
- SD/FAT file access
- Home dashboard
- Reader and library flow
- Wi-Fi Transfer
- Date & Time sync mode
- Settings and sleep-image behavior

The root workspace exists to grow Vaachak-owned architecture around the working firmware without regressing the device.

## Root workspace role

The root workspace is not yet the primary flashed product. Its purpose is to hold target-neutral and X4-specific seams that can be adopted gradually:

- `core/` provides shared reader, storage, state, and text models.
- `hal-xteink-x4/` provides X4 HAL seams and smoke helpers.
- `target-xteink-x4/` provides adapter and contract code for future ownership transfer.

## Current product baseline

The current product baseline is reader-first:

- Category Home dashboard.
- Library and Reader path.
- Prepared cache support for mixed-script text.
- Wi-Fi Transfer with chunked resume for large cache folders.
- Date & Time sync through an isolated Wi-Fi mode.
- Settings persistence.
- Sleep-image modes and cached image support.

## Extraction rule

Hardware behavior remains in the active runtime until there is a small, tested, device-validated reason to move it. Pure logic can move earlier when it does not change device-visible behavior.
