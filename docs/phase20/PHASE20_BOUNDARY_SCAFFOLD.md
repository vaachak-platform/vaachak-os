# Phase 20 — Vaachak Display/Input/Storage Boundary Scaffold

## Goal

Phase 20 creates Vaachak-owned boundary modules for the Xteink X4 target without moving any working hardware or reader behavior out of the imported X4/Pulp runtime.

This phase is intentionally a scaffold. It is not a driver rewrite.

## Added modules

```text
target-xteink-x4/src/runtime/display_boundary.rs
target-xteink-x4/src/runtime/input_boundary.rs
target-xteink-x4/src/runtime/storage_boundary.rs
```

## Current ownership

The imported Pulp runtime remains authoritative for:

```text
SSD1677 display initialization and refresh
Shared SPI bus ownership
microSD / FAT storage behavior
button ladder / power-button input behavior
ReaderApp / FilesApp / AppManager construction
TXT / EPUB progress and bookmarks
Reader footer/menu/theme/continue behavior
```

## Vaachak-owned scaffold

The new boundary modules record:

```text
Display pins: EPD_CS GPIO21, DC GPIO4, RST GPIO5, BUSY GPIO6, SPI GPIO8/10/7
Input pins: row ladder GPIO1/GPIO2, power GPIO3
Storage pin: SD_CS GPIO12 and shared display SPI bus note
```

## Boot marker

Phase 20 adds:

```text
phase20=x4-boundary-scaffold-ok
```

The marker is emitted through the existing Phase 19 Vaachak facade path. This confirms the boundary scaffold is linked into the firmware without moving behavior.

## Non-goals

Phase 20 does not:

```text
Move SSD1677 code
Move SPI code
Move SD code
Move ADC/input code
Modify vendor/pulp-os
Modify vendor/smol-epub
Change reader UX
Rename branding/UI strings
```
