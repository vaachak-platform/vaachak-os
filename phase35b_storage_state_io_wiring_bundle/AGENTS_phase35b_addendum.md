# AGENTS.md Addendum — Phase 35B

Append this section to the repository `AGENTS.md`.

---

## Phase 35B — Wire Storage State IO Seam Into Active Runtime Without Vendor Edits

### Accepted Baseline

Phase 35A added the Vaachak-owned storage state IO seam.

Normal boot marker remains:

```text
vaachak=x4-runtime-ready
```

Reader behavior is known-good:

```text
TXT opens
EPUB/EPU opens with real text
Continue works
Bookmarks work
Theme/menu/footer behavior works
```

### Phase 35B Goal

Phase 35B wires the storage state IO seam into the active runtime as a safe runtime bridge.

This phase must not replace physical persistence.

### Allowed Changes

Allowed:

```text
- Add `target-xteink-x4/src/vaachak_x4/io/storage_state_runtime.rs`.
- Add a path-only/no-op runtime bridge.
- Call the bridge from `target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs`.
- Exercise Progress, Bookmark, Theme, and Metadata path resolution.
- Add docs and checks.
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
reader app construction
EPUB parsing/rendering
TXT reader behavior
```

### Boot Marker Policy

Normal boot must continue to print only:

```text
vaachak=x4-runtime-ready
```

Do not add normal boot output for:

```text
phase35=
phase35b=
```

### Required Checks

Run:

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings

./scripts/check_imported_reader_runtime_sync_phase35b.sh
./scripts/check_phase35b_storage_state_io_wiring.sh
./scripts/check_phase35b_no_vendor_or_hardware_regression.sh
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

No tracked vendor edits are allowed in Phase 35B.

### Stop Conditions

Stop and report if wiring requires changing:

```text
filesystem IO
SD/SPI behavior
EPUB cache IO
reader behavior
vendor code
```

Phase 35B should only prove that the active runtime can reach Vaachak-owned storage state IO seam through a safe path-only runtime bridge.
