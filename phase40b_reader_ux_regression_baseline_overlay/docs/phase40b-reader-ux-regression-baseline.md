# Phase 40B — Reader UX Regression Baseline

Phase 40B captures the current reader UX as a baseline before any new UX work.

It does not change behavior.

Baseline flow:

```text
Home
  -> Files/Library
  -> Reader
  -> scroll
  -> Back
  -> Files/Library
  -> reopen same EPUB
  -> restore progress/theme/bookmarks
```

Manual device baseline checklist:

```text
Home opens
Files/Library opens
EPUB titles display correctly
Footer/button labels are captured
Open EPUB
Scroll a few pages
Back returns to Library
Reopen same EPUB and confirm progress restores
Theme persists after reopen
Bookmark list/index remain correct
No crash/reboot
```

Run:

```bash
./phase40b_reader_ux_regression_baseline_overlay/scripts/guard_phase40b_write_lane_closed.sh
./phase40b_reader_ux_regression_baseline_overlay/scripts/inspect_phase40b_reader_ux_surface.sh
SD=/media/mindseye73/C0D2-109E ./phase40b_reader_ux_regression_baseline_overlay/scripts/inspect_phase40b_epub_title_baseline.sh
HOME_FILES_READER_CONFIRMED=1 FOOTER_LABELS_CONFIRMED=1 EPUB_TITLES_CONFIRMED=1 READER_RESTORE_CONFIRMED=1 NO_CRASH_REBOOT=1 ./phase40b_reader_ux_regression_baseline_overlay/scripts/write_phase40b_manual_device_ux_report.sh
HOME_FILES_READER_CONFIRMED=1 FOOTER_LABELS_CONFIRMED=1 EPUB_TITLES_CONFIRMED=1 READER_RESTORE_CONFIRMED=1 NO_CRASH_REBOOT=1 SD=/media/mindseye73/C0D2-109E ./phase40b_reader_ux_regression_baseline_overlay/scripts/accept_phase40b_reader_ux_regression_baseline.sh
```
