# Codex Prompt — Phase 35B — Wire Storage State IO Seam Into Active Runtime Without Vendor Edits

You are working in the `vaachak-os` repository.

## Current known-good baseline

Phase 35A is accepted at the software level and should be hardware-validated by the user.

Current normal boot marker:

```text
vaachak=x4-runtime-ready
```

Current active target namespace:

```text
target-xteink-x4/src/vaachak_x4/
```

Current imported runtime wrapper:

```text
target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs
```

Current storage IO seam files from Phase 35A:

```text
target-xteink-x4/src/vaachak_x4/io/mod.rs
target-xteink-x4/src/vaachak_x4/io/storage_state.rs
target-xteink-x4/src/vaachak_x4/io/storage_state_adapter.rs
```

The device reader behavior before this phase is known-good:

```text
TXT opens
EPUB/EPU opens with real text through smol-epub
Continue works
Bookmarks work
Theme/menu/footer behavior works
```

## Goal

Implement:

```text
Phase 35B — Wire Storage State IO Seam Into Active Runtime Without Vendor Edits
```

This phase should connect the Vaachak-owned storage state IO seam to the active runtime **without editing vendored code** and **without replacing persistence behavior yet**.

Think of Phase 35B as a runtime hook/bridge phase.

It must prove the active runtime can reach the Vaachak storage state IO seam and can validate state path resolution for the currently supported state record kinds:

```text
Progress
Bookmark
Theme
Metadata
```

## Critical distinction

Phase 35B should wire the seam into active runtime, but it should **not** take over physical persistence yet.

Allowed:

```text
- Add a Vaachak-owned runtime storage-state bridge/hook module.
- Call that bridge from the active imported runtime wrapper.
- Perform pure/path-only preflight validation using the seam and storage path helpers.
- Use no-op or path-probe backends to validate the seam shape.
- Keep physical SD/SPI/FAT IO owned by the imported Pulp runtime.
```

Forbidden:

```text
- Editing vendor/pulp-os/**
- Editing vendor/smol-epub/**
- Moving SD/SPI initialization
- Moving filesystem open/read/write/close calls
- Replacing Pulp progress/bookmark/theme persistence
- Changing EPUB cache IO
- Changing reader app internals
- Changing reader behavior
- Printing phase marker spam at boot
```

## Expected boot marker

Normal boot must continue to emit only:

```text
vaachak=x4-runtime-ready
```

Do not print:

```text
phase35=
phase35b=
```

You may keep Phase 35B constants in docs/tests/checks, but normal boot should not emit them.

## Preferred implementation

Add a new module:

```text
target-xteink-x4/src/vaachak_x4/io/storage_state_runtime.rs
```

Suggested public API:

```rust
pub struct VaachakStorageStateRuntimeBridge;

impl VaachakStorageStateRuntimeBridge {
    pub fn active_runtime_preflight() -> bool {
        // Exercise VaachakStorageStatePaths / VaachakStorageStateIoAdapter
        // in path-only/no-op mode for:
        // Progress, Bookmark, Theme, Metadata.
        // No SD/FAT IO.
    }
}
```

Then update:

```text
target-xteink-x4/src/vaachak_x4/io/mod.rs
```

to export:

```rust
pub mod storage_state_runtime;
```

Then update:

```text
target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs
```

to call the bridge silently at startup, near existing Vaachak helper adoption probe calls:

```rust
let _ = crate::vaachak_x4::io::storage_state_runtime::VaachakStorageStateRuntimeBridge::active_runtime_preflight();
```

The call must not alter persistence behavior.

## Acceptable alternative

If a better non-vendor runtime hook exists in the active wrapper, use it.

If no safe hook exists, do not force the change. Add docs explaining why active persistence wiring must wait until a later phase.

But the preferred outcome is a silent path-only runtime bridge call from the active wrapper.

## Required docs

Add:

```text
docs/phase35b/PHASE35B_STORAGE_STATE_IO_WIRING.md
docs/phase35b/PHASE35B_ACCEPTANCE.md
docs/phase35b/PHASE35B_NOTES.md
docs/phase35b/PHASE35B_WIRING_OPTIONS.md
```

Docs must explicitly say:

```text
- Phase 35B wires a Vaachak-owned storage state IO seam into active runtime as a path-only/no-op preflight.
- Phase 35B does not replace progress/bookmark/theme persistence.
- Physical SD/SPI/FAT IO remains owned by the imported Pulp runtime.
- vendor/pulp-os and vendor/smol-epub are untouched.
- Normal boot remains vaachak=x4-runtime-ready only.
```

## Required scripts

Add:

```text
scripts/check_imported_reader_runtime_sync_phase35b.sh
scripts/check_phase35b_storage_state_io_wiring.sh
scripts/check_phase35b_no_vendor_or_hardware_regression.sh
scripts/revert_phase35b_storage_state_io_wiring.sh
```

The scripts should validate:

```text
- Cargo metadata works.
- target-xteink-x4 compiles.
- Clippy passes.
- Vaachak storage state seam files exist.
- New runtime bridge exists.
- active runtime wrapper calls the Phase 35B bridge.
- bridge references Progress, Bookmark, Theme, Metadata.
- bridge uses VaachakStorageStatePaths and/or VaachakStorageStateIoAdapter.
- bridge does not own physical SD/SPI/FAT IO.
- active runtime wrapper does not add physical IO.
- fake EPUB smoke code is absent.
- normal boot marker remains vaachak=x4-runtime-ready.
- old phase markers are not printed during normal boot.
- vendor/pulp-os and vendor/smol-epub have no tracked edits.
- imported reader runtime sync passes after allowed Vaachak normalization.
```

## Required validation commands

Run these before final response:

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings

./scripts/check_imported_reader_runtime_sync_phase35b.sh
./scripts/check_phase35b_storage_state_io_wiring.sh
./scripts/check_phase35b_no_vendor_or_hardware_regression.sh
```

Do not claim success unless all pass.

## Hardware behavior acceptance

Do not flash.

The user will flash manually.

Expected after user flash:

```text
vaachak=x4-runtime-ready
```

Reader behavior must remain unchanged:

```text
TXT opens
EPUB/EPU opens with real text
Continue works
Bookmarks work
Theme/menu/footer behavior unchanged
```

## Final response requirements

Report:

```text
- Files changed
- Whether vendor/pulp-os and vendor/smol-epub were untouched
- Whether normal boot marker remains vaachak=x4-runtime-ready
- Whether old phase markers remain silenced
- Whether the bridge is path-only/no-op or active persistence
- Validation commands run
- Any failures and exact next fix if not passing
```

## Stop conditions

Stop and report instead of guessing if:

```text
- wiring requires editing vendor/pulp-os or vendor/smol-epub
- wiring requires changing filesystem IO
- wiring requires changing progress/bookmark/theme persistence
- wiring requires changing EPUB cache IO
- wiring changes reader behavior
- imported runtime sync cannot be normalized safely
```
