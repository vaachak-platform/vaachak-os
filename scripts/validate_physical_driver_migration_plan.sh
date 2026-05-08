#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "physical_driver_migration_plan validation failed: $1" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing file $1"
}

require_text() {
  local file="$1"
  local text="$2"
  grep -Fq "$text" "$file" || fail "missing text '$text' in $file"
}

require_regex() {
  local file="$1"
  local pattern="$2"
  python3 - "$file" "$pattern" <<'PYREGEX' || fail "missing pattern '$pattern' in $file"
import pathlib
import re
import sys
text = pathlib.Path(sys.argv[1]).read_text()
pattern = sys.argv[2]
raise SystemExit(0 if re.search(pattern, text, re.S) else 1)
PYREGEX
}

PHYS="target-xteink-x4/src/vaachak_x4/physical/physical_driver_migration_plan.rs"
SMOKE="target-xteink-x4/src/vaachak_x4/contracts/physical_driver_migration_plan_smoke.rs"
DOC="docs/architecture/physical-driver-migration-plan.md"

require_file "$PHYS"
require_file "$SMOKE"
require_file "$DOC"
require_file "target-xteink-x4/src/vaachak_x4/physical/mod.rs"
require_file "target-xteink-x4/src/vaachak_x4/contracts/mod.rs"

require_text "target-xteink-x4/src/vaachak_x4/physical/mod.rs" "pub mod physical_driver_migration_plan;"
require_text "target-xteink-x4/src/vaachak_x4/contracts/mod.rs" "pub mod physical_driver_migration_plan_smoke;"

require_text "$PHYS" "VaachakPhysicalDriverMigrationPlan"
require_text "$PHYS" "physical_driver_migration_plan=ok"
require_text "$PHYS" "PulpCompatibility"
require_text "$PHYS" "VaachakHardwareNativeBehaviorConsolidation::native_behavior_consolidation_ok()"
require_text "$PHYS" "VaachakHardwareNativeBehaviorConsolidationCleanup::cleanup_ok()"
require_text "$PHYS" "input_physical_sampling_native_driver"
require_text "$PHYS" "spi_physical_transaction_native_driver"
require_text "$PHYS" "display_ssd1677_physical_refresh_native_driver"
require_text "$PHYS" "storage_sd_mmc_block_native_driver"
require_text "$PHYS" "storage_fat_algorithm_native_driver"

require_regex "$PHYS" "MIGRATION_STEP_COUNT:\s*usize\s*=\s*5"
require_regex "$PHYS" "INPUT_PHYSICAL_SAMPLING_SELECTED_FIRST:\s*bool\s*=\s*true"
require_regex "$PHYS" "SPI_PHYSICAL_TRANSACTION_BEFORE_STORAGE_OR_DISPLAY_TAKEOVER:\s*bool\s*=\s*true"
require_regex "$PHYS" "DISPLAY_LOW_LEVEL_REFRESH_AFTER_SPI_DRIVER_GATE:\s*bool\s*=\s*true"
require_regex "$PHYS" "SD_MMC_BLOCK_DRIVER_AFTER_SPI_DRIVER_GATE:\s*bool\s*=\s*true"
require_regex "$PHYS" "FAT_ALGORITHM_DRIVER_LAST:\s*bool\s*=\s*true"
require_regex "$PHYS" "ROLLBACK_GATES_DECLARED:\s*bool\s*=\s*true"
require_regex "$PHYS" "HARDWARE_SMOKE_REQUIRED_FOR_EVERY_STEP:\s*bool\s*=\s*true"
require_regex "$PHYS" "DESTRUCTIVE_STORAGE_OPERATIONS_DEFERRED:\s*bool\s*=\s*true"
require_regex "$PHYS" "READER_FILE_BROWSER_UX_CHANGED:\s*bool\s*=\s*false"
require_regex "$PHYS" "APP_NAVIGATION_BEHAVIOR_CHANGED:\s*bool\s*=\s*false"

require_text "$SMOKE" "physical_driver_migration_plan_smoke_ok"
require_text "$SMOKE" "VaachakPhysicalDriverMigrationTarget::InputPhysicalSampling"
require_text "$SMOKE" "VaachakPhysicalDriverMigrationTarget::StorageFatAlgorithm"

require_text "$DOC" "Physical Driver Migration Plan"
require_text "$DOC" "input_physical_sampling_native_driver"
require_text "$DOC" "spi_physical_transaction_native_driver"
require_text "$DOC" "display_ssd1677_physical_refresh_native_driver"
require_text "$DOC" "storage_sd_mmc_block_native_driver"
require_text "$DOC" "storage_fat_algorithm_native_driver"
require_text "$DOC" "Rollback rules"
require_text "$DOC" "Storage safety rule"

# The plan deliverable must not include app, vendor, or bin files.
if find physical_driver_migration_plan -type f 2>/dev/null | grep -Eq '(^|/)vendor/|target-xteink-x4/src/apps/|target-xteink-x4/src/bin/'; then
  fail "overlay contains vendor/app/bin files"
fi

# This is a plan checkpoint, not a physical driver implementation.
if grep -nE 'fn[[:space:]]+(read_block|write_block|spi_transfer|toggle_cs|busy_wait|draw_buffer|poll_adc|sample_adc|mount_volume|open_fat)' "$PHYS"; then
  fail "plan appears to add low-level physical driver implementation functions"
fi

echo "physical_driver_migration_plan=ok"
