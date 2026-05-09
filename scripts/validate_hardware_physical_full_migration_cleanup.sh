#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "hardware_physical_full_migration_cleanup validation failed: $*" >&2
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

PHYSICAL="target-xteink-x4/src/vaachak_x4/physical/hardware_physical_full_migration_cleanup.rs"
CONTRACT="target-xteink-x4/src/vaachak_x4/contracts/hardware_physical_full_migration_cleanup_smoke.rs"
DOC="docs/architecture/hardware-physical-full-migration-cleanup.md"
CLEANUP="scripts/cleanup_hardware_physical_full_migration_artifacts.sh"

require_file "$PHYSICAL"
require_file "$CONTRACT"
require_file "$DOC"
require_file "$CLEANUP"
require_file "target-xteink-x4/src/vaachak_x4/physical/hardware_physical_full_migration_consolidation.rs"
require_file "target-xteink-x4/src/vaachak_x4/contracts/hardware_physical_full_migration_consolidation_smoke.rs"

require_text "target-xteink-x4/src/vaachak_x4/physical/mod.rs" "pub mod hardware_physical_full_migration_cleanup;"
require_text "target-xteink-x4/src/vaachak_x4/contracts/mod.rs" "pub mod hardware_physical_full_migration_cleanup_smoke;"

require_text "$PHYSICAL" "VaachakHardwarePhysicalFullMigrationCleanup"
require_text "$PHYSICAL" "VaachakHardwarePhysicalFullMigrationConsolidation"
require_text "$PHYSICAL" "hardware_physical_full_migration_cleanup=ok"
require_text "$PHYSICAL" "cleanup_ok"
require_text "$PHYSICAL" "LEGACY_OVERLAY_ARTIFACTS_REMOVED"
require_text "$PHYSICAL" "native_spi_driver_consolidated"
require_text "$PHYSICAL" "native_display_driver_consolidated"
require_text "$PHYSICAL" "native_sd_mmc_driver_consolidated"
require_text "$PHYSICAL" "native_fat_algorithm_driver_consolidated"
require_text "$PHYSICAL" "native_input_sampling_driver_consolidated"
require_text "$PHYSICAL" "READER_FILE_BROWSER_UX_CHANGED: bool = false"
require_text "$PHYSICAL" "APP_NAVIGATION_BEHAVIOR_CHANGED: bool = false"
require_text "$PHYSICAL" "ADDITIONAL_PULP_HARDWARE_FALLBACK_ENABLED: bool = false"

require_text "$CONTRACT" "VaachakHardwarePhysicalFullMigrationCleanupSmoke"
require_text "$CONTRACT" "VaachakHardwarePhysicalFullMigrationCleanup::cleanup_ok()"
require_text "$DOC" "hardware_physical_full_migration_cleanup=ok"
require_text "$DOC" "SPI physical native driver"
require_text "$DOC" "SSD1677 display physical native driver"
require_text "$DOC" "SD/MMC physical native driver"
require_text "$DOC" "FAT algorithm native driver"
require_text "$DOC" "Input physical sampling native driver"

# Verify no old generated overlay directories remain at repo root, except the
# current cleanup overlay if it is still present after applying.
for entry in ./*; do
  [ -d "$entry" ] || continue
  name="${entry#./}"
  [ "$name" != "hardware_physical_full_migration_cleanup" ] || continue
  if [ -f "$entry/MANIFEST.txt" ] && [ -f "$entry/README-APPLY.md" ]; then
    fail "old generated overlay folder still present: $name"
  fi
done

# Verify no generated overlay zip files remain at repo root. Unrelated zips that
# do not contain generated overlay manifests are ignored.
for zipfile in ./*.zip; do
  [ -f "$zipfile" ] || continue
  if unzip -Z -1 "$zipfile" >/tmp/vaachak-overlay-zip-list-validate.$$ 2>/dev/null; then
    if grep -Eq '(^|/)MANIFEST\.txt$' /tmp/vaachak-overlay-zip-list-validate.$$ \
      && grep -Eq '(^|/)README-APPLY\.md$' /tmp/vaachak-overlay-zip-list-validate.$$; then
      rm -f /tmp/vaachak-overlay-zip-list-validate.$$
      fail "old generated overlay zip still present: $zipfile"
    fi
  fi
  rm -f /tmp/vaachak-overlay-zip-list-validate.$$
done

printf 'hardware_physical_full_migration_cleanup=ok\n'
