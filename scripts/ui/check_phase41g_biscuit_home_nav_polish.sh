#!/usr/bin/env bash
set -euo pipefail

HOME="vendor/pulp-os/src/apps/home.rs"
MODEL="vendor/pulp-os/kernel/src/app/model.rs"

test -f "$HOME"
test -f "$MODEL"

grep -q 'phase41g=x4-biscuit-home-nav-polish-placeholder-routing-ok' "$HOME"
grep -q 'PHASE41G_HOME_NAV_POLISH_MARKER' "$HOME"
grep -q 'fn draw_home_card' "$HOME"
grep -q 'fn card_title_font' "$HOME"
grep -q 'fonts::body_font(1)' "$HOME"
grep -q 'fn card_meta_font' "$HOME"
grep -q 'fonts::chrome_font()' "$HOME"
grep -q 'HOME_FOOTER_RESERVED_H: u16 = BUTTON_BAR_H' "$HOME"
grep -q 'MenuAction::Placeholder("Sync")' "$HOME"
grep -q 'MenuAction::Push(AppId::Upload)' "$HOME"
grep -q 'HomeMenuItem::Sync' "$MODEL"

if grep -q 'Back   Select   Left   Right' "$HOME"; then
  echo "phase41g-nav-polish-check=failed custom-home-footer-still-present" >&2
  exit 3
fi

echo "phase41g-nav-polish-check=ok"
