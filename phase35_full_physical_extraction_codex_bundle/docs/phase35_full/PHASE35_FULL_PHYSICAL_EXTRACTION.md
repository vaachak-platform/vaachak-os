# Phase 35 Full — Vaachak-Owned Physical Runtime Extraction

## Purpose

Phase 35 Full moves all physical runtime behavior ownership into Vaachak-owned code.

This is different from Phase 35A/35B, which only added seams and preflight bridges.

## Required behavior areas

```text
1. Storage state IO
2. Input semantic mapping
3. Display geometry helper usage
4. Input ADC/debounce
5. SD/SPI arbitration
6. SSD1677 refresh/strip rendering
7. Reader app internals
```

All seven must be active in Vaachak-owned code.

## Expected marker

```text
vaachak=x4-physical-runtime-owned
```

## Vendor policy

Do not edit:

```text
vendor/pulp-os
vendor/smol-epub
```

Code may be copied into Vaachak-owned modules, but the active runtime must live under:

```text
target-xteink-x4/src/vaachak_x4
```
