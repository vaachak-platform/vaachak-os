# Vaachak OS Architecture

## Product architecture

Vaachak OS targets the Xteink X4 as a reader-first firmware. The main Home dashboard remains a Biscuit-style launcher, while all internal configuration, list, and app pages use CrossInk-style layout conventions: compact header, tab strip where useful, large readable rows, right-aligned values, footer-safe content, and shared button hints.

## Active crates

```text
core/                 format-neutral models and contracts
hal-xteink-x4/        X4-facing hardware seams
target-xteink-x4/     production X4 firmware target
support/vaachak-lua-vm optional hostable Lua VM bridge
```

`target-xteink-x4/src/vaachak_x4/**` is the active runtime and app implementation area.

## Hardware target

```text
MCU:       ESP32-C3
Display:   4.26 inch 800 × 480 e-paper panel
Controller: SSD1677
Flash:     16 MB
Storage:   microSD on shared SPI bus
Input:     X4 hardware buttons
```

The repository preserves the X4/CrossPoint-compatible partition table:

```text
nvs       0x009000  0x005000
otadata   0x00e000  0x002000
app0      0x010000  0x640000
app1      0x650000  0x640000
spiffs    0xc90000  0x360000
coredump  0xff0000  0x010000
```

## Runtime ownership

Vaachak-owned runtime code handles Home, app dispatch, reader UI, files, settings, network screens, Wi-Fi Transfer, Date & Time, Lua app surfaces, and CrossInk-style internal UI. `vendor/pulp-os` may remain as reference or compatibility material, but new functionality belongs in Vaachak-owned paths.

`vendor/smol-epub` remains the EPUB dependency source and is intentionally excluded from the workspace.

## UI model

The UI has two layers:

```text
Home dashboard:  Biscuit-style category launcher
Internal pages:  CrossInk-style chrome and list geometry
```

Internal pages use a fixed Inter UI font path with e-ink weight calibration. Reader/book font settings remain separate and do not change OS chrome.

Covered internal surfaces:

- Settings
- Reader tabs: Recent, Books, Files, Bookmarks
- Files and Bookmarks lists
- Network / Wi-Fi / Transfer / Time / Status
- Dictionary
- Panchang
- Daily Mantra
- Combined Calendar
- Games catalog and game shells
- Sleep Image and Device Info

## Reader model

The reader path supports local TXT and EPUB files with persistent progress, bookmarks, prepared cache metadata, title cache, and reader settings. Reader page rendering remains separate from internal app chrome so pagination and display timing can stay optimized for reading.

Reader state and cache files live on SD-card paths used by the X4 firmware. Logical reader state is intended to become the basis for future XTC and `.vchk` support.

## Lua app model

Lua apps are optional SD-loaded apps rooted at:

```text
/VAACHAK/APPS
```

Physical folders remain uppercase and 8.3-safe where practical. Native firmware features remain native unless intentionally moved behind the bounded Lua app model.

Current app-data examples include:

```text
/VAACHAK/APPS/CALENDAR
/VAACHAK/APPS/DICT
/VAACHAK/APPS/MANTRA
/VAACHAK/APPS/PANCHANG
```

## Network model

Network pages are Vaachak-owned. Wi-Fi setup reads and writes the existing settings file path used by the firmware. Wi-Fi Transfer supports nested SD uploads, and Date & Time maintains live/cached/unsynced status for screens such as Calendar and Daily Mantra.

## Firmware artifact model

The repository builds the embedded ELF with Cargo and can generate a full X4 flash image with `espflash save-image`. The full image is intended for GitHub artifacts and new-device installation. Source-tree flashing remains available with the existing flash scripts.
