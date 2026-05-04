#!/usr/bin/env bash
set -euo pipefail

HOME_DASHBOARD_STILL_ACTIVE="${HOME_DASHBOARD_STILL_ACTIVE:-0}"
SETTINGS_CARD_OPENS_SETTINGS="${SETTINGS_CARD_OPENS_SETTINGS:-0}"
SETTINGS_SECTIONS_VISIBLE="${SETTINGS_SECTIONS_VISIBLE:-0}"
READER_SETTINGS_VISIBLE="${READER_SETTINGS_VISIBLE:-0}"
DISPLAY_SETTINGS_VISIBLE="${DISPLAY_SETTINGS_VISIBLE:-0}"
STORAGE_SETTINGS_VISIBLE="${STORAGE_SETTINGS_VISIBLE:-0}"
DEVICE_SETTINGS_VISIBLE="${DEVICE_SETTINGS_VISIBLE:-0}"
ABOUT_VISIBLE="${ABOUT_VISIBLE:-0}"
SETTINGS_NAVIGATION_WORKS="${SETTINGS_NAVIGATION_WORKS:-0}"
SETTINGS_BACK_RETURNS_HOME="${SETTINGS_BACK_RETURNS_HOME:-0}"
READER_CARD_STILL_WORKS="${READER_CARD_STILL_WORKS:-0}"
LIBRARY_CARD_STILL_WORKS="${LIBRARY_CARD_STILL_WORKS:-0}"
SYNC_UPLOAD_PLACEHOLDERS_SAFE="${SYNC_UPLOAD_PLACEHOLDERS_SAFE:-0}"
FILES_TITLES_STILL_OK="${FILES_TITLES_STILL_OK:-0}"
READER_RESTORE_STILL_OK="${READER_RESTORE_STILL_OK:-0}"
SINGLE_FOOTER_CONFIRMED="${SINGLE_FOOTER_CONFIRMED:-0}"
NO_INPUT_WRITE_GEOMETRY_REGRESSION="${NO_INPUT_WRITE_GEOMETRY_REGRESSION:-0}"
NO_CRASH_REBOOT="${NO_CRASH_REBOOT:-0}"
OUT="${OUT:-/tmp/phase42a-device-app-shell-settings-report.txt}"

status="ACCEPTED"
reason="AppShellRoutingSettingsImplemented"

for pair in \
  "HOME_DASHBOARD_STILL_ACTIVE:$HOME_DASHBOARD_STILL_ACTIVE:HomeDashboardMissing" \
  "SETTINGS_CARD_OPENS_SETTINGS:$SETTINGS_CARD_OPENS_SETTINGS:SettingsCardRoutingRejected" \
  "SETTINGS_SECTIONS_VISIBLE:$SETTINGS_SECTIONS_VISIBLE:SettingsSectionsMissing" \
  "READER_SETTINGS_VISIBLE:$READER_SETTINGS_VISIBLE:ReaderSettingsMissing" \
  "DISPLAY_SETTINGS_VISIBLE:$DISPLAY_SETTINGS_VISIBLE:DisplaySettingsMissing" \
  "STORAGE_SETTINGS_VISIBLE:$STORAGE_SETTINGS_VISIBLE:StorageSettingsMissing" \
  "DEVICE_SETTINGS_VISIBLE:$DEVICE_SETTINGS_VISIBLE:DeviceSettingsMissing" \
  "ABOUT_VISIBLE:$ABOUT_VISIBLE:AboutMissing" \
  "SETTINGS_NAVIGATION_WORKS:$SETTINGS_NAVIGATION_WORKS:SettingsNavigationRejected" \
  "SETTINGS_BACK_RETURNS_HOME:$SETTINGS_BACK_RETURNS_HOME:SettingsBackRejected" \
  "READER_CARD_STILL_WORKS:$READER_CARD_STILL_WORKS:ReaderCardRegression" \
  "LIBRARY_CARD_STILL_WORKS:$LIBRARY_CARD_STILL_WORKS:LibraryCardRegression" \
  "SYNC_UPLOAD_PLACEHOLDERS_SAFE:$SYNC_UPLOAD_PLACEHOLDERS_SAFE:PlaceholderRoutingUnsafe" \
  "FILES_TITLES_STILL_OK:$FILES_TITLES_STILL_OK:FilesTitleRegression" \
  "READER_RESTORE_STILL_OK:$READER_RESTORE_STILL_OK:ReaderRestoreRegression" \
  "SINGLE_FOOTER_CONFIRMED:$SINGLE_FOOTER_CONFIRMED:FooterRegression" \
  "NO_INPUT_WRITE_GEOMETRY_REGRESSION:$NO_INPUT_WRITE_GEOMETRY_REGRESSION:InputWriteGeometryRegression" \
  "NO_CRASH_REBOOT:$NO_CRASH_REBOOT:CrashOrRebootObserved"
do
  value="$(echo "$pair" | cut -d: -f2)"
  fail_reason="$(echo "$pair" | cut -d: -f3)"
  if [ "$value" != "1" ]; then
    status="REJECTED"
    reason="$fail_reason"
    break
  fi
done

{
  echo "# Phase 42A Device App Shell Settings Report"
  echo "status=$status"
  echo "reason=$reason"
  echo "home_dashboard_still_active=$HOME_DASHBOARD_STILL_ACTIVE"
  echo "settings_card_opens_settings=$SETTINGS_CARD_OPENS_SETTINGS"
  echo "settings_sections_visible=$SETTINGS_SECTIONS_VISIBLE"
  echo "reader_settings_visible=$READER_SETTINGS_VISIBLE"
  echo "display_settings_visible=$DISPLAY_SETTINGS_VISIBLE"
  echo "storage_settings_visible=$STORAGE_SETTINGS_VISIBLE"
  echo "device_settings_visible=$DEVICE_SETTINGS_VISIBLE"
  echo "about_visible=$ABOUT_VISIBLE"
  echo "settings_navigation_works=$SETTINGS_NAVIGATION_WORKS"
  echo "settings_back_returns_home=$SETTINGS_BACK_RETURNS_HOME"
  echo "reader_card_still_works=$READER_CARD_STILL_WORKS"
  echo "library_card_still_works=$LIBRARY_CARD_STILL_WORKS"
  echo "sync_upload_placeholders_safe=$SYNC_UPLOAD_PLACEHOLDERS_SAFE"
  echo "files_titles_still_ok=$FILES_TITLES_STILL_OK"
  echo "reader_restore_still_ok=$READER_RESTORE_STILL_OK"
  echo "single_footer_confirmed=$SINGLE_FOOTER_CONFIRMED"
  echo "no_input_write_geometry_regression=$NO_INPUT_WRITE_GEOMETRY_REGRESSION"
  echo "no_crash_reboot=$NO_CRASH_REBOOT"
  echo "marker=phase42a=x4-app-shell-routing-settings-implementation-ok"
} | tee "$OUT"

echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 4
fi
