# Vaachak OS Scope

## In scope now

- Stable Xteink X4 reader experience using the Pulp-derived runtime.
- Home dashboard, Reader, Settings, Wi-Fi Transfer, Date & Time, and sleep-image flows already present in the active runtime.
- Prepared reader cache support for large and mixed-script books.
- Host-side tools for prepared cache generation, title-cache generation, and sleep-image assets.
- Vaachak-owned models and contracts that can be adopted without disturbing working hardware behavior.

## In scope next

- Repository cleanup and current-runtime documentation.
- Reader stabilization across TXT, prepared TXT, and mixed EPUB smoke files.
- Wi-Fi Transfer hardening for large `/FCACHE/<BOOKID>` uploads.
- Date & Time reliability, including live/cached/unsynced state display.
- Settings-to-reader preference consistency.
- Careful extraction of target-neutral state, title, and prepared-cache logic.

## Explicitly deferred

- Cloud sync.
- Crypto or signed package handling.
- Advanced EPUB fidelity beyond the current smoke path.
- Generic desktop simulator.
- Waveshare target support.
- Full app sandboxing.
- Rewriting display, SPI, SD, or button hardware behavior outside the proven runtime.
