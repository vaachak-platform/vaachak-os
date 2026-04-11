# VaachakOS — Implementation Guide

This document translates the architecture into concrete implementation rules.

It is intended for contributors and coding agents who need to make progress without widening scope or restructuring the repository in harmful ways.

---

## 1. Implementation Objective

The objective is to ship a stable X4 e-reader with a codebase that remains clean enough to support later extraction for another board.

That means the implementation must optimize for:

- correctness on X4
- low fragility
- clear ownership boundaries
- small, verifiable steps

It must not optimize for speculative extensibility.

---

## 2. Active Repository Shape

The active structure is:

```text
core/
drivers/
boards/x4/
targets/x4/
```

Any new code must justify which of those areas it belongs to.

### Ownership summary

#### `core/`
Owns:
- book model
- reader state
- progress model
- settings model
- shell state
- sidecar model
- cache schema
- shared rendering model

#### `drivers/`
Owns:
- SSD1677 behavior
- SD/FAT behavior helpers
- ADC key decoding
- battery calculation
- refresh policy

#### `boards/x4/`
Owns:
- pin mapping
- bus and peripheral wiring
- board profile
- board boot glue
- X4-specific wake/sleep handling
- optional SDK/FFI bridge

#### `targets/x4/`
Owns:
- executor startup
- top-level boot path
- task spawning
- panic path integration

---

## 3. Runtime Shape

VaachakOS v1 should stay small at runtime.

### Tasks

#### `main_task`
Responsible for:
- owning current screen state
- reacting to input events
- initiating render
- coordinating current book/session state

#### `input_task`
Responsible for:
- polling button source
- debouncing or receiving debounced events
- forwarding input events to main task

#### `housekeeping_task`
Responsible for:
- low-frequency battery refresh
- progress flush
- lightweight cache maintenance
- low-frequency state persistence

#### `sleep_task`
Responsible for:
- idle timer
- sleep request generation
- pre-sleep state flush coordination

Do not add more tasks unless there is a real need.

---

## 4. Screen Model

Use a fixed screen enum.

Example shape:

```rust
pub enum Screen {
    Home(HomeState),
    Library(LibraryState),
    Reader(ReaderState),
    Settings(SettingsState),
    Debug(DebugState),
}
```

### Rules

- screen transitions must be explicit
- state should remain owned by the current screen or core services it actually needs
- avoid heap-backed trait-object navigation in v1

### Why

This reduces heap churn, reduces indirection, and makes debugging on constrained hardware much easier.

---

## 5. Display Implementation Rules

### 5.1 Start from stability, not flexibility

The display path must first solve:

- init
- orientation
- inversion/polarity
- full refresh
- partial refresh
- sleep

Do not start by building a generalized display API wider than what is needed.

### 5.2 Strip rendering only

The renderer should produce strips and hand them to the display driver.

Do not require a full framebuffer for v1.

### 5.3 Keep refresh policy separate

The policy for when to do partial vs full refresh should live in `drivers/refresh_policy.rs`, not be scattered across screens.

---

## 6. Reader Implementation Rules

### 6.1 Support a constrained EPUB subset first

The first goal is readable content, not perfect fidelity.

Allowed approach:
- metadata extraction
- simple text flow
- minimal formatting subset
- constrained pagination

Deferred:
- rich CSS
- complex tables
- footnote systems
- advanced typography tuning

### 6.2 TXT support should land early

TXT is a simpler path and should be used to validate:
- library flow
- pagination flow
- progress persistence
- render stability

### 6.3 Cache is disposable

Cache artifacts may be rebuilt.

Reader progress and bookmarks are durable state and must not depend on cache survival.

---

## 7. Storage Rules

### Durable state

Durable state includes:
- settings
- current book reference
- reading progress
- bookmarks

### Disposable state

Disposable state includes:
- rendered page cache
- temporary layout outputs
- derived metadata that can be rebuilt

### Required behavior

- corrupt durable state falls back safely
- corrupt cache is rebuilt
- missing cache must not panic the reader

---

## 8. Sidecar Rules

Sidecar support is the only approved cross-device state exchange for v1.

### Implement only these behaviors

- export current state to canonical JSON
- import canonical JSON
- merge newer progress by timestamp
- merge bookmarks by id and timestamp
- reject mismatched fingerprint

Do not add:
- network transport
- auth
- encryption
- sync service abstractions

---

## 9. New File Checklist

Before adding a file, confirm:

1. this file is required by the current milestone
2. its ownership is clear
3. the logic cannot live cleanly in an existing file
4. the file is not being created only for hypothetical future use

If any answer is weak, do not add the file.

---

## 10. Board Glue Rules

### `boards/x4/pins.rs`
May contain:
- GPIO assignments
- ADC channels
- SPI/I2C bus pin configuration

### `boards/x4/profile.rs`
May contain:
- board constants
- display dimensions
- capability flags

### `boards/x4/board.rs`
May contain:
- board bring-up
- peripheral assembly
- board service construction

### `boards/x4/ffi.rs`
May contain:
- C SDK wrappers
- unsafe confinement

### Must not contain

- reader screen logic
- pagination logic
- settings parsing
- sidecar JSON logic
- bookmark storage rules

---

## 11. Validation Expectations

### M1 validation

- repeated boot
- repeated redraw
- repeated button navigation
- SD mount across reboots
- sleep/wake cycle testing

### M2 validation

- TXT open and paginate
- EPUB open and paginate
- save/load progress
- reboot and resume

### M3 validation

- settings persist
- recents work
- bookmarks work
- longer reading sessions are usable

### M4 validation

- invalid settings recovery
- corrupt cache recovery
- bad-book rejection without panic
- soak testing

### M5 validation

- export sidecar
- import sidecar
- merge correct newer progress
- reject wrong fingerprint

Do not claim a milestone complete without the corresponding validation.

---

## 12. Implementation Order

Use this order unless a task explicitly narrows it further:

1. repo trim and structure freeze
2. display stability
3. input stability
4. SD stability
5. battery/sleep stability
6. shell state and rendering loop
7. TXT path
8. EPUB minimal path
9. progress persistence
10. settings persistence
11. home/recents/bookmarks
12. hardening
13. sidecar support
14. fidelity upgrades

This order is intentionally conservative.

---

## 13. What Good Implementation Looks Like

Good implementation in this repo usually has these properties:

- small diff
- obvious ownership
- no speculative abstractions
- no scope widening
- explicit limitation notes
- compiles early
- keeps the X4 path easier to understand

Bad implementation usually has these properties:

- adds many files at once
- creates generic interfaces not needed now
- mixes board logic and reader logic
- quietly adds deferred features
- rewrites stable code while hardware is still unstable

---

## 14. Delivery Standard

A contribution is good enough to land when:

- it moves the current milestone forward
- it does not widen scope
- it respects file ownership boundaries
- it does not add fragile abstractions
- it clearly states what it did not solve

This repository values disciplined progress over ambitious patch size.
