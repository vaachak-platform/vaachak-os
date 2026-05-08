#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="${1:-$(pwd)}"
cd "$REPO_ROOT"

fail() {
  echo "storage_probe_mount_contract validation failed: $*" >&2
  exit 1
}

require_file() {
  local path="$1"
  [[ -f "$path" ]] || fail "missing required file: $path"
}

require_rg() {
  local pattern="$1"
  shift
  [[ "$#" -gt 0 ]] || fail "require_rg called without path for pattern: $pattern"
  rg -n "$pattern" "$@" >/dev/null || fail "missing pattern '$pattern' in $*"
}

reject_rg() {
  local pattern="$1"
  shift
  [[ "$#" -gt 0 ]] || fail "reject_rg called without path for pattern: $pattern"
  if rg -n "$pattern" "$@" >/dev/null; then
    fail "forbidden pattern '$pattern' found in $*"
  fi
}

SPI_RUNTIME="target-xteink-x4/src/vaachak_x4/physical/spi_bus_runtime.rs"
STORAGE_CONTRACT="target-xteink-x4/src/vaachak_x4/physical/storage_probe_mount_contract.rs"
STORAGE_SMOKE="target-xteink-x4/src/vaachak_x4/contracts/storage_probe_mount_contract_smoke.rs"
PHYSICAL_MOD="target-xteink-x4/src/vaachak_x4/physical/mod.rs"
CONTRACTS_MOD="target-xteink-x4/src/vaachak_x4/contracts/mod.rs"
DOC="docs/architecture/storage-probe-mount-contract.md"

for path in "$SPI_RUNTIME" "$STORAGE_CONTRACT" "$STORAGE_SMOKE" "$PHYSICAL_MOD" "$CONTRACTS_MOD" "$DOC"; do
  require_file "$path"
done

require_rg '^pub mod spi_bus_runtime;' "$PHYSICAL_MOD"
require_rg '^pub mod storage_probe_mount_contract;' "$PHYSICAL_MOD"
require_rg '^pub mod storage_probe_mount_contract_smoke;' "$CONTRACTS_MOD"

require_rg 'struct VaachakStorageProbeMountContract' "$STORAGE_CONTRACT"
require_rg 'enum VaachakStorageLifecycleStep' "$STORAGE_CONTRACT"
require_rg 'RuntimeBoot' "$STORAGE_CONTRACT"
require_rg 'SharedSpiAvailable' "$STORAGE_CONTRACT"
require_rg 'SlowSdIdentification' "$STORAGE_CONTRACT"
require_rg 'CardAvailabilityKnown' "$STORAGE_CONTRACT"
require_rg 'FatVolumeAvailable' "$STORAGE_CONTRACT"
require_rg 'ReadOnlyFacadeAvailable' "$STORAGE_CONTRACT"
require_rg 'struct VaachakStorageLifecycleOwner' "$STORAGE_CONTRACT"
require_rg 'struct VaachakStorageProbeMountReport' "$STORAGE_CONTRACT"
require_rg 'VaachakSpiBusRuntimeBridge' "$STORAGE_CONTRACT"
require_rg 'STORAGE_PROBE_MOUNT_CONTRACT_MARKER' "$STORAGE_CONTRACT"
require_rg 'x4-storage-probe-mount-contract-ok' "$STORAGE_CONTRACT"

require_rg 'ACTIVE_SD_PROBE_OWNER:.*vendor/pulp-os imported runtime' "$STORAGE_CONTRACT"
require_rg 'ACTIVE_SD_MOUNT_OWNER:.*vendor/pulp-os imported runtime' "$STORAGE_CONTRACT"
require_rg 'ACTIVE_FAT_OWNER:.*vendor/pulp-os imported runtime' "$STORAGE_CONTRACT"
require_rg 'ACTIVE_SPI_ARBITRATION_OWNER:.*vendor/pulp-os imported runtime' "$STORAGE_CONTRACT"
require_rg 'ACTIVE_DISPLAY_OWNER:.*vendor/pulp-os imported runtime' "$STORAGE_CONTRACT"
require_rg 'SHARED_SPI_CONTRACT_DOC:.*docs/architecture/spi-bus-runtime-contract.md' "$STORAGE_CONTRACT"

require_rg 'SD_DRIVER_MOVED_TO_VAACHAK: bool = false' "$STORAGE_CONTRACT"
require_rg 'SD_PROBE_BEHAVIOR_MOVED_TO_VAACHAK: bool = false' "$STORAGE_CONTRACT"
require_rg 'SD_MOUNT_BEHAVIOR_MOVED_TO_VAACHAK: bool = false' "$STORAGE_CONTRACT"
require_rg 'FAT_BEHAVIOR_MOVED_TO_VAACHAK: bool = false' "$STORAGE_CONTRACT"
require_rg 'FAT_READ_BEHAVIOR_MOVED_TO_VAACHAK: bool = false' "$STORAGE_CONTRACT"
require_rg 'FAT_WRITE_BEHAVIOR_MOVED_TO_VAACHAK: bool = false' "$STORAGE_CONTRACT"
require_rg 'SPI_ARBITRATION_MOVED_TO_VAACHAK: bool = false' "$STORAGE_CONTRACT"
require_rg 'DISPLAY_BEHAVIOR_MOVED_TO_VAACHAK: bool = false' "$STORAGE_CONTRACT"

require_rg 'STORAGE_SD_CS_GPIO: u8 = 12' "$STORAGE_CONTRACT"
require_rg 'SD_IDENTIFICATION_KHZ: u32 = 400' "$STORAGE_CONTRACT"
require_rg 'OPERATIONAL_SPI_MHZ: u32 = 20' "$STORAGE_CONTRACT"
require_rg 'contract_ok' "$STORAGE_CONTRACT"
require_rg 'no_runtime_behavior_moved' "$STORAGE_CONTRACT"
require_rg 'storage_probe_mount_contract=ok' "$STORAGE_SMOKE"

require_rg 'SD probe/mount lifecycle' "$DOC"
require_rg 'vendor/pulp-os' "$DOC"
require_rg 'docs/architecture/spi-bus-runtime-contract.md' "$DOC"
require_rg 'GPIO12' "$DOC"
require_rg '400 kHz' "$DOC"
require_rg 'Operational speed' "$DOC"
require_rg 'does not move SD, FAT, SPI, display, reader, or file browser behavior' "$DOC"
require_rg 'storage_probe_mount_contract=ok' "$DOC"

# The contract must stay metadata-only. Do not import embedded SD/FAT/SPI/display implementation crates.
reject_rg '^use .*embedded_sdmmc' "$STORAGE_CONTRACT"
reject_rg '^use .*embedded_hal' "$STORAGE_CONTRACT"
reject_rg '^use .*esp_hal' "$STORAGE_CONTRACT"
reject_rg '^use .*x4_kernel' "$STORAGE_CONTRACT"
reject_rg '^use .*pulp' "$STORAGE_CONTRACT"
reject_rg 'SdMmcSpi|VolumeManager|RawVolume|BlockDevice|ExclusiveDevice|Output<' "$STORAGE_CONTRACT"
reject_rg 'ssd1677|draw_iter|refresh|partial_refresh|full_refresh' "$STORAGE_CONTRACT"

# This file may discuss the lifecycle, but it must not define active hardware or FAT operations.
reject_rg 'pub fn (mount|unmount|probe|open|read|write|append|delete|rename|mkdir|format|flush|seek)_' "$STORAGE_CONTRACT"
reject_rg 'fn (mount|unmount|probe|open|read|write|append|delete|rename|mkdir|format|flush|seek)_' "$STORAGE_CONTRACT"
reject_rg '\.(mount|unmount|probe|open|read|write|append|delete|rename|mkdir|format|flush|seek)\(' "$STORAGE_CONTRACT"

# Keep SD/FAT/display implementation out of Vaachak-owned physical contract modules.
for physical_file in target-xteink-x4/src/vaachak_x4/physical/*.rs; do
  case "$physical_file" in
    *spi_bus_runtime.rs|*spi_bus_runtime_contract.rs|*storage_probe_mount_contract.rs) ;;
    *)
      reject_rg 'embedded_sdmmc|SdMmcSpi|VolumeManager|ssd1677|draw_iter|partial_refresh|full_refresh' "$physical_file"
      ;;
  esac
done

# Ensure the active runtime ownership statements still identify vendor/pulp-os as the physical owner.
require_rg 'SD card probing, mounting, and file I/O|SD mount/probe and file I/O behavior|SD/FAT I/O' docs/architecture/controlled-extraction-ownership.md docs/architecture/runtime-adapter-contracts.md docs/architecture/ownership-map.md

printf '%s\n' 'storage_probe_mount_contract=ok'
