# vaachak-os

Bootstrap workspace for the future VaachakOS repository.

This repo is intentionally a **clean architecture skeleton plus first extracted X4 bootstrap slice**, not a migration of `x4-reader-os-rs`.
The current `x4-reader-os-rs` repo remains the X4 proving-ground and hardware truth source.

## Goals of this bootstrap

- establish the long-term crate boundaries
- define the first HAL seams
- extract the first real X4 slice for storage/display/input/power bootstrap
- keep Kernel, AppManager, and reader behavior in `x4-reader-os-rs` until the slice compiles cleanly
- keep sync, crypto, and advanced EPUB fidelity as deferred work

## Initial crates

- `core/` — shared `no_std` logic and interfaces
- `hal-xteink-x4/` — X4-specific HAL implementation placeholders and first extracted slice
- `target-xteink-x4/` — X4 target entrypoint placeholder
- `docs/` — architecture and migration notes

## What is extracted now

- X4 display geometry and bus topology constants
- X4 input threshold model for the proven button ladders
- X4 battery conversion/discharge mapping
- X4 storage lifecycle shape: probe -> mount -> close
- bootstrap sequencing for storage + display + power

## What is intentionally not extracted yet

- real esp-hal board init
- real SSD1677 driver code
- real SD/FAT plumbing
- real Reader rendering
- Kernel / AppManager orchestration
- Vaachak Sync implementation
- Waveshare support
- desktop simulator

## Intended next extraction order

1. refine the X4 HAL types against real `x4-reader-os-rs` files
2. extract display/input/power/storage bootstrap code cleanly
3. make `target-xteink-x4` build for the embedded target
4. only then start moving portable reader/cache state

## Current build stance

This workspace is still host-friendly and bootstrap-oriented.
It establishes the seams and the first extracted slice, but it is not yet a full embedded runtime.
