# Phase 35+ Physical Behavior Extraction Plan

## Accepted Baseline

Phase 32–34 accepted on Xteink X4 hardware.

Normal boot marker:

```text
vaachak=x4-runtime-ready
```

TXT and EPUB behavior works.

## Overall Sequence

```text
1. Storage state IO
2. Input semantic mapping
3. Display geometry helper usage
4. Input ADC/debounce
5. SD/SPI arbitration
6. SSD1677 refresh/strip rendering
7. Reader app internals
```

## Phase 35 — Storage State IO Seam

Goal:

```text
Create Vaachak-owned storage state IO boundary without moving physical SD/SPI/FAT behavior.
```

Deliverables:

```text
docs/phase35/*
scripts/check_phase35_*.sh
target-xteink-x4/src/vaachak_x4/io/storage_state.rs, if safe
```

Acceptance:

```text
cargo fmt/check/clippy pass
phase35 checks pass
vendor untouched
normal boot marker remains vaachak=x4-runtime-ready
TXT/EPUB behavior unchanged after user flash
```

## Phase 36 — Input Semantic Mapping Active Path

Goal:

```text
Make active runtime use Vaachak-owned input semantic mapping, while leaving ADC sampling/debounce in imported runtime.
```

## Phase 37 — Display Geometry Active Path

Goal:

```text
Make active runtime use Vaachak-owned display geometry helpers, while leaving SSD1677 refresh/strip rendering in imported runtime.
```

## Phase 38 — Input ADC/Debounce Extraction

Goal:

```text
Move input ADC/debounce only after semantic mapping has proven stable.
```

Risk:

```text
High: button ladder calibration and repeat behavior can regress navigation.
```

## Phase 39 — SD/SPI Arbitration Extraction

Goal:

```text
Move shared bus ownership/arbitration only after storage state IO boundaries are stable.
```

Risk:

```text
Very high: display and SD share SPI bus.
```

## Phase 40 — SSD1677 Refresh/Strip Rendering Extraction

Goal:

```text
Move display refresh/strip rendering only after display geometry helpers are active and stable.
```

Risk:

```text
Very high: orientation, strip windows, refresh commands, and visible artifacts can regress.
```

## Phase 41+ — Reader App Internals

Goal:

```text
Move reader app internals last, after storage/input/display behavior is owned by Vaachak.
```
