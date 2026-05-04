#!/usr/bin/env bash
set -euo pipefail

OUT="${OUT:-/tmp/phase42a-app-shell-settings-inspection.txt}"

{
  echo "# Phase 42A App Shell Settings Inspection"
  echo
  echo "## Home routing"
  rg -n 'Reader|Library|Bookmarks|Settings|Sync|Upload|MenuAction|AppId::Settings|AppId::Upload|Placeholder' vendor/pulp-os/src/apps/home.rs || true
  echo
  echo "## Settings app"
  rg -n 'phase42a|SettingsRowKind|ReaderFont|DisplayRefresh|StorageSdStatus|DeviceSleepTimeout|About|cycle_selected|Transition::Pop' vendor/pulp-os/src/apps/settings.rs || true
  echo
  echo "## Shell wiring"
  rg -n 'AppScreen::Settings|AppId::Settings|SettingsApp|with_app' vendor/pulp-os/src/apps/manager.rs vendor/pulp-os/kernel/src/app/model.rs vendor/pulp-os/src/apps/mod.rs || true
  echo
  echo "## Frozen surface diff summary"
  git diff --name-only | grep -E '^(vendor/pulp-os/src/apps/(files|reader)|vendor/pulp-os/kernel/src/kernel/dir_cache\.rs|hal-xteink-x4/src/|target-xteink-x4/src/vaachak_x4/(input|physical|runtime)/)' || true
  echo
  echo "marker=phase42a=x4-app-shell-routing-settings-implementation-ok"
} | tee "$OUT"

echo "Wrote: $OUT"
