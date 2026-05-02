# Phase 35D-0 Notes

Phase 35C-1 showed that wrapping the active app layer can break input behavior even when state logic appears isolated. Phase 35D-0 deliberately avoids the active runtime hook and keeps the imported Pulp app manager path unchanged.

This phase is useful because it gives Vaachak-owned code the same progress and bookmark record vocabulary as the working reader while preserving the known-good physical runtime.

Deferred work:

- active progress `.PRG` IO takeover
- active bookmark `.BKM` IO takeover
- active bookmark index `BMIDX.TXT` IO takeover
- reader-app integration of the Vaachak facade

Those steps should be performed as small hardware-validation phases, not by wrapping the entire app manager.
