# Vaachak OS Scope

## Current accepted scope

Vaachak OS is currently scoped as an Xteink X4 reader-first firmware with Vaachak-native hardware ownership accepted.

In scope now:

- X4 boot, display, input, storage, reader, settings, transfer, and sleep-state validation.
- Vaachak-native hardware runtime surfaces for SPI, SSD1677 display, SD/MMC, FAT, and input sampling.
- Reader Home / Continue Reading and local library polish.
- Reader state model freeze for progress, bookmarks, highlights, per-book settings, and library entries.
- XTC compatibility as an import/open path.
- `.vchk` as the long-term Vaachak-native book package contract.
- Sync alignment after local reader state is stable.

## Explicitly retained but not active hardware scope

`vendor/pulp-os` remains in the repository for scoped non-hardware compatibility, import, reference, and historical comparison surfaces. It is not the active X4 hardware owner.

## Deferred scope

- Palm/Tern compatibility as a product milestone.
- Broad app ecosystem.
- Script/plugin runtime.
- OPDS, analytics, achievements, games, and cloud-only flows.
- Waveshare/S3 profile implementation until the X4 reader path is stable.
- Sync transport features that require reader-state redesign.
