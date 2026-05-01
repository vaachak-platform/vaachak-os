# Codex Prompt — Phase 30 — Vaachak Runtime Ownership Consolidation

You are working in the `vaachak-os` repository.

## Current known-good baseline

Phase 29 is working on real Xteink X4 hardware.

Latest accepted boot marker:

```text
phase29=x4-storage-path-helpers-ok
```

The current firmware flashes and boots on:

```text
ESP32-C3 revision v0.4
40 MHz crystal
16 MB flash
target: riscv32imc-unknown-none-elf
```

The active reader runtime is still the imported Pulp/X4 runtime backed by:

```text
vendor/pulp-os
vendor/smol-epub
```

Do not rewrite the reader, EPUB parser, input ladder, display driver, SD/SPI handling, or app manager.

## Goal

Implement:

```text
Phase 30 — Vaachak Runtime Ownership Consolidation
```

This phase should make the active target structure look like VaachakOS-owned code while keeping the proven imported Pulp reader runtime behavior unchanged.

Expected active boot marker:

```text
vaachak=x4-runtime-ready
```

Do not print old phase markers during normal boot.

## Required scope

Create a Vaachak-owned namespace under:

```text
target-xteink-x4/src/vaachak_x4/
```

Preferred final shape:

```text
target-xteink-x4/src/
  main.rs
  vaachak_x4/
    mod.rs
    boot.rs
    runtime.rs
    contracts/
      mod.rs
      boundary_contract.rs
      boundary_contract_smoke.rs
      storage.rs
      input.rs
      display.rs
      storage_state_contract.rs
      storage_path_helpers.rs
      input_contract_smoke.rs
      display_contract_smoke.rs
    imported/
      mod.rs
      pulp_reader_runtime.rs
```

The old `target-xteink-x4/src/runtime/` modules may be removed only after the new namespace compiles and checks pass. Avoid leaving duplicate active modules that can drift.

## Behavior constraints

Do not change physical behavior.

Do not move or rewrite:

```text
SD/SPI initialization
filesystem reads/writes
EPUB cache IO
ADC sampling
button debounce/repeat handling
SSD1677 init
display refresh
strip rendering
reader app construction
EPUB parsing/rendering
bookmark/progress IO
theme IO
```

All of that must remain owned by the imported Pulp runtime.

## Imported code constraints

Do not modify:

```text
vendor/pulp-os/**
vendor/smol-epub/**
```

The new file:

```text
target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs
```

should be copied from the existing accepted runtime entrypoint and should remain behavior-equivalent to the current imported runtime path.

Allowed changes inside the copied imported runtime wrapper:

```text
- Module path updates required by the namespace move
- Crate alias references such as pulp_os
- The single new Vaachak boot marker call
- Removing old internal phase marker print calls from the active boot path
```

## Boot marker cleanup

Normal boot should print only:

```text
vaachak=x4-runtime-ready
```

Remove or silence active printing of old markers:

```text
phase16=x4-reader-parity-ok
phase17=x4-reader-refactor-ok
phase18=x4-runtime-adapter-ok
phase19=x4-vaachak-runtime-facade-ok
phase20=x4-boundary-scaffold-ok
phase21=x4-storage-boundary-ok
phase22=x4-input-boundary-ok
phase23=x4-display-boundary-ok
phase24=x4-boundary-contract-ok
phase25=x4-storage-contract-smoke-ok
phase26=x4-input-contract-smoke-ok
phase27=x4-display-contract-smoke-ok
phase28=x4-boundary-contract-smoke-ok
phase29=x4-storage-path-helpers-ok
```

It is acceptable to keep old marker constants/helper functions for compatibility and tests, but they must not be emitted during normal boot.

## Required docs

Add:

```text
docs/phase30/PHASE30_RUNTIME_OWNERSHIP.md
docs/phase30/PHASE30_ACCEPTANCE.md
docs/phase30/PHASE30_NOTES.md
```

The docs must clearly state:

```text
- Vaachak now owns the target namespace and contract modules.
- Imported Pulp still owns the physical reader runtime behavior.
- vendor/pulp-os and vendor/smol-epub are unchanged.
- Phase 30 is not a hardware-behavior extraction.
- Future phases should move one behavior path at a time.
```

## Required scripts

Add:

```text
scripts/check_imported_reader_runtime_sync.sh
scripts/check_vaachak_x4_runtime.sh
scripts/revert_phase30_runtime_ownership.sh
```

`check_imported_reader_runtime_sync.sh` should verify that the imported runtime remains behavior-equivalent to the accepted Pulp reader runtime after allowed namespace and boot-marker normalization.

`check_vaachak_x4_runtime.sh` should verify:

```text
- cargo metadata works
- target-xteink-x4 files exist
- vaachak_x4 namespace exists
- imported Pulp runtime wrapper exists
- contracts namespace exists
- storage/input/display contract modules exist
- storage path helpers exist
- fake/raw EPUB smoke code is absent
- smol-epub path is present
- vendor/pulp-os and vendor/smol-epub were not edited
- active boot marker is vaachak=x4-runtime-ready
- old phase markers are not actively printed during normal boot
- no physical storage/input/display behavior moved into contract modules
```

The check script may skip cargo check/clippy unless:

```text
PHASE30_RUN_CARGO=1
```

is set.

## Required validation commands

Run these before final response:

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings

./scripts/check_imported_reader_runtime_sync.sh
./scripts/check_vaachak_x4_runtime.sh
```

Do not claim success unless all pass.

## Final response requirements

Report:

```text
- Files changed
- Whether vendor/pulp-os and vendor/smol-epub were untouched
- Validation commands run
- Any failures and exact next fix if not passing
```

Do not flash the device. The user will flash manually.

## Acceptance marker

Expected boot marker after user flashes:

```text
vaachak=x4-runtime-ready
```
