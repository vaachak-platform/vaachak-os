# Phase 35D-1 - Reader State Runtime Bridge

Phase 35D-1 wires the Vaachak-owned reader state facade into the active boot path as a silent, format-only runtime preflight after the heap allocator is installed.

The bridge validates that active runtime code can reach Vaachak-owned contracts for:

- progress records
- per-book bookmark records
- bookmark index records
- theme records
- metadata records
- helper-backed state filenames for `.PRG`, `.BKM`, `.THM`, `.MTA`, and `BMIDX.TXT`

This phase does not replace active persistence. The imported Pulp reader still owns runtime file reads and writes for progress, bookmarks, theme, metadata, and bookmark index behavior.

The allocation-free storage state runtime preflight still runs during the earliest boot marker section. The reader-state bridge is reached through a separate storage-state alloc preflight after `esp_alloc::heap_allocator!` has installed heap memory, keeping the active runtime wrapper free of a new app-layer wrapper.

Normal boot remains:

```text
vaachak=x4-runtime-ready
```

No phase marker is printed during normal boot.
