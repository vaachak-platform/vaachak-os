#!/usr/bin/env bash
set -euo pipefail

HOME_FILES_READER_CONFIRMED="${HOME_FILES_READER_CONFIRMED:-0}"
FOOTER_LABELS_CONFIRMED="${FOOTER_LABELS_CONFIRMED:-0}"
EPUB_TITLES_CONFIRMED="${EPUB_TITLES_CONFIRMED:-0}"
READER_RESTORE_CONFIRMED="${READER_RESTORE_CONFIRMED:-0}"
NO_CRASH_REBOOT="${NO_CRASH_REBOOT:-0}"
FOOTER_LABELS_OBSERVED="${FOOTER_LABELS_OBSERVED:-capture-current-device-footer-labels}"
TITLE_DISPLAY_OBSERVED="${TITLE_DISPLAY_OBSERVED:-capture-current-device-title-display}"
OUT="${OUT:-/tmp/phase40b-manual-device-ux-report.txt}"

status="ACCEPTED"
reason="ReaderUxBaselineConfirmed"

if [ "$HOME_FILES_READER_CONFIRMED" != "1" ]; then
  status="REJECTED"
  reason="HomeFilesReaderFlowNotConfirmed"
elif [ "$FOOTER_LABELS_CONFIRMED" != "1" ]; then
  status="REJECTED"
  reason="FooterLabelsNotConfirmed"
elif [ "$EPUB_TITLES_CONFIRMED" != "1" ]; then
  status="REJECTED"
  reason="EpubTitlesNotConfirmed"
elif [ "$READER_RESTORE_CONFIRMED" != "1" ]; then
  status="REJECTED"
  reason="ReaderRestoreNotConfirmed"
elif [ "$NO_CRASH_REBOOT" != "1" ]; then
  status="REJECTED"
  reason="CrashOrRebootObserved"
fi

{
  echo "# Phase 40B Manual Device UX Report"
  echo "status=$status"
  echo "reason=$reason"
  echo "home_files_reader_confirmed=$HOME_FILES_READER_CONFIRMED"
  echo "footer_labels_confirmed=$FOOTER_LABELS_CONFIRMED"
  echo "epub_titles_confirmed=$EPUB_TITLES_CONFIRMED"
  echo "reader_restore_confirmed=$READER_RESTORE_CONFIRMED"
  echo "no_crash_reboot=$NO_CRASH_REBOOT"
  echo "footer_labels_observed=$FOOTER_LABELS_OBSERVED"
  echo "title_display_observed=$TITLE_DISPLAY_OBSERVED"
  echo "marker=phase40b=x4-reader-ux-regression-baseline-ok"
  echo
  echo "## Manual baseline checklist"
  echo "- Home opens"
  echo "- Files/Library opens"
  echo "- EPUB titles display correctly"
  echo "- Footer/button labels captured"
  echo "- Open EPUB"
  echo "- Scroll a few pages"
  echo "- Back returns to Library"
  echo "- Reopen same EPUB and confirm progress restores"
  echo "- Theme persists after reopen"
  echo "- Bookmark list/index remain correct"
  echo "- No crash/reboot"
} | tee "$OUT"

echo
echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 4
fi
