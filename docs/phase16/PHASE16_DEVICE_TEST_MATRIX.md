# Phase 16 Device Test Matrix

Use this matrix when testing on the physical Xteink X4.

| ID | Test | Input | Expected serial/display result |
|---|---|---|---|
| P16-001 | Boot marker | Power on / flash monitor | Serial contains `phase16=x4-reader-parity-ok`. |
| P16-002 | TXT open | Select `.TXT`/`.MD` | Reader page shows plain text. |
| P16-003 | TXT progress | Page forward, back, reopen | Reopens on last TXT offset/page. |
| P16-004 | TXT bookmark | Toggle bookmark, reopen | Bookmark state persists. |
| P16-005 | EPUB open | Select `.EPUB`/`.EPU` | Reader page shows extracted book text, not ZIP bytes. |
| P16-006 | EPUB progress | Page forward, back, reopen | Reopens on last EPUB reader position. |
| P16-007 | EPUB bookmark | Toggle bookmark, reopen | Bookmark state persists. |
| P16-008 | Footer mapping | Press each button in reader | Label/action mapping is correct. |
| P16-009 | Reader menu | Open/close menu and execute actions | Menu actions work without resetting reader state. |
| P16-010 | Theme preset | Change preset/theme/font and reopen | State persists according to X4/Pulp behavior. |
| P16-011 | Continue TXT | Return home/library then Continue | Reopens last TXT session. |
| P16-012 | Continue EPUB | Return home/library then Continue | Reopens last EPUB session. |
| P16-013 | Power cycle | Power cycle after state change | Last persisted progress/bookmark/theme reloads. |
