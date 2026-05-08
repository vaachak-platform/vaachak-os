#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="${1:-$(pwd)}"
cd "$REPO_ROOT"

fail() {
  echo "spi_bus_arbitration_runtime_owner validation failed: $*" >&2
  exit 1
}

require_file() {
  local path="$1"
  [[ -f "$path" ]] || fail "missing required file: $path"
}

require_rg() {
  local pattern="$1"
  local path="$2"
  rg -n "$pattern" "$path" >/dev/null || fail "missing pattern '$pattern' in $path"
}

OWNER="target-xteink-x4/src/vaachak_x4/physical/spi_bus_arbitration_runtime_owner.rs"
BACKEND="target-xteink-x4/src/vaachak_x4/physical/spi_bus_arbitration_pulp_backend.rs"
SMOKE="target-xteink-x4/src/vaachak_x4/contracts/spi_bus_arbitration_runtime_ownership_smoke.rs"
PHYSICAL_MOD="target-xteink-x4/src/vaachak_x4/physical/mod.rs"
CONTRACTS_MOD="target-xteink-x4/src/vaachak_x4/contracts/mod.rs"
SPI_OWNER="target-xteink-x4/src/vaachak_x4/physical/spi_bus_runtime_owner.rs"
DOC="docs/architecture/spi-bus-arbitration-runtime-ownership.md"
SPI_DOC="docs/architecture/spi-bus-runtime-ownership.md"
HARDWARE_DOC="docs/architecture/hardware-runtime-ownership.md"

for path in "$OWNER" "$BACKEND" "$SMOKE" "$PHYSICAL_MOD" "$CONTRACTS_MOD" "$SPI_OWNER" "$DOC"; do
  require_file "$path"
done

require_rg '^pub mod spi_bus_arbitration_pulp_backend;' "$PHYSICAL_MOD"
require_rg '^pub mod spi_bus_arbitration_runtime_owner;' "$PHYSICAL_MOD"
require_rg '^pub mod spi_bus_arbitration_runtime_ownership_smoke;' "$CONTRACTS_MOD"

require_rg 'struct VaachakSpiBusArbitrationRuntimeOwner' "$OWNER"
require_rg 'SPI_BUS_ARBITRATION_RUNTIME_OWNER_MARKER' "$OWNER"
require_rg 'spi_bus_arbitration_runtime_owner=ok' "$OWNER"
require_rg 'SPI_BUS_ARBITRATION_RUNTIME_IDENTITY' "$OWNER"
require_rg 'xteink-x4-shared-spi-arbitration-runtime' "$OWNER"
require_rg 'SPI_BUS_ARBITRATION_OWNERSHIP_AUTHORITY' "$OWNER"
require_rg 'target-xteink-x4 Vaachak layer' "$OWNER"
require_rg 'SPI_BUS_ARBITRATION_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK: bool = true' "$OWNER"
require_rg 'SPI_BUS_ARBITRATION_POLICY_MOVED_TO_VAACHAK: bool = true' "$OWNER"
require_rg 'VaachakSpiArbitrationRuntimeBackend::PulpCompatibility' "$OWNER"
require_rg 'ACTIVE_BACKEND_NAME' "$OWNER"
require_rg 'VaachakSpiArbitrationPulpBackend::BACKEND_NAME' "$OWNER"
require_rg 'ACTIVE_PHYSICAL_EXECUTOR_OWNER' "$OWNER"
require_rg 'VaachakSpiBusRuntimeOwner::ownership_bridge_ok' "$OWNER"
require_rg 'VaachakSpiRuntimeUser::Display' "$OWNER"
require_rg 'VaachakSpiRuntimeUser::Storage' "$OWNER"
require_rg 'VaachakSpiTransactionKind::DisplayRefreshMetadata' "$OWNER"
require_rg 'VaachakSpiTransactionKind::StorageProbeMetadata' "$OWNER"
require_rg 'VaachakSpiTransactionKind::StorageMountMetadata' "$OWNER"
require_rg 'VaachakSpiTransactionKind::StorageFatIoMetadata' "$OWNER"
require_rg 'DISPLAY_CS_GPIO' "$OWNER"
require_rg 'STORAGE_SD_CS_GPIO' "$OWNER"
require_rg 'PHYSICAL_SPI_TRANSFER_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$OWNER"
require_rg 'CHIP_SELECT_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$OWNER"
require_rg 'DISPLAY_BEHAVIOR_CHANGED: bool = false' "$OWNER"
require_rg 'STORAGE_BEHAVIOR_CHANGED: bool = false' "$OWNER"
require_rg 'SD_PROBE_MOUNT_BEHAVIOR_CHANGED: bool = false' "$OWNER"
require_rg 'SD_FAT_BEHAVIOR_CHANGED: bool = false' "$OWNER"
require_rg 'READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false' "$OWNER"
require_rg 'request_for' "$OWNER"
require_rg 'grant_for' "$OWNER"
require_rg 'runtime_owner_ok' "$OWNER"

require_rg 'struct VaachakSpiArbitrationPulpBackend' "$BACKEND"
require_rg 'BACKEND_NAME' "$BACKEND"
require_rg 'PulpCompatibility' "$BACKEND"
require_rg 'ACTIVE_PHYSICAL_EXECUTOR: bool = true' "$BACKEND"
require_rg 'ACTIVE_PHYSICAL_EXECUTOR_OWNER' "$BACKEND"
require_rg 'vendor/pulp-os imported runtime' "$BACKEND"
require_rg 'SPI_TRANSFER_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$BACKEND"
require_rg 'CHIP_SELECT_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$BACKEND"
require_rg 'DISPLAY_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$BACKEND"
require_rg 'SD_PROBE_MOUNT_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$BACKEND"
require_rg 'SD_FAT_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$BACKEND"
require_rg 'READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false' "$BACKEND"
require_rg 'backend_ok' "$BACKEND"

require_rg 'struct VaachakSpiBusArbitrationRuntimeOwnershipSmoke' "$SMOKE"
require_rg 'x4-spi-bus-arbitration-runtime-ownership-smoke-ok' "$SMOKE"
require_rg 'VaachakSpiBusArbitrationRuntimeOwner::runtime_owner_ok' "$SMOKE"
require_rg 'VaachakSpiArbitrationPulpBackend::backend_ok' "$SMOKE"
require_rg 'SPI_BUS_ARBITRATION_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK: bool = true' "$SMOKE"
require_rg 'SPI_BUS_ARBITRATION_POLICY_MOVED_TO_VAACHAK: bool = true' "$SMOKE"
require_rg 'PHYSICAL_SPI_TRANSFER_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$SMOKE"
require_rg 'CHIP_SELECT_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$SMOKE"
require_rg 'DISPLAY_BEHAVIOR_CHANGED: bool = false' "$SMOKE"
require_rg 'STORAGE_BEHAVIOR_CHANGED: bool = false' "$SMOKE"
require_rg 'READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false' "$SMOKE"
require_rg 'smoke_ok' "$SMOKE"

require_rg 'SPI Bus Arbitration Runtime Ownership' "$DOC"
require_rg 'spi_bus_arbitration_runtime_owner=ok' "$DOC"
require_rg 'SPI_BUS_ARBITRATION_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK = true' "$DOC"
require_rg 'SPI_BUS_ARBITRATION_POLICY_MOVED_TO_VAACHAK = true' "$DOC"
require_rg 'PulpCompatibility' "$DOC"
require_rg 'VaachakSpiBusRuntimeOwner::ownership_bridge_ok' "$DOC"
require_rg 'SCLK GPIO8' "$DOC"
require_rg 'MOSI GPIO10' "$DOC"
require_rg 'MISO GPIO7' "$DOC"
require_rg 'Display CS GPIO21' "$DOC"
require_rg 'SD CS GPIO12' "$DOC"
require_rg 'draw/full-refresh/partial-refresh' "$DOC"
require_rg 'SD probe/mount' "$DOC"
require_rg 'reader/file-browser' "$DOC"

require_rg 'SPI_BUS_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK: bool = true' "$SPI_OWNER"
require_rg 'VaachakSpiRuntimeUser::Display' "$SPI_OWNER"
require_rg 'VaachakSpiRuntimeUser::Storage' "$SPI_OWNER"

# This slice may add Vaachak arbitration request/grant metadata only. It must not import or call physical SPI/display/storage executors.
if rg -n '(embedded_hal|embedded_hal_bus|esp_hal|x4_kernel::drivers|ExclusiveDevice|SpiDevice|OutputPin|InputPin|PinDriver|SdCard|VolumeManager|FileSystem|DisplayInterface|FrameBuffer|Board::init|init_spi|speed_up_spi|paint_stack\(|draw_packed_pixels\(|set_pixels\(|flush\(|wait_until_idle\(|partial_update|full_update|read_file|write_file|open_file|list_dir)' "$OWNER" "$BACKEND" "$SMOKE"; then
  fail "SPI arbitration runtime owner must not import or call physical SPI, display, storage, reader, or file-browser executors"
fi

# Hardware execution verbs remain forbidden as public function names in this owner/backend/smoke.
if rg -n '\bfn +(transfer|toggle|select|deselect|lock|unlock|write|append|delete|remove|rename|truncate|mkdir|create|open|close|format|init_card|init_spi|speed_up_spi|mount|unmount|probe|draw|refresh|partial_refresh|full_refresh|flush|paint|set_pixels|wait_until_idle)\b' "$OWNER" "$BACKEND" "$SMOKE"; then
  fail "SPI arbitration runtime owner exposes direct hardware/display/storage/reader behavior"
fi

# The overlay must not edit active runtime/app/vendor/storage/display behavior.
if [[ -d spi_bus_arbitration_runtime_owner ]]; then
  if find spi_bus_arbitration_runtime_owner -type f | rg -n '(^|/)(vendor/pulp-os|vendor/smol-epub|src/apps|target-xteink-x4/src/vaachak_x4/ui|target-xteink-x4/src/vaachak_x4/io|target-xteink-x4/src/vaachak_x4/display)/' >/dev/null; then
    fail "overlay includes vendor/app/ui/io/display behavior files; this slice must only add SPI arbitration ownership files"
  fi
fi

if [[ -f "$SPI_DOC" ]]; then
  require_rg 'spi-bus-arbitration-runtime-ownership.md' "$SPI_DOC"
fi
if [[ -f "$HARDWARE_DOC" ]]; then
  require_rg 'spi-bus-arbitration-runtime-ownership.md' "$HARDWARE_DOC"
fi

printf '%s\n' 'spi_bus_arbitration_runtime_owner=ok'
