#!/usr/bin/env bash
set -euo pipefail

HOME_DASHBOARD_STILL_ACTIVE="${HOME_DASHBOARD_STILL_ACTIVE:-0}"
CARD_FONT_SIZE_ACCEPTABLE="${CARD_FONT_SIZE_ACCEPTABLE:-0}"
READER_CARD_TEXT_NOT_CLIPPED="${READER_CARD_TEXT_NOT_CLIPPED:-0}"
SINGLE_FOOTER_CONFIRMED="${SINGLE_FOOTER_CONFIRMED:-0}"
OLD_DUPLICATE_FOOTER_GONE="${OLD_DUPLICATE_FOOTER_GONE:-0}"
LEFT_RIGHT_NAV_CONFIRMED="${LEFT_RIGHT_NAV_CONFIRMED:-0}"
UP_DOWN_NAV_CONFIRMED="${UP_DOWN_NAV_CONFIRMED:-0}"
READER_CARD_OPENS_READER="${READER_CARD_OPENS_READER:-0}"
LIBRARY_CARD_OPENS_FILES="${LIBRARY_CARD_OPENS_FILES:-0}"
PLACEHOLDER_APPS_SAFE="${PLACEHOLDER_APPS_SAFE:-0}"
FILES_TITLES_STILL_OK="${FILES_TITLES_STILL_OK:-0}"
READER_RESTORE_STILL_OK="${READER_RESTORE_STILL_OK:-0}"
NO_INPUT_WRITE_GEOMETRY_REGRESSION="${NO_INPUT_WRITE_GEOMETRY_REGRESSION:-0}"
NO_CRASH_REBOOT="${NO_CRASH_REBOOT:-0}"
OUT="${OUT:-/tmp/phase41h-device-biscuit-ui-report.txt}"

status="ACCEPTED"
reason="BiscuitUiDeviceFreezeConfirmed"

if [ "$HOME_DASHBOARD_STILL_ACTIVE" != "1" ]; then
  status="REJECTED"; reason="HomeDashboardNotConfirmed"
elif [ "$CARD_FONT_SIZE_ACCEPTABLE" != "1" ]; then
  status="REJECTED"; reason="CardFontSizeNotAcceptable"
elif [ "$READER_CARD_TEXT_NOT_CLIPPED" != "1" ]; then
  status="REJECTED"; reason="ReaderCardTextClipped"
elif [ "$SINGLE_FOOTER_CONFIRMED" != "1" ]; then
  status="REJECTED"; reason="SingleFooterNotConfirmed"
elif [ "$OLD_DUPLICATE_FOOTER_GONE" != "1" ]; then
  status="REJECTED"; reason="DuplicateFooterStillPresent"
elif [ "$LEFT_RIGHT_NAV_CONFIRMED" != "1" ]; then
  status="REJECTED"; reason="LeftRightNavNotConfirmed"
elif [ "$UP_DOWN_NAV_CONFIRMED" != "1" ]; then
  status="REJECTED"; reason="UpDownNavNotConfirmed"
elif [ "$READER_CARD_OPENS_READER" != "1" ]; then
  status="REJECTED"; reason="ReaderCardRouteRegression"
elif [ "$LIBRARY_CARD_OPENS_FILES" != "1" ]; then
  status="REJECTED"; reason="LibraryCardRouteRegression"
elif [ "$PLACEHOLDER_APPS_SAFE" != "1" ]; then
  status="REJECTED"; reason="PlaceholderAppsUnsafe"
elif [ "$FILES_TITLES_STILL_OK" != "1" ]; then
  status="REJECTED"; reason="FilesTitlesRegression"
elif [ "$READER_RESTORE_STILL_OK" != "1" ]; then
  status="REJECTED"; reason="ReaderRestoreRegression"
elif [ "$NO_INPUT_WRITE_GEOMETRY_REGRESSION" != "1" ]; then
  status="REJECTED"; reason="InputWriteGeometryRegression"
elif [ "$NO_CRASH_REBOOT" != "1" ]; then
  status="REJECTED"; reason="CrashOrRebootObserved"
fi

{
  echo "# Phase 41H Device Biscuit UI Report"
  echo "status=$status"
  echo "reason=$reason"
  echo "home_dashboard_still_active=$HOME_DASHBOARD_STILL_ACTIVE"
  echo "card_font_size_acceptable=$CARD_FONT_SIZE_ACCEPTABLE"
  echo "reader_card_text_not_clipped=$READER_CARD_TEXT_NOT_CLIPPED"
  echo "single_footer_confirmed=$SINGLE_FOOTER_CONFIRMED"
  echo "old_duplicate_footer_gone=$OLD_DUPLICATE_FOOTER_GONE"
  echo "left_right_nav_confirmed=$LEFT_RIGHT_NAV_CONFIRMED"
  echo "up_down_nav_confirmed=$UP_DOWN_NAV_CONFIRMED"
  echo "reader_card_opens_reader=$READER_CARD_OPENS_READER"
  echo "library_card_opens_files=$LIBRARY_CARD_OPENS_FILES"
  echo "placeholder_apps_safe=$PLACEHOLDER_APPS_SAFE"
  echo "files_titles_still_ok=$FILES_TITLES_STILL_OK"
  echo "reader_restore_still_ok=$READER_RESTORE_STILL_OK"
  echo "no_input_write_geometry_regression=$NO_INPUT_WRITE_GEOMETRY_REGRESSION"
  echo "no_crash_reboot=$NO_CRASH_REBOOT"
  echo "marker=phase41h=x4-biscuit-ui-acceptance-freeze-ok"
} | tee "$OUT"

echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 4
fi
