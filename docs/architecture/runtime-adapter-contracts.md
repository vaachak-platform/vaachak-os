# Runtime adapter contracts

This document records the adapter-facing contract between Vaachak-owned pure models and the active X4 runtime.

## Current active runtime

`vendor/pulp-os` remains the active firmware runtime. It still owns:

- SD mount/probe and file I/O behavior
- SPI bus behavior
- SSD1677 display driver behavior
- strip rendering
- EPD full/partial refresh behavior
- ADC ladder scanning and debounce behavior
- Wi-Fi connection, HTTP upload, and mDNS behavior
- active reader, file browser, settings, date/time, title-cache, sleep-image, and transfer behavior

The adapter contracts added in this slice do not move any of that behavior.

## Contract purpose

The contracts are thin mapping helpers that align Vaachak-owned core models with names and paths used by the current runtime:

- core storage/path models to current Pulp SD paths
- core input semantics to current button event names
- core display/chrome layout roles to current runtime chrome roles
- core Wi-Fi Transfer config defaults to the current browser upload UI defaults

These contracts are meant to make future migration safer by keeping the active runtime boundary explicit.

## Non-goals

This slice does not change reader behavior, file browser behavior, settings behavior, Wi-Fi behavior, date/time behavior, title-cache behavior, sleep behavior, display refresh behavior, SD behavior, or SPI behavior.

## Readiness status

The next hardware-adjacent migration should not start until this contract gate remains green after a flash and on-device smoke pass.

## Storage read-only adapter facade

The storage read-only adapter facade is now defined in `target-xteink-x4/src/vaachak_x4/io/storage_readonly_adapter.rs`.

It adds Vaachak-owned contracts for file existence, start-of-file reads, offset chunk reads, directory metadata listing, and resolving the current storage path map.

The active implementation remains Pulp-backed. SD mount/probe, SD driver behavior, FAT/filesystem behavior, SPI arbitration, display behavior, and reader behavior still live in `vendor/pulp-os`.

## Storage read-only Pulp bridge

The storage read-only Pulp bridge is now defined in `target-xteink-x4/src/vaachak_x4/io/storage_readonly_pulp_bridge.rs`.

It implements the Vaachak-owned read-only facade through a narrow `PulpReadonlyStorageBackend` interface and an embedded `X4PulpReadonlyStorageBackend` that delegates to existing Pulp read/list/size helpers.

The bridge is read-only. It does not add write, delete, rename, create, truncate, append, mkdir, mount, unmount, format, SD probe, SPI arbitration, display, reader, or file-browser behavior. Those active behaviors remain in `vendor/pulp-os`.


## Storage read-only boundary consolidation

The canonical storage read-only boundary is now defined in `target-xteink-x4/src/vaachak_x4/io/storage_readonly_boundary.rs` and documented in `docs/architecture/storage-readonly-boundary.md`.

It consolidates the Vaachak-owned read-only facade and the Pulp-backed bridge into one boundary entrypoint. The public contract remains `VaachakReadonlyStorage`, and the active implementation path remains `PulpReadonlyStorageBridge` backed by existing Pulp read/list/size helpers.

This consolidation does not add write, delete, rename, create, truncate, append, mkdir, mount, unmount, format, SD probe, SPI arbitration, display, reader, or file-browser behavior. Those active behaviors remain in `vendor/pulp-os`.
