# AGENTS.md — Vaachak OS

This file provides instructions for coding agents working in the `vaachak-os` repository.

Keep changes small, validated, and hardware-safe.

## Project Summary

Vaachak OS is an embedded Rust reader OS effort for the Xteink X4 / ESP32-C3 e-paper device.

Current accepted hardware target:

```text
Board/device: Xteink X4
MCU: ESP32-C3 revision v0.4
Crystal: 40 MHz
Flash: 16 MB
Display: SSD1677 e-paper, 800x480 native
Rust target: riscv32imc-unknown-none-elf
```

The current working reader runtime comes from imported X4/Pulp code under:

```text
vendor/pulp-os
vendor/smol-epub
```

Do not rewrite or casually refactor the imported reader runtime. It currently provides the known-good EPUB/TXT reader behavior.

## Current Development State

Latest accepted phase before Phase 30:

```text
Phase 29 — Storage Path Helpers
Accepted boot marker:
phase29=x4-storage-path-helpers-ok
```

Phase 30 goal:

```text
Phase 30 — Vaachak Runtime Ownership Consolidation
Expected boot marker:
vaachak=x4-runtime-ready
```

## Hard Rules

Do not modify:

```text
vendor/pulp-os/**
vendor/smol-epub/**
```

unless the prompt explicitly says to patch vendored code.

Do not change physical hardware behavior unless the prompt explicitly says so.

Do not move or rewrite these during Phase 30:

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

Do not reintroduce the fake EPUB smoke reader. These strings must never appear in active source:

```text
run_epub_reader_page_storage_smoke
ZIP container parsed
First readable bytes
ensure_pulp_dir_async
```

## Expected Target Structure After Phase 30

Preferred structure:

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

The `vaachak_x4` namespace is Vaachak-owned.

The `vaachak_x4/imported/pulp_reader_runtime.rs` file is an imported-runtime wrapper and should remain behavior-equivalent to the previously accepted Pulp runtime.

## Boot Marker Policy

For Phase 30, normal boot should emit only:

```text
vaachak=x4-runtime-ready
```

Old phase markers may remain as constants or test fixtures, but they must not be printed during normal boot.

Old markers include:

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

## Rust Validation Commands

Always run:

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings
```

For Phase 30 also run:

```bash
./scripts/check_imported_reader_runtime_sync.sh
./scripts/check_vaachak_x4_runtime.sh
```

Do not report success until these pass.

## Clippy Rules

`cargo clippy ... -- -D warnings` must pass.

Avoid adding broad crate-level allows. If scaffold code is intentionally unused, prefer one of:

```rust
#[allow(dead_code)]
```

on the specific module or item, or actually use the item in the contract smoke.

## Documentation Rules

When adding phase docs, use:

```text
docs/phaseXX/
```

For Phase 30, add:

```text
docs/phase30/PHASE30_RUNTIME_OWNERSHIP.md
docs/phase30/PHASE30_ACCEPTANCE.md
docs/phase30/PHASE30_NOTES.md
```

Docs should explain the boundary clearly:

```text
Vaachak owns the target namespace and contracts.
Imported Pulp owns the working physical reader behavior.
No hardware behavior moved in Phase 30.
```

## Script Rules

Scripts must be safe to rerun.

Scripts should create backups before destructive changes.

Scripts should fail clearly with actionable messages.

Do not depend on network access.

## Final Response Requirements

When done, report:

```text
- Files changed
- Validation commands run
- Whether vendor/pulp-os and vendor/smol-epub remained untouched
- Whether old phase boot markers are silenced
- Remaining risks or follow-up if any
```

Do not claim device success. The user performs flashing and hardware validation.

# AGENTS.md Addendum — Phase 31

Append this section to the repository `AGENTS.md`.

---

## Phase 31 — Active Storage Path Helper Adoption

### Accepted Baseline

Phase 30 is accepted on real Xteink X4 hardware.

Normal boot marker:

```text
vaachak=x4-runtime-ready
```

TXT and EPUB reader behavior is confirmed working after Phase 30.

### Phase 31 Goal

Phase 31 adopts Vaachak-owned pure storage path/name helpers in the active runtime path where safe.

This is not a physical storage extraction.

### Allowed Changes

Allowed:

```text
- Add or refine Vaachak-owned pure storage path helper functions.
- Add host-side tests for pure helper behavior.
- Update the imported runtime wrapper only to call Vaachak-owned pure helper functions where mechanically equivalent.
- Add Phase 31 docs and checks.
- Keep normal boot marker as vaachak=x4-runtime-ready.
```

### Forbidden Changes

Do not modify:

```text
vendor/pulp-os/**
vendor/smol-epub/**
```

Do not move or rewrite:

```text
SD card initialization
SPI bus setup
SPI bus sharing/arbitration
filesystem open/read/write/close
progress file IO
bookmark file IO
theme file IO
EPUB cache IO
ADC sampling
display refresh
reader app construction
EPUB parsing/rendering
TXT reader behavior
```

### Boot Marker Policy

Normal boot must continue to print only:

```text
vaachak=x4-runtime-ready
```

Do not reintroduce old phase marker logs.

Old phase strings may remain as constants or docs, but must not be actively printed during normal boot.

### Required Checks

Run:

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings

./scripts/check_imported_reader_runtime_sync_phase31.sh
./scripts/check_phase31_storage_path_adoption.sh
```

### Fake EPUB Smoke Ban

Active source must not contain:

```text
run_epub_reader_page_storage_smoke
ZIP container parsed
First readable bytes
ensure_pulp_dir_async
```

### Vendor Rule

Before reporting success, verify:

```bash
git diff --quiet -- vendor/pulp-os vendor/smol-epub
```

No tracked vendor edits are allowed in Phase 31.

### Stop Conditions

Stop and report if adopting Vaachak helpers would require changing:

```text
filesystem IO
SD/SPI behavior
EPUB cache IO
reader behavior
vendor code
```

Phase 31 should move only pure deterministic path/name helper ownership.
