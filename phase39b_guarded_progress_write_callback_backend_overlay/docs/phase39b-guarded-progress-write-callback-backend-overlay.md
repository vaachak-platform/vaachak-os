# Phase 39B — Guarded Progress Write Callback Backend Overlay

This phase makes the Phase 39A progress-write binding usable by adding a
callback-backed backend adapter.

It can dispatch a guarded `.PRG` write through a caller-supplied callback.

Scope:
- `.PRG` progress state only
- callback backend only
- no `.THM`, `.MTA`, `.BKM`, or `BMIDX.TXT` writes yet
- no hard-coded SD/FAT implementation

Expected marker:

```text
phase39b=x4-guarded-progress-write-callback-backend-ok
```
