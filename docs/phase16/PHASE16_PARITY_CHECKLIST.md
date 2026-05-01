# Phase 16 Reader Parity Checklist Against x4-reader-os-rs

Use this checklist on the X4 device after the Phase 16 patch compiles.

## 1. Source parity

| Check | Expected |
|---|---|
| Target main source | `target-xteink-x4/src/main.rs` is copied from `vendor/pulp-os/src/bin/main.rs`. |
| Allowed source delta | Only `x4_os::` → `pulp_os::` crate alias and `phase16=x4-reader-parity-ok` marker. |
| EPUB path | Uses vendored X4/Pulp reader + `smol-epub`. |
| Fake EPUB path | No `run_epub_reader_page_storage_smoke`, no raw ZIP byte rendering. |

## 2. TXT/MD progress

| Step | Expected |
|---|---|
| Open TXT/MD | First page renders normally. |
| Page forward | Offset/page changes. |
| Back/exit | Returns to file/library shell without losing state. |
| Reopen same file | Resumes at last TXT/MD position. |
| Power-cycle/reopen | Last TXT/MD position persists if the X4/Pulp state layer flushes to SD. |

## 3. EPUB/EPU progress

| Step | Expected |
|---|---|
| Open EPUB/EPU | Real chapter text renders. No `PK`, no ZIP central directory bytes. |
| Page forward | Page/chapter position changes. |
| Back/exit | Returns to file/library shell. |
| Reopen same EPUB | Resumes at last EPUB reader position. |
| Power-cycle/reopen | Last EPUB position persists if the X4/Pulp state layer flushes to SD. |

## 4. TXT/MD bookmarks

| Step | Expected |
|---|---|
| Open TXT/MD page | Page renders. |
| Toggle bookmark | UI reflects bookmark state. |
| Exit/reopen | Bookmark state is still present. |
| Toggle again | Bookmark is removed or toggled according to X4/Pulp behavior. |

## 5. EPUB/EPU bookmarks

| Step | Expected |
|---|---|
| Open EPUB page | Page renders extracted text. |
| Toggle bookmark | UI reflects bookmark state. |
| Exit/reopen | Bookmark state is still present for EPUB. |
| Navigate chapters/pages | Bookmark state remains stable. |

## 6. Reader footer action labels

Record the visible labels for each reader mode.

| Context | Expected check |
|---|---|
| Library/file screen | Labels are not shifted/misordered relative to button behavior. |
| TXT reader | Footer labels correspond to Back/prev/next/menu/bookmark behavior. |
| EPUB reader | Footer labels correspond to Back/prev/next/menu/bookmark behavior. |
| Menu open | Footer labels change consistently with menu actions. |

Known prior issue to guard against: footer labels must not be visually shifted against button behavior.

## 7. Reader menu actions

| Action family | Expected |
|---|---|
| Back/return | Returns to the previous shell without resetting book state. |
| Bookmark | Toggles current page/bookmark state. |
| Navigation | Next/previous page or chapter works without display corruption. |
| Settings/theme | Opens or cycles reader display setting if exposed by X4/Pulp. |
| Close menu | Returns to reader page with no full state reset. |

## 8. Theme preset/state file support

| Step | Expected |
|---|---|
| Change reader theme/font/preset | UI updates. |
| Exit/reopen same book | Selected preset persists. |
| Open different book | Preset behavior follows X4/Pulp global/per-book state rules. |
| Power-cycle | Persisted state reloads if the state path has been flushed. |

## 9. Continue behavior

| Step | Expected |
|---|---|
| Open TXT/MD and move position | Last-session state updates. |
| Return home/library | Continue is available if X4/Pulp exposes it. |
| Select Continue | Reopens same TXT/MD at last position. |
| Open EPUB and move position | Last-session state updates. |
| Select Continue | Reopens same EPUB at last position. |

## 10. Serial markers

Expected marker:

```text
phase16=x4-reader-parity-ok
```

Failure markers to reject:

```text
First readable bytes
ZIP container parsed
run_epub_reader_page_storage_smoke
```
