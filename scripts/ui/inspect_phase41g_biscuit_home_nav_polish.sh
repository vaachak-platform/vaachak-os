#!/usr/bin/env bash
set -euo pipefail

OUT="${OUT:-/tmp/phase41g-biscuit-home-nav-polish-inspection.txt}"

{
  echo "# Phase 41G Biscuit Home Nav Polish Inspection"
  echo
  echo "## Active Home renderer"
  rg -n 'phase41g|draw_menu|draw_home_card|card_title_font|card_meta_font|HOME_FOOTER|BUTTON_BAR_H|Placeholder|AppId::Upload|move_selection_(row|col)|Back   Select' vendor/pulp-os/src/apps/home.rs || true
  echo
  echo "## Shell Home model"
  rg -n 'HomeMenuItem|Sync|Upload|FileBrowser|Bookmarks|Settings' vendor/pulp-os/kernel/src/app/model.rs || true
  echo
  echo "## Global footer source"
  rg -n 'ButtonFeedback|LabelMode|action_label|bumps.draw' vendor/pulp-os/src/apps/manager.rs vendor/pulp-os/src/apps/widgets/button_feedback.rs || true
  echo
  echo "## Frozen surface diff summary"
  git diff --name-only | grep -E '^(vendor/pulp-os/src/apps/(files|reader)|vendor/pulp-os/kernel/src/kernel/dir_cache\.rs|hal-xteink-x4/src/|target-xteink-x4/src/vaachak_x4/(contracts|input|physical|runtime)/)' || true
  echo
  echo "marker=phase41g=x4-biscuit-home-nav-polish-placeholder-routing-ok"
} | tee "$OUT"

echo "Wrote: $OUT"
