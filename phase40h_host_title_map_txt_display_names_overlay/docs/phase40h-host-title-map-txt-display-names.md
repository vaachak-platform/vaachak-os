# Phase 40H — FAT Long Filename / Host Title Map for TXT Display Names

Phase 40H is the safe replacement for TXT body-title guessing.

The host creates `_X4/TITLEMAP.TSV` from SD root long filenames. The firmware loads
this title map before `_X4/TITLES.BIN`.

Each line is:

```text
DEVICE_VISIBLE_FILENAME<TAB>Friendly Display Title
```

Example:

```text
POIROT~1.TXT	Poirot Investigates
THEMUR~1.TXT	The Murders in the Rue Morgue
```
