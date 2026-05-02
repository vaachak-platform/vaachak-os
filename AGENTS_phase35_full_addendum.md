# AGENTS.md Addendum — Phase 35 Full Physical Behavior Extraction

Append this section to the repository `AGENTS.md` before running Codex for Phase 35 Full.

---

## Phase 35 Full — Vaachak-Owned Physical Runtime Extraction

### Non-negotiable success rule

This phase is successful only if all seven behavior areas are actively moved into Vaachak-owned runtime code:

```text
1. Storage state IO
2. Input semantic mapping
3. Display geometry helper usage
4. Input ADC/debounce
5. SD/SPI arbitration
6. SSD1677 refresh/strip rendering
7. Reader app internals
```

A scaffold, preflight, no-op bridge, or docs-only implementation is a failure.

### Accepted baseline

The accepted boot marker before this phase is:

```text
vaachak=x4-runtime-ready
```

TXT and EPUB behavior works on Xteink X4 hardware.

### Expected Phase 35 Full boot marker

Normal boot should emit only:

```text
vaachak=x4-physical-runtime-owned
```

Do not emit old phase markers during normal boot.

### Vendor policy

Do not edit:

```text
vendor/pulp-os/**
vendor/smol-epub/**
```

You may copy code from `vendor/pulp-os` into Vaachak-owned code under:

```text
target-xteink-x4/src/vaachak_x4/
```

Copied code must become the active runtime path.

### Required ownership

Active runtime must be Vaachak-owned for:

```text
state IO
input event semantics
input ADC/debounce
shared SD/display SPI bus arbitration
SSD1677 init/refresh/strip rendering
reader app internals
```

`vendor/smol-epub` may remain the EPUB parser dependency.

### Failure conditions

Stop and report failure if any of these are true:

```text
- one or more of the seven areas remains only in imported Pulp runtime
- the active runtime still calls Pulp app manager/reader as a black box
- only probes/scaffolds were added
- vendor/pulp-os or vendor/smol-epub has tracked edits
- cargo check/clippy fails
- any Phase 35 Full check script fails
```

### Required validation

Run:

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

Do not report success unless all pass.
