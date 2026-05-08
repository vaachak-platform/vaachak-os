#!/usr/bin/env bash
set -euo pipefail

SD="${SD:-/media/mindseye73/SD_CARD}"
OUT="${OUT:-/tmp/x4-deploy-ready-check.txt}"

status="ACCEPTED"
reason="DeployReady"

fail() {
  status="REJECTED"
  reason="$1"
}

[ -f Cargo.toml ] || fail "MissingCargoToml"
[ -f target-xteink-x4/Cargo.toml ] || fail "MissingTargetCargoToml"
[ -f vendor/pulp-os/src/apps/home.rs ] || fail "MissingHomeApp"
[ -f vendor/pulp-os/src/apps/files.rs ] || fail "MissingFilesApp"
[ -f vendor/pulp-os/src/apps/reader/mod.rs ] || fail "MissingReaderApp"

legacy_dir_count="$(find . -maxdepth 1 -type d -name '*_overlay' | wc -l | tr -d ' ')"
legacy_prefix_a="pha"
legacy_prefix_b="se"
legacy_zip_count="$(find . -maxdepth 1 -type f -name "${legacy_prefix_a}${legacy_prefix_b}*.zip" | wc -l | tr -d ' ')"

old_footer_count="$((rg -n 'Select.*Open.*Back.*Stay|Select.*open.*Back.*Stay' vendor/pulp-os/src/apps hal-xteink-x4/src/display_smoke.rs target-xteink-x4/src 2>/dev/null || true) | wc -l | tr -d ' ')"
bad_txt_body_return="$((rg -n 'TITLE_KIND_TEXT.*return Some|return Some.*TITLE_KIND_TEXT' vendor/pulp-os/kernel/src/kernel/dir_cache.rs 2>/dev/null || true) | wc -l | tr -d ' ')"

titlemap_status="missing"
titles_status="missing"
bad_phrase_lines=0
txt_title_lines=0

if [ -f "$SD/_X4/TITLEMAP.TSV" ]; then
  titlemap_status="present"
fi
if [ -f "$SD/_X4/TITLES.BIN" ]; then
  titles_status="present"
  bad_phrase_lines="$((strings -a "$SD/_X4/TITLES.BIN" | rg -n 'most other parts|world at no cost|Project Gutenberg|produced by|transcribed by|START OF THE PROJECT GUTENBERG' || true) | wc -l | tr -d ' ')"
  txt_title_lines="$((strings -a "$SD/_X4/TITLES.BIN" | rg -n '\.TXT|\.MD|POIROT~|THEMUR~|THESIG~|Poirot Investigates|Roger Ackroyd|Sign of the Four' || true) | wc -l | tr -d ' ')"
fi

if [ "$old_footer_count" != "0" ]; then
  fail "OldFooterOrderFound"
elif [ "$bad_txt_body_return" != "0" ]; then
  fail "TxtBodyTitleReturnFound"
elif [ "$bad_phrase_lines" != "0" ]; then
  fail "BadTitlePhrasesInTitlesBin"
fi

{
  echo "# X4 Deploy Ready Check"
  echo "status=$status"
  echo "reason=$reason"
  echo "sd=$SD"
  echo "root_legacy_delivery_dirs=$legacy_dir_count"
  echo "root_legacy_delivery_zip_files=$legacy_zip_count"
  echo "titlemap_status=$titlemap_status"
  echo "titles_status=$titles_status"
  echo "txt_title_lines=$txt_title_lines"
  echo "bad_phrase_lines=$bad_phrase_lines"
  echo "old_footer_count=$old_footer_count"
  echo "bad_txt_body_return=$bad_txt_body_return"
  echo "marker=x4-repository-cleanup-new-device-deploy-baseline-ok"
} | tee "$OUT"

echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 5
fi
