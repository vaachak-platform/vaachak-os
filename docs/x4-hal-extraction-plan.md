# VaachakOS Bootstrap Phase 2 — Real X4 HAL Extraction Plan

**Status:** Phase 2 planning deliverable  
**Repository:** `vaachak-os`  
**Source of hardware truth:** `x4-reader-os-rs` proving-ground  
**Target crates:** `hal-xteink-x4`, `target-xteink-x4`, and `core` HAL traits

---

## 1. Purpose

Phase 1 established a clean VaachakOS workspace, core models, storage layouts, and CI.

Phase 2 decides **how** to extract the real Xteink X4 hardware implementation without destabilizing either repository.

This phase is a planning and traceability phase. It does **not** move SSD1677, SD/FAT, input ladder, or boot runtime code yet.

---

## 2. Decision summary

VaachakOS should extract real X4 HAL code in this order:

1. **Power and battery facts** — smallest surface, low runtime risk.
2. **Input ladder mapping** — already proven and testable with thresholds.
3. **Storage probe/mount contract** — interface first, then real SD adapter.
4. **Display geometry and strip rendering contract** — model first, driver later.
5. **SSD1677 display implementation** — last among HAL pieces because it is most failure-prone.
6. **Target runtime boot glue** — after HAL seams are proven.

Do not start by moving `main.rs`, the executor, AppManager, ReaderApp, or SSD1677 driver wholesale.

---

## 3. Extraction principles

### 3.1 Preserve proving-ground truth

`x4-reader-os-rs` remains the fastest place to verify real hardware behavior.

Do not delete or reshape working X4 code there just because VaachakOS now exists.

### 3.2 Extract contracts before implementations

Each hardware area must first have:

- a core trait contract
- a HAL wrapper shape
- a source map
- acceptance logs from real hardware
- tests for stable facts

Only then should implementation code move.

### 3.3 Avoid architecture cosplay

If a seam cannot yet be validated on real X4 hardware, keep it as a documented placeholder.

### 3.4 Keep app policy out of HAL

The HAL maps hardware into neutral events and device capabilities.

The HAL must not know about:

- Reader actions
- Home menu labels
- bookmark behavior
- theme behavior
- TXT/EPUB policy

---

## 4. Phase 2 deliverables

This phase adds:

- source file map from `x4-reader-os-rs` to VaachakOS crates
- extraction order and branch sequence
- hardware validation matrix
- GitHub task template for extraction work
- ADR for the extraction policy

---

## 5. Target module ownership

### 5.1 `core`

Owns traits and neutral models:

```text
core/src/hal/display.rs
core/src/hal/input.rs
core/src/hal/power.rs
core/src/hal/storage.rs
core/src/models/*
```

Allowed here:

- geometry structs
- display frame/strip contracts
- button event types
- storage error and read/write traits
- battery reading models
- capability flags

Not allowed here:

- X4 GPIO numbers
- ADC threshold constants
- SSD1677 command sequences
- SPI bus speed changes specific to X4
- FAT/SD card implementation details

### 5.2 `hal-xteink-x4`

Owns X4 hardware behavior:

```text
hal-xteink-x4/src/display.rs
hal-xteink-x4/src/input.rs
hal-xteink-x4/src/power.rs
hal-xteink-x4/src/storage.rs
```

Allowed here:

- SSD1677 display adapter
- X4 ADC ladder decoder
- GPIO3 power button behavior
- SD card over shared SPI behavior
- X4 battery divider logic
- X4 capability constants

Not allowed here:

- Reader bookmark policy
- Home/Files navigation
- EPUB parsing
- chapter/page calculations

### 5.3 `target-xteink-x4`

Owns boot/runtime wiring:

```text
target-xteink-x4/src/main.rs
```

Allowed here:

- allocator setup
- executor startup
- concrete HAL construction
- boot console wiring
- task spawn wiring
- handoff into VaachakOS runtime

Not allowed here:

- reader business logic
- storage record formats
- app menu state machines

---

## 6. Recommended extraction branch plan

### Branch A — `extract/x4-power-hal`

Goal:

- Port only power/battery model details.
- Keep it host-testable.
- No embedded runtime dependency yet.

Acceptance:

- `cargo test --workspace` passes.
- X4 battery percentage tests match proving-ground behavior.
- No target boot change.

### Branch B — `extract/x4-input-hal`

Goal:

- Port ADC ladder thresholds and GPIO3 power button mapping.
- Keep semantic output as neutral `InputEvent`.

Acceptance:

- Threshold tests cover bottom-left cluster.
- Power button precedence is preserved.
- No app-level Reader actions appear in HAL.

### Branch C — `extract/x4-storage-contract`

Goal:

- Expand `StorageHal` only enough to support real app state persistence later.
- Preserve `X4CompatFlat83` layout.

Acceptance:

- mock storage tests pass.
- no real FAT/SD adapter yet unless source map is complete.
- nested path support remains a capability flag, not an assumption.

### Branch D — `extract/x4-display-contract`

Goal:

- Lock geometry, strip rendering, refresh modes, and bus timing model.
- Do not port SSD1677 implementation yet.

Acceptance:

- tests assert native 800x480 panel and logical portrait behavior.
- no full framebuffer assumption is introduced.

### Branch E — `extract/x4-display-driver`

Goal:

- Port real SSD1677 implementation behind `DisplayHal`.
- Keep strip rendering.

Acceptance:

- minimal test firmware can render boot/status screen.
- no Reader migration yet.
- real device logs captured.

### Branch F — `extract/x4-target-boot`

Goal:

- Port target startup shell and instantiate HAL.

Acceptance:

- target crate can build for the embedded target.
- boot console or minimal display proof runs.
- app runtime still minimal.

---

## 7. Non-goals for Phase 2

Do not implement:

- real Reader migration
- Home/Files migration
- EPUB parser migration
- sync
- highlights
- Waveshare target
- desktop simulator
- WebDAV/OPDS
- advanced typography

---

## 8. Phase 2 exit criteria

Phase 2 is complete when:

- the file source map is committed
- the validation matrix is committed
- the extraction ADR is committed
- each HAL seam has a first implementation issue/task
- no runtime code was moved prematurely
- CI remains green

---

## 9. Next phase recommendation

Bootstrap Phase 3 should be:

```text
VaachakOS Bootstrap Phase 3 — X4 Input + Power HAL Extraction
```

Reason:

- input/power are lower risk than display/storage
- they produce useful real-HAL code quickly
- they preserve the working reader/runtime path in the proving-ground
