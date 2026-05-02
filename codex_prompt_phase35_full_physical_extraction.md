# Codex Prompt — Phase 35 Full Physical Behavior Extraction

You are working in the `vaachak-os` repository.

## Current accepted baseline

Phases 30–35B have been accepted on Xteink X4 hardware.

Normal boot marker currently remains:

```text
vaachak=x4-runtime-ready
```

TXT and EPUB behavior is working.

Current architecture has a Vaachak-owned namespace:

```text
target-xteink-x4/src/vaachak_x4/
```

The current active runtime still depends on an imported Pulp reader/runtime wrapper for most physical behavior.

## Goal

Implement a single full physical behavior extraction phase:

```text
Phase 35 Full — Vaachak-Owned Physical Runtime Extraction
```

This deliverable is considered successful only if **all seven behavior areas below are actively moved into Vaachak-owned code and wired into the normal runtime path**.

A scaffold, probe, no-op bridge, docs-only plan, or partial movement is a failure.

## Required scope — all must be implemented

The implementation must actively cover all of these:

```text
1. Storage state IO
2. Input semantic mapping
3. Display geometry helper usage
4. Input ADC/debounce
5. SD/SPI arbitration
6. SSD1677 refresh/strip rendering
7. Reader app internals
```

## Success criteria

Codex must not claim success unless all of the following are true:

### 1. Storage state IO

Vaachak-owned code must perform active state IO for:

```text
progress .PRG
bookmarks .BKM
theme .THM
metadata .MTA
bookmark index BMIDX.TXT
```

The active reader runtime must use Vaachak-owned state IO in normal reader behavior.

Path-only probes or no-op preflight calls do not satisfy this criterion.

### 2. Input semantic mapping

Vaachak-owned code must convert raw physical button events into reader/navigation semantics.

At minimum, active runtime must use Vaachak-owned mapping for:

```text
Back
Select/Open
Up
Down
Left
Right
Power
Next page
Previous page
Menu/Bookmark where applicable
```

Contract-only definitions or tests do not satisfy this criterion.

### 3. Display geometry helper usage

Active rendering/display layout must use Vaachak-owned geometry helpers for:

```text
native 800x480
logical portrait 480x800
rotation/orientation
strip height/strip range mapping
footer/content bounds where applicable
```

Unused helpers or probes do not satisfy this criterion.

### 4. Input ADC/debounce

Vaachak-owned code must own active physical input sampling/debounce/repeat handling.

At minimum, active code must handle:

```text
GPIO1 ADC ladder
GPIO2 ADC ladder
GPIO3 power button
sampling
threshold classification
debounce
repeat/hold policy if present in imported runtime
```

Leaving physical input handling entirely in imported Pulp code is a failure.

### 5. SD/SPI arbitration

Vaachak-owned code must own active SD/display shared SPI arbitration.

At minimum, active code must define and use a Vaachak-owned SPI/shared-bus manager or equivalent for:

```text
SCLK GPIO8
MOSI GPIO10
MISO GPIO7
EPD CS GPIO21
SD CS GPIO12
transaction ownership between SD and display
```

Merely documenting pins or using Pulp's arbitration unchanged is a failure.

### 6. SSD1677 refresh/strip rendering

Vaachak-owned code must own active SSD1677 display behavior:

```text
SSD1677 init/config sequence
current RAM command 0x24
previous RAM command 0x26
refresh control 0x22
master activate 0x20
strip rendering path
busy wait/release handling
active refresh call path
```

Leaving SSD1677 refresh/strip rendering in imported Pulp code is a failure.

### 7. Reader app internals

Vaachak-owned code must own active reader app internals:

```text
home/files/library app flow
reader app entry
TXT/MD reader path
EPUB/EPU reader path using smol-epub
progress/continue behavior
bookmarks behavior
reader menu/footer behavior
theme behavior
```

The active runtime must not merely call Pulp's app manager/reader app as an imported black box.

## Vendor policy

Do not modify:

```text
vendor/pulp-os/**
vendor/smol-epub/**
```

You may copy code from `vendor/pulp-os` into Vaachak-owned modules under:

```text
target-xteink-x4/src/vaachak_x4/
```

Copied code must be adapted so that the active runtime is Vaachak-owned.

`vendor/smol-epub` may remain the EPUB parser dependency.

## Required final structure

Implement or adapt an equivalent Vaachak-owned structure under:

```text
target-xteink-x4/src/vaachak_x4/
  physical/
    mod.rs
    runtime.rs
    storage_state_io.rs
    input_adc.rs
    input_semantics_runtime.rs
    spi_bus.rs
    ssd1677_display.rs
    display_geometry_runtime.rs
  apps/
    mod.rs
    app_manager.rs
    home.rs
    files.rs
    reader.rs
    reader_state.rs
    settings.rs
  ui/
    mod.rs
  imported/
    mod.rs
```

The exact internal module split can vary, but these ownership concepts must exist and be active.

## Boot marker

Normal boot should emit one marker:

```text
vaachak=x4-physical-runtime-owned
```

Do not emit old phase markers during normal boot.

## Prohibited success claims

Do not claim success if:

```text
- only a seam/scaffold/probe was added
- any of the seven behavior areas remain solely owned by imported Pulp runtime
- the active runtime still delegates app/reader/hardware behavior to Pulp as a black box
- vendor/pulp-os or vendor/smol-epub has tracked edits
- cargo check/clippy fails
- check scripts fail
```

If all seven behavior areas cannot be completed in one pass, stop and report failure clearly. Do not present a partial implementation as Phase 35 Full.

## Required docs

Add or update:

```text
docs/phase35_full/PHASE35_FULL_PHYSICAL_EXTRACTION.md
docs/phase35_full/PHASE35_FULL_ACCEPTANCE.md
docs/phase35_full/PHASE35_FULL_OWNERSHIP_MATRIX.md
docs/phase35_full/PHASE35_FULL_RISK_REGISTER.md
docs/phase35_full/PHASE35_FULL_DEVICE_TEST_PLAN.md
```

Docs must explicitly describe how each of the seven areas is now Vaachak-owned and active.

## Required scripts

Add:

```text
scripts/check_phase35_full_physical_extraction.sh
scripts/check_phase35_full_runtime_ownership.sh
scripts/check_phase35_full_no_vendor_edits.sh
scripts/check_phase35_full_no_scaffold_only.sh
scripts/check_phase35_full_device_acceptance_notes.sh
scripts/revert_phase35_full_physical_extraction.sh
```

The scripts must fail if any of the seven behavior areas are not actively owned by Vaachak code.

## Required validation commands

Run before final response:

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings

./scripts/check_phase35_full_no_vendor_edits.sh
./scripts/check_phase35_full_runtime_ownership.sh
./scripts/check_phase35_full_no_scaffold_only.sh
./scripts/check_phase35_full_physical_extraction.sh
./scripts/check_phase35_full_device_acceptance_notes.sh
```

Do not claim success unless all pass.

## Final response requirements

Report:

```text
- Files changed
- For each of the seven areas, the active Vaachak-owned module and call path
- Confirmation that vendor/pulp-os and vendor/smol-epub were untouched
- Confirmation that normal boot marker is vaachak=x4-physical-runtime-owned
- Validation commands run
- Any failures
```

Do not flash the device. The user will flash manually.
