# Phase 35C-0 Notes

## Why This Exists

The active Pulp reader owns theme and metadata persistence inside private reader
methods. A clean active IO move needs Vaachak-owned record formats first, then a
small reader slice extraction.

## What Did Not Move

```text
progress .PRG IO
bookmark .BKM IO
bookmark index BMIDX.TXT IO
theme .THM active IO
metadata .MTA active IO
SD/SPI/FAT operations
reader app internals
```

## Next Step

Phase 35C should wire active `.THM` and `.MTA` read/write behavior through the
Vaachak facade once the required reader slice can be owned outside vendored
code.
