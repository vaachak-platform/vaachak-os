# Hardware Runtime Executor Live Handoff

This document describes the accepted live handoff checkpoint for the Vaachak hardware runtime executor.

Canonical cleanup checkpoint:

- `docs/architecture/hardware-runtime-executor-live-handoff-cleanup.md`

## Scope

Live runtime handoff is wired for:

- boot preflight
- imported Pulp reader runtime boundary
- storage availability handoff
- display refresh handoff
- input runtime handoff

The low-level backend remains `PulpCompatibility` / Pulp-compatible. This checkpoint does not rewrite physical SPI transfer, SD/MMC, FAT, SSD1677 display, button ADC/debounce/navigation, reader UX, file-browser UX, or app navigation behavior.

## Marker

```text
hardware_runtime_executor_live_path_handoff=ok
```
