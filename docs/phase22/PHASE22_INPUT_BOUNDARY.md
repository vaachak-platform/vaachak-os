# Phase 22 — Vaachak Input Boundary Extraction

## Purpose

Phase 22 makes the Xteink X4 input contract explicit in Vaachak-owned code without changing the working imported Pulp/X4 runtime.

The active physical input path remains imported:

```text
vendor/pulp-os imported runtime
  -> ADC ladder reads
  -> power button GPIO reads
  -> debounce/repeat/long-press handling
  -> reader/menu action routing
```

Vaachak now owns metadata and helper contracts for future extraction:

```text
target-xteink-x4/src/runtime/input_boundary.rs
```

## Marker

```text
phase22=x4-input-boundary-ok
```

## X4 input metadata recorded

```text
ROW1_ADC_GPIO=1
ROW2_ADC_GPIO=2
POWER_BUTTON_GPIO=3
```

## Button roles recorded

```text
Back
Select
Up
Down
Left
Right
Power
```

## Not moved in Phase 22

```text
ADC reads
ADC ladder thresholds/calibration
debounce/repeat behavior
long-press behavior
reader/menu event routing
```

These remain in the imported Pulp runtime until a later behavior-preserving extraction phase.
