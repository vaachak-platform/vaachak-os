# vaachak-os

Vaachak OS is currently an Xteink X4-first reader OS built on a proven Pulp-derived runtime. The repository keeps the working device firmware path intact while Vaachak-owned core, HAL, and runtime-contract crates mature around it.

## Current firmware truth

The active firmware path for the physical Xteink X4 is:

```text
vendor/pulp-os
```

That runtime currently owns:

- ESP32-C3 boot and scheduler integration
- SSD1677 display behavior
- button/input handling
- SD/FAT file access
- Reader, Home, Settings, Wi-Fi Transfer, Date & Time, and sleep-image behavior

The root workspace currently owns extracted and target-neutral architecture work:

- `core/` — shared reader, storage, state, and text/domain models
- `hal-xteink-x4/` — X4 HAL seams and smoke helpers
- `target-xteink-x4/` — emerging X4 adapter and contract code
- `tools/` — host-side cache/font/title-map utilities
- `docs/` — current architecture, build, and operating notes

This means the repository is not pretending the root workspace is already the complete flashed firmware. The working product remains the Pulp-derived runtime until a path is deliberately extracted and validated on device.

## Reader-first baseline

The active product baseline is:

- Home dashboard with category navigation
- Reader/library path for TXT and EPUB smoke usage
- prepared cache support under `/FCACHE/<BOOKID>`
- useful cache-open diagnostics only on failure
- Wi-Fi Transfer with original transfer and chunked-resume upload paths
- Settings persistence through `/_X4/SETTINGS.TXT`
- Date & Time sync as an explicit isolated Wi-Fi mode
- sleep-image mode selection and cached sleep image support

Prepared TXT/EPUB caches are still important for mixed-script books that need host-generated glyph runs. When a prepared cache opens successfully, Reader shows the prepared-page status. If cache open fails, Reader keeps the compact diagnostic code so the loader step can be fixed.

## Repository policy

- Keep `vendor/pulp-os` behavior stable unless the change is a narrowly scoped bug fix or dead-code cleanup.
- Do not add historical delivery labels, generated patch directories, generated zip files, or local backup folders to source control.
- Keep old local output outside the repo or under ignored paths.
- Prefer semantic names in code, docs, scripts, and logs.
- Move one real behavior path at a time from the Pulp-derived runtime into Vaachak-owned crates.

## Build and flash

See:

```text
docs/development/build-and-flash.md
```

## Current architecture notes

See:

```text
docs/architecture/current-runtime.md
docs/architecture/ownership-map.md
ROADMAP.md
SCOPE.md
```
