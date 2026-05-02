# AGENTS.md Addendum — Phase 35+ Physical Behavior Extraction

Append this section to the repository `AGENTS.md`.

---

## Phase 35+ — Physical Behavior Extraction Guardrails

### Accepted Baseline

Phase 32–34 is accepted on real Xteink X4 hardware.

Normal boot marker:

```text
vaachak=x4-runtime-ready
```

TXT and EPUB reader behavior is confirmed working.

### Extraction Sequence

The physical behavior extraction sequence is:

```text
1. Storage state IO
2. Input semantic mapping
3. Display geometry helper usage
4. Input ADC/debounce
5. SD/SPI arbitration
6. SSD1677 refresh/strip rendering
7. Reader app internals
```

Do not combine all of these into one implementation phase.

### Phase 35 Scope

Phase 35 is limited to:

```text
Physical behavior extraction plan
Storage state IO seam/scaffold
Guardrail scripts
Docs/risk register
```

Phase 35 may define a Vaachak-owned storage state IO trait/interface and adapter scaffolding.

Phase 35 must not move SD/SPI/FAT/EPUB cache IO unless an explicit later prompt allows it.

### Vendor Rule

Do not modify:

```text
vendor/pulp-os/**
vendor/smol-epub/**
```

Before reporting success, verify:

```bash
git diff --quiet -- vendor/pulp-os vendor/smol-epub
```

### Normal Boot Marker Policy

Normal boot must continue to emit only:

```text
vaachak=x4-runtime-ready
```

Do not reintroduce old phase marker logs.

### Hard Non-Scope for Phase 35

Do not move or rewrite:

```text
Input ADC sampling
button debounce/repeat handling
SD/SPI arbitration
SSD1677 init
SSD1677 refresh/strip rendering
reader app internals
EPUB parser/rendering
TXT reader behavior
EPUB cache IO
```

### Allowed in Phase 35

Allowed:

```text
- Add docs/phase35.
- Add check scripts for physical extraction guardrails.
- Add Vaachak-owned storage state IO trait/interface.
- Add storage state adapter scaffold if it does not perform physical IO.
- Add host-side tests for pure state/path helpers.
- Make a tiny imported runtime call only if it is a no-op probe or a clearly isolated non-behavior-changing seam.
```

### Required Validation

Run:

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

### Stop Conditions

Stop and report if the implementation requires:

```text
vendor edits
filesystem IO changes
SD/SPI behavior changes
EPUB cache IO changes
reader app internals changes
SSD1677 refresh/rendering changes
ADC/debounce changes
```
