# Phase 35 — Physical Behavior Extraction Plan

## Purpose

Phase 35 starts the physical behavior extraction track safely.

It does not move all physical behavior at once. It creates guardrails and begins with the lowest-risk area: Storage State IO.

## Accepted Baseline

Phase 32–34 is accepted on hardware.

Normal boot marker:

```text
vaachak=x4-runtime-ready
```

TXT and EPUB reader behavior works.

## Full Extraction Sequence

```text
1. Storage state IO
2. Input semantic mapping
3. Display geometry helper usage
4. Input ADC/debounce
5. SD/SPI arbitration
6. SSD1677 refresh/strip rendering
7. Reader app internals
```

## Phase 35 Scope

Phase 35 is limited to:

```text
- extraction plan
- risk register
- guardrail scripts
- storage state IO seam/scaffold
```

## Explicit Non-Scope

Phase 35 must not move:

```text
Input ADC/debounce
SD/SPI arbitration
SSD1677 refresh/strip rendering
reader app internals
EPUB cache IO
reader parsing/rendering behavior
```

## Ownership Model

Vaachak owns:

```text
target-xteink-x4/src/vaachak_x4/contracts/*
target-xteink-x4/src/vaachak_x4/io/*, if introduced
```

Imported Pulp still owns:

```text
physical storage runtime
physical input runtime
physical display runtime
reader app internals
```
