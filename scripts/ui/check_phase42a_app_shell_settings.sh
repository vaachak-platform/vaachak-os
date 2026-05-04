#!/usr/bin/env bash
set -euo pipefail

HOME="vendor/pulp-os/src/apps/home.rs"
SETTINGS="vendor/pulp-os/src/apps/settings.rs"
MANAGER="vendor/pulp-os/src/apps/manager.rs"
MODEL="vendor/pulp-os/kernel/src/app/model.rs"

test -f "$HOME"
test -f "$SETTINGS"
test -f "$MANAGER"
test -f "$MODEL"

grep -q 'phase42a=x4-app-shell-routing-settings-implementation-ok' "$SETTINGS"
grep -q 'PHASE42A_APP_SHELL_SETTINGS_MARKER' "$SETTINGS"
grep -q 'AppScreen::Settings' "$MANAGER"
grep -q 'Settings,' "$MODEL"
grep -q 'MenuAction::Push(AppId::Settings)' "$HOME"
grep -q 'MenuAction::Placeholder("Sync")' "$HOME"
grep -q 'MenuAction::Push(AppId::Upload)' "$HOME"

for label in \
  Reader Display Storage Device About \
  'Font size' 'Line spacing' Margins 'Show progress' \
  'Refresh mode' 'Invert colors' Contrast \
  'SD status' 'Books count' 'Title cache' 'Rebuild title cache' \
  Battery 'Sleep timeout' 'Button test' \
  VaachakOS 'Xteink X4' Build 'Storage layout'
do
  grep -q "$label" "$SETTINGS"
done

if grep -q 'Back   Select   Left   Right' "$HOME"; then
  echo "phase42a-check=failed duplicate-home-footer-risk" >&2
  exit 3
fi

echo "phase42a-check=ok"
