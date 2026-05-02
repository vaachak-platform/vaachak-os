# Codex Prompt — Phase 35+ — Physical Behavior Extraction Roadmap and Phase 35 Storage-State IO Seam

You are working in the `vaachak-os` repository.

## Current accepted baseline

Phase 32–34 has been accepted on real Xteink X4 hardware.

Normal boot marker:

```text
vaachak=x4-runtime-ready
```

The device boots, and TXT/EPUB behavior works as expected.

Current architecture:

```text
target-xteink-x4/src/
  main.rs
  vaachak_x4/
    boot.rs
    runtime.rs
    contracts/
      storage_path_helpers.rs
      storage_state_contract.rs
      input_semantics.rs
      display_geometry.rs
      ...
    imported/
      pulp_reader_runtime.rs
vendor/
  pulp-os/
  smol-epub/
```

The imported Pulp/X4 runtime remains the known-good physical runtime. `vendor/pulp-os` and `vendor/smol-epub` must remain untouched unless the prompt explicitly says otherwise.

## Phase 35+ strategic scope

The overall physical behavior extraction sequence is:

```text
1. Storage state IO
2. Input semantic mapping
3. Display geometry helper usage
4. Input ADC/debounce
5. SD/SPI arbitration
6. SSD1677 refresh/strip rendering
7. Reader app internals
```

Do not implement all seven items in one step.

Phase 35 must establish the extraction plan, guardrails, checks, and the first safe extraction seam for **Storage State IO** only.

## Phase 35 goal

Implement:

```text
Phase 35 — Physical Behavior Extraction Plan + Storage State IO Seam
```

Phase 35 should create the infrastructure needed to start physical behavior extraction safely, and should extract only the smallest safe storage-state IO boundary if a non-invasive seam exists.

Normal boot marker must remain:

```text
vaachak=x4-runtime-ready
```

Do not reintroduce phase marker spam during normal boot.

## Hard constraints

Do not modify:

```text
vendor/pulp-os/**
vendor/smol-epub/**
```

Do not break:

```text
TXT reader
EPUB reader
Continue behavior
Bookmarks
Theme/menu/footer behavior
Input navigation
Display orientation/refresh
```

Do not move these in Phase 35:

```text
Input ADC sampling
button debounce/repeat handling
SD/SPI bus arbitration
SSD1677 init
SSD1677 refresh/strip rendering
reader app internals
EPUB parser/rendering
```

## What Phase 35 may change

Allowed:

```text
- Add Vaachak-owned storage IO trait/interface definitions.
- Add Vaachak-owned storage state IO adapter scaffolding.
- Add compile-time wiring that can be used later by the imported runtime.
- Add pure host-side tests for path/state IO contracts.
- Add docs and scripts for the full Phase 35+ extraction sequence.
- Add checks proving only the intended seam exists and no risky behavior moved.
```

Conditionally allowed:

```text
- If the imported runtime already has a small, isolated state/progress/bookmark/theme IO call site that can call a Vaachak-owned helper without changing physical file IO behavior, wire it through a thin adapter.
- If the call site is not obvious or touches broad reader internals, do not wire it. Add the seam and document the next required manual decision instead.
```

Forbidden:

```text
- Do not replace Pulp's SD card driver.
- Do not replace the filesystem volume manager.
- Do not change SPI pins or device ownership.
- Do not change EPUB cache IO.
- Do not change reader app construction.
- Do not change bookmark/progress file formats.
- Do not change user-visible reader behavior.
```

## Required files to add

Add docs:

```text
docs/phase35/PHASE35_PHYSICAL_EXTRACTION_PLAN.md
docs/phase35/PHASE35_STORAGE_STATE_IO_SEAM.md
docs/phase35/PHASE35_ACCEPTANCE.md
docs/phase35/PHASE35_RISK_REGISTER.md
docs/phase35/PHASE35_NEXT_PHASES.md
```

Add scripts:

```text
scripts/check_phase35_physical_extraction_plan.sh
scripts/check_phase35_storage_state_io_seam.sh
scripts/check_phase35_no_hardware_regression.sh
scripts/check_imported_reader_runtime_sync_phase35.sh
scripts/revert_phase35_storage_state_io_seam.sh
```

Add code only if safe:

```text
target-xteink-x4/src/vaachak_x4/io/mod.rs
target-xteink-x4/src/vaachak_x4/io/storage_state.rs
target-xteink-x4/src/vaachak_x4/io/storage_state_adapter.rs
```

If `io/` is added, update:

```text
target-xteink-x4/src/vaachak_x4/mod.rs
```

## Storage state IO seam design

The storage state IO seam should separate:

```text
Vaachak-owned semantic state paths/formats
  from
Pulp-owned physical SD/FAT/filesystem operations
```

Define a trait or equivalent abstraction for state IO. It should be small and explicit.

Recommended shape:

```rust
pub enum VaachakStateIoKind {
    Progress,
    Bookmark,
    Theme,
    Metadata,
}

pub trait VaachakStorageStateIo {
    type Error;

    fn read_state(&mut self, book_id: &[u8], kind: VaachakStateIoKind, out: &mut [u8]) -> Result<usize, Self::Error>;
    fn write_state(&mut self, book_id: &[u8], kind: VaachakStateIoKind, data: &[u8]) -> Result<(), Self::Error>;
}
```

Adjust for `no_std` and existing project style.

Do not require allocation unless already accepted in target code.

Use existing helpers from:

```text
target-xteink-x4/src/vaachak_x4/contracts/storage_path_helpers.rs
```

for path/name construction.

## Required checks

`check_phase35_physical_extraction_plan.sh` should verify:

```text
- docs/phase35 exist
- Phase 35+ extraction sequence is documented
- all seven physical behavior extraction areas are listed
- Phase 35 is limited to Storage State IO seam
- normal boot marker remains vaachak=x4-runtime-ready
```

`check_phase35_storage_state_io_seam.sh` should verify:

```text
- storage state IO seam files exist if implementation is added
- seam uses storage_path_helpers
- seam references Progress, Bookmark, Theme, Metadata
- seam does not own SD/SPI/FAT operations
- no EPUB cache IO moved
```

`check_phase35_no_hardware_regression.sh` should verify active Vaachak-owned code does not newly own:

```text
Adc::new
read_oneshot
button debounce/repeat loops
spi::master
RefCellDevice
SdCard::new
AsyncVolumeManager
SSD1677 init/refresh/write_cmd/write_data
strip rendering
draw_/refresh()
```

`check_imported_reader_runtime_sync_phase35.sh` should compare the active imported runtime wrapper against the vendored Pulp main after allowing:

```text
x4_os:: -> pulp_os::
Vaachak namespace path differences
vaachak=x4-runtime-ready marker
calls to pure Vaachak helper/adoption probes
optional call to Vaachak storage state IO seam only if actually wired
old phase marker print removal/silencing
```

## Required validation commands

Run before final response:

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings

./scripts/check_imported_reader_runtime_sync_phase35.sh
./scripts/check_phase35_physical_extraction_plan.sh
./scripts/check_phase35_storage_state_io_seam.sh
./scripts/check_phase35_no_hardware_regression.sh
```

Do not claim success unless all pass.

## Final response requirements

Report:

```text
- Files changed
- Whether vendor/pulp-os and vendor/smol-epub were untouched
- Whether normal boot marker remains vaachak=x4-runtime-ready
- Whether storage state IO seam was implemented or only scaffolded
- Whether any imported runtime call site was changed
- Validation commands run
- Any failures and exact next fix if not passing
```

Do not flash the device. The user will flash manually.

## Stop conditions

Stop and report instead of guessing if:

```text
- storage state IO extraction requires editing vendor/pulp-os
- storage state IO extraction requires changing EPUB cache IO
- storage state IO extraction touches SD/SPI arbitration
- storage state IO extraction changes reader app internals
- imported runtime sync cannot be normalized safely
```
