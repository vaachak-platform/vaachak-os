# vaachak-os

Bootstrap workspace for the future VaachakOS repository.

This repo is intentionally a **clean architecture skeleton**, not a migration of `x4-reader-os-rs`.
The current `x4-reader-os-rs` repo remains the X4 proving-ground and hardware truth source.

## Goals of this bootstrap

- establish the long-term crate boundaries
- define the first HAL seams
- create a reading-first Activity/UI backbone
- create the first X4 target crate without importing real runtime code yet
- keep sync, crypto, and advanced EPUB fidelity as deferred work

## Initial crates

- `core/` — shared `no_std` logic and interfaces
- `hal-xteink-x4/` — X4-specific HAL implementation placeholders
- `target-xteink-x4/` — X4 target entrypoint placeholder
- `docs/` — architecture and migration notes

## Not in this bootstrap

- real display driver code
- real input ladder decoding
- real SD/FAT plumbing
- real Reader rendering
- Vaachak Sync implementation
- Waveshare support
- desktop simulator

## Intended next extraction order

1. display/input/power/storage trait refinement
2. X4 HAL implementation skeleton
3. target boot path shell
4. reader/cache identity model
5. first portable reader state extraction

## Current build stance

This skeleton is set up to compile in a host-friendly placeholder mode so the workspace can be validated
while the embedded target crates are still just shells.
