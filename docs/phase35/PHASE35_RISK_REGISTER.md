# Phase 35 Risk Register

## Storage State IO

Risk: medium.

Failure modes:

```text
progress not restored
bookmarks not loaded/saved
theme state lost
state filename mismatch
```

Mitigation:

```text
only introduce seam first
keep physical IO in imported runtime
use existing storage path helpers
do not wire imported runtime call sites until a narrow non-invasive path exists
```

Phase 35 status:

```text
seam scaffold implemented
physical IO not moved
imported runtime call sites not changed
next manual decision required before active persistence wiring
```

## Input Semantic Mapping

Risk: medium.

Failure modes:

```text
wrong button action
Back/Select swapped
page navigation broken
```

Mitigation:

```text
move semantic mapping before ADC/debounce
preserve imported physical input runtime
```

## Display Geometry Helper Usage

Risk: medium-high.

Failure modes:

```text
rotation regression
footer/header misplacement
strip boundary errors
```

Mitigation:

```text
move pure geometry before SSD1677 rendering
keep refresh behavior imported
```

## Input ADC/Debounce

Risk: high.

Failure modes:

```text
missed button presses
repeated presses
wrong ladder thresholds
```

Mitigation:

```text
defer until semantic mapping is stable
add hardware test matrix
```

## SD/SPI Arbitration

Risk: very high.

Failure modes:

```text
SD read failures
display corruption
bus contention
boot hang
```

Mitigation:

```text
defer until storage seam is stable
change one bus owner at a time
```

## SSD1677 Refresh/Strip Rendering

Risk: very high.

Failure modes:

```text
inverted display
clipped display
strip artifacts
refresh flicker
blank screen
```

Mitigation:

```text
move last among hardware boundaries
preserve known-good commands and strip math
```

## Reader App Internals

Risk: very high.

Failure modes:

```text
EPUB rendering regression
bookmark/progress regression
reader menu regression
cache regression
```

Mitigation:

```text
move after storage/input/display behavior is stable
```
