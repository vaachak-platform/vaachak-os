# Controlled Extraction Ownership

This document records the current controlled extraction state after the documentation reset.

## Current posture

The active X4 runtime is still the imported Pulp-derived runtime. Vaachak-owned code has accumulated useful contracts, models, adapters, layout helpers, and preflight probes, but the uploaded code does not yet remove the imported runtime from the active device path.

## Vaachak-owned extracted areas

Current extracted areas that remain useful:

- reader state and progress/bookmark models
- book identity and title-cache helpers
- prepared-cache metadata models
- input semantic mapping
- storage path helpers
- Wi-Fi Transfer configuration models
- display geometry and drawing metadata
- Biscuit-style UI layout helpers
- font/glyph/text helpers
- sleep-image and daily mantra helpers
- Date & Time status helpers
- runtime adapter and boundary contracts
- SPI bus runtime metadata

## Imported runtime areas still active

The imported runtime still executes:

- board initialization
- display driver behavior
- input sampling and input task
- SD/FAT behavior
- Reader app behavior
- Home/Files/Settings behavior
- Wi-Fi runtime behavior
- worker tasks and kernel run loop

## Non-goals for this checkpoint

This docs checkpoint does not move runtime behavior. It updates documentation to match the uploaded code state and removes stale transition-era documents.

## Future extraction principle

Move behavior only when there is:

1. a Vaachak-owned implementation,
2. a validator proving the intended boundary,
3. a device smoke test proving no product regression,
4. clear rollback or fallback notes.

The next product work should focus on Reader Home, Library, Resume, and the reader data model before lower-level hardware migrations are resumed.
