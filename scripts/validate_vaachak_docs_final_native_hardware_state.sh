#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "vaachak_docs_final_native_hardware_state validation failed: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing file: $1"
}

require_text() {
  local file="$1"
  local text="$2"
  require_file "$file"
  grep -Fq "$text" "$file" || fail "missing text '$text' in $file"
}

forbid_text() {
  local file="$1"
  local text="$2"
  require_file "$file"
  if grep -Fq "$text" "$file"; then
    fail "stale text '$text' still present in $file"
  fi
}

canonical_docs=(
  README.md
  SCOPE.md
  ROADMAP.md
  docs/architecture/current-runtime.md
  docs/architecture/ownership-map.md
  docs/architecture/current-architecture-and-roadmap.md
  docs/architecture/reference-material-review.md
  docs/architecture/documentation-index.md
  docs/architecture/pulp-os-post-hardware-migration-scope.md
  docs/architecture/vaachak-os-hardware-runtime-architecture.md
  docs/development/build-and-flash.md
  docs/development/consolidated-validation.md
  docs/roadmap/x4-reader-roadmap.md
  docs/formats/vchk-package-contract.md
)

for doc in "${canonical_docs[@]}"; do
  require_file "$doc"
done

require_text README.md "vaachak_hardware_runtime_final_acceptance=ok"
require_text README.md "VaachakNativeSpiPhysicalDriver"
require_text README.md "VaachakNativeSsd1677PhysicalDriver"
require_text README.md "VaachakNativeSdMmcPhysicalDriver"
require_text README.md "VaachakNativeFatAlgorithmDriver"
require_text README.md "Pulp OS is not the active hardware runtime"

require_text docs/architecture/current-runtime.md "Pulp OS is not the active hardware runtime"
require_text docs/architecture/current-runtime.md "VaachakNativeSpiPhysicalDriver"
require_text docs/architecture/current-runtime.md "VaachakNativeSsd1677PhysicalDriver"
require_text docs/architecture/current-runtime.md "VaachakNativeSdMmcPhysicalDriver"
require_text docs/architecture/current-runtime.md "VaachakNativeFatAlgorithmDriver"

require_text docs/architecture/ownership-map.md "VaachakNativeSpiPhysicalDriver"
require_text docs/architecture/ownership-map.md "VaachakNativeSsd1677PhysicalDriver"
require_text docs/architecture/ownership-map.md "VaachakNativeSdMmcPhysicalDriver"
require_text docs/architecture/ownership-map.md "VaachakNativeFatAlgorithmDriver"
require_text docs/architecture/pulp-os-post-hardware-migration-scope.md "non-hardware compatibility"
require_text docs/architecture/vaachak-os-hardware-runtime-architecture.md "Vaachak owns the X4 hardware runtime"

require_text ROADMAP.md "Reader Home + Resume"
require_text ROADMAP.md "XTC compatibility"
require_text ROADMAP.md '.vchk'
require_text docs/roadmap/x4-reader-roadmap.md "Reader Home + Resume Foundation"
require_text docs/formats/vchk-package-contract.md "Status: Draft planning contract"

require_file scripts/validate_vaachak_hardware_runtime_final_acceptance.sh
require_file scripts/validate_hardware_physical_full_migration_consolidation.sh
require_file scripts/validate_vendor_pulp_os_scope_reduction.sh

require_file target-xteink-x4/src/vaachak_x4/physical/spi_physical_native_driver.rs
require_file target-xteink-x4/src/vaachak_x4/physical/display_physical_ssd1677_native_driver.rs
require_file target-xteink-x4/src/vaachak_x4/physical/storage_physical_sd_mmc_native_driver.rs
require_file target-xteink-x4/src/vaachak_x4/physical/storage_fat_algorithm_native_driver.rs
require_file target-xteink-x4/src/vaachak_x4/physical/vaachak_hardware_runtime_final_acceptance.rs

require_text target-xteink-x4/src/vaachak_x4/physical/spi_physical_native_driver.rs "VaachakNativeSpiPhysicalDriver"
require_text target-xteink-x4/src/vaachak_x4/physical/display_physical_ssd1677_native_driver.rs "VaachakNativeSsd1677PhysicalDriver"
require_text target-xteink-x4/src/vaachak_x4/physical/storage_physical_sd_mmc_native_driver.rs "VaachakNativeSdMmcPhysicalDriver"
require_text target-xteink-x4/src/vaachak_x4/physical/storage_fat_algorithm_native_driver.rs "VaachakNativeFatAlgorithmDriver"
require_text target-xteink-x4/src/vaachak_x4/physical/vaachak_hardware_runtime_final_acceptance.rs "vaachak_hardware_runtime_final_acceptance=ok"

for doc in "${canonical_docs[@]}"; do
  forbid_text "$doc" "vendor/pulp-os remains the active firmware runtime"
  forbid_text "$doc" "Pulp owns active"
  forbid_text "$doc" "Pulp hardware is active"
  forbid_text "$doc" "active hardware runtime is still Pulp"
  forbid_text "$doc" "hardware migration is pending"
  forbid_text "$doc" "Pulp-derived runtime is still the active"
  forbid_text "$doc" "Pulp owns display/input/storage/SPI"
done

echo "vaachak_docs_final_native_hardware_state=ok"
