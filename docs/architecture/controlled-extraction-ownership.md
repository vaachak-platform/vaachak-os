# Controlled extraction ownership

This document consolidates the current Vaachak-owned pure model extractions and the active runtime boundary.

## Active runtime

`vendor/pulp-os` remains the active X4 runtime. It still owns the running firmware behavior for:

- SD card probing, mounting, and file I/O
- SPI bus behavior
- SSD1677 display driver behavior
- strip rendering
- EPD refresh behavior
- partial/full refresh policy
- button scan/debounce behavior
- Wi-Fi connection logic
- HTTP upload handling
- mDNS behavior
- reader app runtime state machine
- file browser runtime behavior
- settings app runtime behavior

The root `vaachak-core` crate now owns pure domain models and compatibility contracts only. These models are intentionally behavior-neutral and hardware-free.

## Vaachak-owned pure models

Current owned model slices:

- `core/src/models/state.rs`
  - reader preferences
  - sleep image mode state
  - Date & Time state
  - safe Wi-Fi Transfer state shape

- `core/src/models/reader_state_io.rs`
  - reader progress records
  - bookmark records
  - bookmark index records
  - compatibility with `state/<BOOKID>.PRG`, `state/<BOOKID>.BKM`, and `state/BMIDX.TXT`

- `core/src/models/book_identity_title_cache.rs`
  - stable book identity
  - title-cache record shape
  - title fallback rules
  - compatibility with `_x4/TITLES.BIN`

- `core/src/models/prepared_cache_metadata.rs`
  - FCACHE path and metadata models
  - prepared page/chapter metadata
  - missing-cache versus malformed-cache classification

- `core/src/models/input_semantic_mapping.rs`
  - semantic input actions
  - app-context input policy
  - reader navigation semantics

- `core/src/models/storage_path_helpers.rs`
  - SD layout constants
  - safe path joining
  - state, FCACHE, settings, title-cache, and sleep-image path helpers

- `core/src/models/wifi_transfer_config.rs`
  - Original Transfer / Chunked Resume configuration
  - chunk/retry limits
  - target folder validation
  - no credential storage

- `core/src/models/display_drawing_abstractions.rs`
  - 800x480 display geometry
  - header/body/footer/popup regions
  - app-context layout metadata
  - diagnostic placement rules

## Non-goals for this checkpoint

This checkpoint does not move SD, SPI, display, input, Wi-Fi, or reader runtime behavior out of `vendor/pulp-os`.

It also does not change on-device behavior. It only consolidates validation and documents current ownership.

## Migration principle

Future hardware-adjacent work should move one behavior path at a time. Every move must preserve the existing Pulp-derived runtime as the comparison baseline and must keep the device flashable after each slice.
