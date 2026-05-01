# Vaachak OS Plans

## Current Track

```text
Xteink X4 / ESP32-C3 Vaachak OS extraction from imported Pulp reader runtime
```

## Accepted Milestones

```text
Phase 16: Reader parity using imported Pulp/smol-epub runtime
Phase 17: Reader runtime refactor and repo hygiene
Phase 18: Runtime adapter
Phase 19: Vaachak runtime facade
Phase 20: Display/input/storage boundary scaffold
Phase 21: Storage boundary metadata
Phase 22: Input boundary metadata
Phase 23: Display boundary metadata
Phase 24: Unified boundary contract
Phase 25: Storage contract smoke
Phase 26: Input contract smoke
Phase 27: Display contract smoke
Phase 28: Unified boundary contract smoke
Phase 29: Storage path helpers and old phase-marker cleanup
```

Latest accepted boot marker:

```text
phase29=x4-storage-path-helpers-ok
```

## Phase 30 — Vaachak Runtime Ownership Consolidation

### Goal

Make the active target code look and feel like VaachakOS while keeping the working imported Pulp reader runtime behavior unchanged.

Expected boot marker:

```text
vaachak=x4-runtime-ready
```

### Scope

Create:

```text
target-xteink-x4/src/vaachak_x4/
```

with:

```text
boot.rs
runtime.rs
contracts/
imported/
```

Move the Vaachak-owned contract modules into the new namespace.

Move the active imported runtime wrapper into:

```text
target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs
```

Keep behavior equivalent to Phase 29.

### Explicit Non-Scope

Do not move:

```text
SD/SPI initialization
filesystem IO
EPUB cache IO
ADC sampling
button debounce/repeat
SSD1677 init
display refresh
strip rendering
reader app construction
EPUB parsing
bookmarks/progress/theme IO
```

### Acceptance

Commands must pass:

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings

./scripts/check_imported_reader_runtime_sync.sh
./scripts/check_vaachak_x4_runtime.sh
```

Expected boot marker after flash:

```text
vaachak=x4-runtime-ready
```

### Risk

Main risk is breaking the known-good imported runtime while moving files.

Mitigation:

```text
- Move namespace first.
- Keep imported runtime behavior-equivalent.
- Normalize and compare imported runtime with scripts.
- Avoid hardware behavior extraction.
```

## Phase 31 — Use Vaachak Storage Path Helpers in Active Runtime

Planned after Phase 30.

Goal:

```text
Start using Vaachak-owned path helpers from the active runtime for pure state path/name construction.
```

Still do not move physical SD/SPI/file IO.

## Phase 32 — State/Progress/Bookmark Path Ownership

Goal:

```text
Make Vaachak-owned state path construction the active source of truth for progress/bookmark/theme/metadata names.
```

## Phase 33 — Input Semantic Mapping Ownership

Goal:

```text
Move semantic button/action mapping into Vaachak-owned code while leaving ADC sampling and debounce in imported runtime.
```

## Phase 34 — Display Geometry Helper Ownership

Goal:

```text
Move display geometry/rotation helper usage into Vaachak-owned code while leaving SSD1677 refresh and strip rendering in imported runtime.
```

## Phase 35+ — Physical Behavior Extraction

Do this one behavior path at a time.

Suggested order:

```text
1. Storage state IO
2. Input semantic mapping
3. Display geometry helper usage
4. Input ADC/debounce
5. SD/SPI arbitration
6. SSD1677 refresh/strip rendering
7. Reader app internals
```
