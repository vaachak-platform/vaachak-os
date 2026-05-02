# plans_phase35b_storage_state_io_wiring.md

# Vaachak OS Plan — Phase 35B

## Current State

Accepted through Phase 35A:

```text
Phase 30: Vaachak runtime namespace
Phase 31: Storage path helper adoption
Phase 32-34: Storage/input/display helper adoption
Phase 35A: Storage state IO seam/scaffold
```

Current normal boot marker:

```text
vaachak=x4-runtime-ready
```

## Phase 35B — Wire Storage State IO Seam Into Active Runtime Without Vendor Edits

### Goal

Connect the Vaachak storage state IO seam to the active runtime as a path-only/no-op bridge.

### Scope

```text
- Add runtime bridge module under vaachak_x4/io.
- Call the bridge from imported runtime wrapper.
- Validate Progress/Bookmark/Theme/Metadata path resolution through Vaachak seam.
- Keep physical persistence owned by Pulp runtime.
```

### Non-Scope

```text
- No vendor edits.
- No SD/SPI changes.
- No filesystem IO migration.
- No reader behavior changes.
- No EPUB cache IO changes.
```

### Acceptance

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings

./scripts/check_imported_reader_runtime_sync_phase35b.sh
./scripts/check_phase35b_storage_state_io_wiring.sh
./scripts/check_phase35b_no_vendor_or_hardware_regression.sh
```

### Expected Hardware Result

After user flashes:

```text
vaachak=x4-runtime-ready
```

TXT/EPUB, continue, bookmarks, and theme/menu/footer behavior remain unchanged.

## Later Phases

### Phase 35C — Shadow State IO Read Probe

Read-only shadow probe for existing state files, if it can be done without changing behavior.

### Phase 35D — Feature-Gated State IO Backend

Feature-gated Vaachak-owned state IO backend, disabled by default.

### Phase 36 — Input Semantic Mapping Active Adoption

Move semantic action mapping into Vaachak-owned code without changing ADC/debounce.

### Phase 37 — Display Geometry Active Adoption

Move geometry helper usage into active path without changing SSD1677 refresh/strip rendering.
