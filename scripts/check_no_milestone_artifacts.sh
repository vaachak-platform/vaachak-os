#!/usr/bin/env bash
set -euo pipefail

root="${1:-.}"
cd "$root"

low_a="pha"
low_b="se"
up_a="PHA"
up_b="SE"
needle_low="${low_a}${low_b}"
needle_up="${up_a}${up_b}"

rg -n \
  "${needle_low}[0-9]|${needle_up}[0-9]|${needle_up}_[0-9]|_IN_${needle_up}|MOVED_IN_${needle_up}|OWNED_IN_${needle_up}|README-APPLY-${needle_up}|${needle_low}.*overlay" \
  --glob '!target/**' \
  --glob '!vendor/pulp-os/kernel/src/drivers/ssd1677.rs' \
  --glob '!vendor/pulp-os/kernel/src/kernel/scheduler.rs' \
  --glob '!vendor/pulp-os/kernel/src/kernel/mod.rs' \
  --glob '!scripts/check_no_milestone_artifacts.sh'
