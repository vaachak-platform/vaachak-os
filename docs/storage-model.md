# Storage Model

## Current rule

The active X4 runtime stores reader state in flat, 8.3-safe files under `state/`. This layout is intentionally conservative because it has been reliable on the device.

## State files

| File | Purpose |
| --- | --- |
| `state/<BOOKID>.PRG` | Reading progress. |
| `state/<BOOKID>.BKM` | Per-book bookmarks. |
| `state/<BOOKID>.THM` | Per-book theme settings. |
| `state/<BOOKID>.MTA` | Per-book metadata. |
| `state/BMIDX.TXT` | Bookmark index. |

## Compatibility

Long names and generated cache paths should map to stable 8-character book IDs before writing reader state. The root workspace can own deterministic naming helpers, but physical SD/FAT behavior remains in the active runtime until a device-validated extraction is ready.

## Prepared cache

Prepared caches live under `/FCACHE/<BOOKID>`. Large cache uploads should use Wi-Fi Transfer chunked resume.
