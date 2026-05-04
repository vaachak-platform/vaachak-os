#!/usr/bin/env bash
set -euo pipefail

HOME="vendor/pulp-os/src/apps/home.rs"
FILES="vendor/pulp-os/src/apps/files.rs"
READER="vendor/pulp-os/src/apps/reader/mod.rs"
DIR_CACHE="vendor/pulp-os/kernel/src/kernel/dir_cache.rs"

test -f "$HOME"
test -f "$FILES"
test -f "$READER"
test -f "$DIR_CACHE"

grep -q 'phase41g=x4-biscuit-home-nav-polish-placeholder-routing-ok' "$HOME"
grep -q 'Reader' "$HOME"
grep -q 'Library' "$HOME"
grep -q 'Bookmarks' "$HOME"
grep -q 'Settings' "$HOME"
grep -q 'Sync' "$HOME"
grep -q 'Upload' "$HOME"
grep -q 'Coming soon' "$HOME"

if rg -n 'Back[[:space:]]+Select[[:space:]]+Left[[:space:]]+Right|Back Select Left Right' "$HOME" >/dev/null 2>&1; then
  echo "custom Home footer row still appears in home.rs; duplicate footer risk" >&2
  exit 3
fi

grep -q 'phase40g-repair3=x4-disable-txt-body-title-scanning-ok' "$DIR_CACHE"
grep -q 'phase40h-repair1=x4-seed-txt-titlemap-into-titles-bin-ok' "$DIR_CACHE"

old_footer_count="$((rg -n 'Select.*Open.*Back.*Stay|Select.*open.*Back.*Stay' vendor/pulp-os/src/apps hal-xteink-x4/src/display_smoke.rs target-xteink-x4/src 2>/dev/null || true) | wc -l | tr -d ' ')"
if [ "$old_footer_count" != "0" ]; then
  echo "old footer ordering found: $old_footer_count" >&2
  exit 4
fi

echo "phase41h-check=ok"
echo "phase41h=x4-biscuit-ui-acceptance-freeze-ok"
