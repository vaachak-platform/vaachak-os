# Phase 35+ Next Phases

## Phase 36 — Input Semantic Mapping Active Path

Before moving to input work, decide whether the Phase 35 storage state IO seam
should be wired into active persistence. Do this only if a narrow path-level
adapter can be introduced without editing vendor code or changing reader state
formats.

Make active runtime use Vaachak-owned input semantic mapping.

Do not move ADC sampling or debounce.

## Phase 37 — Display Geometry Active Path

Make active runtime use Vaachak-owned display geometry helpers.

Do not move SSD1677 refresh or strip rendering.

## Phase 38 — Input ADC/Debounce

Move physical input reading only after semantic mapping is stable.

## Phase 39 — SD/SPI Arbitration

Move shared bus ownership only after storage state IO and display behavior are stable.

## Phase 40 — SSD1677 Refresh/Strip Rendering

Move display driver behavior only after geometry adoption has been validated.

## Phase 41+ — Reader App Internals

Move reader internals last.
