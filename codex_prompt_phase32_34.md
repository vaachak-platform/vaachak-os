# Codex Prompt — Phase 32–34 — Active Helper Adoption Consolidation

You are working in the `vaachak-os` repository.

## Current known-good baseline

Phase 31 is accepted on real Xteink X4 hardware.

Accepted active boot marker:

```text
vaachak=x4-runtime-ready
```

The device boots and TXT/EPUB reader behavior still works.

Current architecture:

```text
target-xteink-x4/src/
  main.rs
  vaachak_x4/
    boot.rs
    runtime.rs
    contracts/
    imported/
      pulp_reader_runtime.rs
```

The imported reader runtime remains behavior-equivalent to the Pulp/X4 runtime and uses `vendor/pulp-os` plus `vendor/smol-epub`.

## Goal

Implement a bundled helper-adoption phase:

```text
Phase 32–34 — Active Helper Adoption Consolidation
```

This combines the next three planned low-risk steps:

```text
Phase 32 — State/progress/bookmark path helper ownership in active runtime
Phase 33 — Input semantic mapping helper ownership in active runtime
Phase 34 — Display geometry helper ownership in active runtime
```

This phase should make the active runtime call Vaachak-owned **pure helper/contract logic** for storage paths, input semantics, and display geometry where safe, while preserving all physical behavior in the imported Pulp runtime.

## Expected boot marker

Do not reintroduce phase-marker spam.

Normal boot must continue to emit only:

```text
vaachak=x4-runtime-ready
```

Do not add normal boot markers like `phase32=...`, `phase33=...`, or `phase34=...`.

It is acceptable to add Phase 32–34 constants/check strings in docs or tests, but they must not print during normal boot.

## Required scope

Use or extend Vaachak-owned pure helper modules under:

```text
target-xteink-x4/src/vaachak_x4/contracts/
```

Expected modules after this phase:

```text
target-xteink-x4/src/vaachak_x4/contracts/storage_path_helpers.rs
target-xteink-x4/src/vaachak_x4/contracts/storage_state_contract.rs
target-xteink-x4/src/vaachak_x4/contracts/input_semantics.rs
target-xteink-x4/src/vaachak_x4/contracts/display_geometry.rs
```

If different names are already present and better aligned with the repo, keep them, but the check scripts must be updated accordingly.

## Phase 32 scope — storage path helper adoption

Make Vaachak-owned storage path/name helpers the active pure helper source for:

```text
state directory name
progress file name/path
bookmark file name/path
theme file name/path
metadata file name/path
bookmark index filename
reserved state file identification
book id validation
state extension validation
```

Allowed active-runtime change:

```text
Call Vaachak-owned pure helper functions from pulp_reader_runtime.rs where mechanically safe.
```

Do not change filesystem IO or state file contents.

## Phase 33 scope — input semantic helper adoption

Add or refine:

```text
target-xteink-x4/src/vaachak_x4/contracts/input_semantics.rs
```

It should define pure helper/contract logic for:

```text
physical input pins: GPIO1, GPIO2, GPIO3
button roles: Back, Select, Up, Down, Left, Right, Power
reader actions: BackToLibrary, OpenOrSelect, NextPage, PreviousPage, BookmarkOrMenu
navigation actions: Up, Down, Left, Right
```

Allowed active-runtime change:

```text
Call a pure input semantic adoption probe or validation helper from the active imported runtime.
```

Do not move ADC sampling, debounce, repeat handling, ladder thresholds, or event polling.

## Phase 34 scope — display geometry helper adoption

Add or refine:

```text
target-xteink-x4/src/vaachak_x4/contracts/display_geometry.rs
```

It should define pure helper/contract logic for:

```text
native geometry: 800x480
logical portrait geometry: 480x800
rotation/orientation contract
strip row assumptions
SSD1677 command constants: 0x24, 0x26, 0x22, 0x20
X4 display pins: CS=21, DC=4, RST=5, BUSY=6
shared SPI pins: SCLK=8, MOSI=10, MISO=7
```

Allowed active-runtime change:

```text
Call a pure display geometry adoption probe or validation helper from the active imported runtime.
```

Do not move SSD1677 init, SPI transactions, refresh, strip rendering, LUT behavior, or framebuffer behavior.

## Behavior constraints

Do not move physical storage/input/display behavior.

Do not move or rewrite:

```text
SD card initialization
SPI bus setup
SPI bus sharing/arbitration
filesystem open/read/write/close
EPUB cache IO
progress file read/write IO
bookmark file read/write IO
theme file read/write IO
ADC sampling
button debounce/repeat handling
button ladder thresholds
input event polling
SSD1677 init
SSD1677 commands over SPI
display refresh
strip rendering
reader app construction
EPUB parser/rendering
TXT reader logic
```

All physical behavior must remain in the imported Pulp runtime.

## Imported code constraints

Do not modify:

```text
vendor/pulp-os/**
vendor/smol-epub/**
```

The active wrapper:

```text
target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs
```

may be changed only as needed to call Vaachak-owned pure helper/probe functions.

Do not rewrite the imported runtime.

Do not change reader behavior.

## Safe implementation model

Preferred implementation:

```text
1. Add pure helper modules for input semantics and display geometry.
2. Refine storage_path_helpers only for pure helper completeness.
3. Add one combined active adoption probe, or three small pure probes:
   - storage path helper adoption
   - input semantic helper adoption
   - display geometry helper adoption
4. Call the pure probe(s) from the active imported runtime at boot.
5. Keep normal boot output limited to vaachak=x4-runtime-ready.
```

The adoption probes must not print. They should return `bool` or a small pure report struct.

## Required docs

Add:

```text
docs/phase32_34/PHASE32_34_ACTIVE_HELPER_ADOPTION.md
docs/phase32_34/PHASE32_34_ACCEPTANCE.md
docs/phase32_34/PHASE32_34_NOTES.md
```

Docs must state:

```text
- Vaachak now owns pure helper contracts for storage path names, input semantics, and display geometry.
- Physical SD/SPI/filesystem/input/display behavior still belongs to the imported Pulp runtime.
- This phase does not move physical hardware behavior.
- Normal boot still emits only vaachak=x4-runtime-ready.
```

## Required scripts

Add:

```text
scripts/check_imported_reader_runtime_sync_phase32_34.sh
scripts/check_phase32_34_active_helper_adoption.sh
scripts/revert_phase32_34_active_helper_adoption.sh
```

`check_phase32_34_active_helper_adoption.sh` should verify:

```text
- cargo metadata works
- vaachak_x4 namespace exists
- imported runtime wrapper exists
- storage path helper module exists
- input semantic helper module exists
- display geometry helper module exists
- active imported runtime calls Vaachak-owned pure helper/probe logic
- fake/raw EPUB smoke code is absent
- normal boot marker remains vaachak=x4-runtime-ready
- old phase markers are not printed during normal boot
- vendor/pulp-os and vendor/smol-epub have no tracked edits
- no physical SD/SPI/file IO moved into storage helpers
- no ADC/debounce/threshold/event polling moved into input helpers
- no SSD1677/SPI/refresh/rendering moved into display helpers
- imported reader runtime sync check passes
```

`check_imported_reader_runtime_sync_phase32_34.sh` should compare the active imported runtime wrapper against the vendored Pulp main after allowing:

```text
x4_os:: -> pulp_os::
Vaachak namespace path differences
vaachak=x4-runtime-ready marker
calls to pure Vaachak storage/input/display helper functions
removal/silencing of old phase marker prints
```

## Required validation commands

Run before final response:

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings

./scripts/check_imported_reader_runtime_sync_phase32_34.sh
./scripts/check_phase32_34_active_helper_adoption.sh
```

Do not claim success unless all pass.

## Hardware behavior acceptance

Do not flash.

The user will flash manually.

Expected after user flash:

```text
vaachak=x4-runtime-ready
```

TXT and EPUB reader behavior must remain unchanged.

## Final response requirements

Report:

```text
- Files changed
- Whether vendor/pulp-os and vendor/smol-epub were untouched
- Whether normal boot marker remains vaachak=x4-runtime-ready
- Whether old phase markers remain silenced
- Whether active runtime now calls Vaachak-owned pure storage/input/display helper probes
- Validation commands run
- Any failures and exact next fix if not passing
```

## Hard stop conditions

Stop and report instead of guessing if:

```text
- adopting helpers requires changing filesystem IO
- adopting helpers requires changing ADC sampling/debounce
- adopting helpers requires changing SSD1677/SPI/refresh behavior
- adopting helpers requires changing EPUB cache IO
- adopting helpers requires editing vendor/pulp-os or vendor/smol-epub
- imported runtime sync cannot be normalized safely
```
