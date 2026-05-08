#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="${1:-$(pwd)}"
cd "$REPO_ROOT"

fail() {
  echo "display_runtime_owner validation failed: $*" >&2
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

OWNER="target-xteink-x4/src/vaachak_x4/physical/display_runtime_owner.rs"
BACKEND="target-xteink-x4/src/vaachak_x4/physical/display_pulp_backend.rs"
SMOKE="target-xteink-x4/src/vaachak_x4/contracts/display_runtime_ownership_smoke.rs"
PHYSICAL_MOD="target-xteink-x4/src/vaachak_x4/physical/mod.rs"
CONTRACTS_MOD="target-xteink-x4/src/vaachak_x4/contracts/mod.rs"
SPI_OWNER="target-xteink-x4/src/vaachak_x4/physical/spi_bus_runtime_owner.rs"
SPI_BACKEND="target-xteink-x4/src/vaachak_x4/physical/spi_bus_pulp_backend.rs"
SD_FAT_OWNER="target-xteink-x4/src/vaachak_x4/physical/sd_fat_runtime_readonly_owner.rs"
PROBE_OWNER="target-xteink-x4/src/vaachak_x4/physical/storage_probe_mount_runtime_owner.rs"
DOC="docs/architecture/display-runtime-ownership.md"
SPI_DOC="docs/architecture/spi-bus-runtime-ownership.md"

for path in "$OWNER" "$BACKEND" "$SMOKE" "$PHYSICAL_MOD" "$CONTRACTS_MOD" "$SPI_OWNER" "$SPI_BACKEND" "$DOC" "$SPI_DOC"; do
  require_file "$path"
done

require_rg '^pub mod display_pulp_backend;' "$PHYSICAL_MOD"
require_rg '^pub mod display_runtime_owner;' "$PHYSICAL_MOD"
require_rg '^pub mod display_runtime_ownership_smoke;' "$CONTRACTS_MOD"

require_rg 'struct VaachakDisplayRuntimeOwner' "$OWNER"
require_rg 'DISPLAY_RUNTIME_OWNERSHIP_MARKER' "$OWNER"
require_rg 'x4-display-runtime-owner-ok' "$OWNER"
require_rg 'DISPLAY_RUNTIME_IDENTITY' "$OWNER"
require_rg 'xteink-x4-ssd1677-display-runtime' "$OWNER"
require_rg 'DISPLAY_RUNTIME_OWNERSHIP_AUTHORITY' "$OWNER"
require_rg 'target-xteink-x4 Vaachak layer' "$OWNER"
require_rg 'DISPLAY_RUNTIME_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK: bool = true' "$OWNER"
require_rg 'PULP_COMPATIBILITY_BACKEND' "$OWNER"
require_rg 'VaachakDisplayRuntimeBackend::PulpCompatibility' "$OWNER"
require_rg 'ACTIVE_BACKEND' "$OWNER"
require_rg 'Self::PULP_COMPATIBILITY_BACKEND' "$OWNER"
require_rg 'ACTIVE_BACKEND_NAME' "$OWNER"
require_rg 'VaachakDisplayPulpBackend::BACKEND_NAME' "$OWNER"
require_rg 'ACTIVE_EXECUTOR_OWNER' "$OWNER"
require_rg 'vendor/pulp-os imported runtime' "$OWNER"
require_rg 'DISPLAY_PANEL_NAME' "$OWNER"
require_rg 'SSD1677 e-paper display' "$OWNER"
require_rg 'EPD_CS_GPIO: u8 = 21' "$OWNER"
require_rg 'EPD_DC_GPIO: u8 = 4' "$OWNER"
require_rg 'EPD_RST_GPIO: u8 = 5' "$OWNER"
require_rg 'EPD_BUSY_GPIO: u8 = 6' "$OWNER"
require_rg 'SPI_SCLK_GPIO: u8 = 8' "$OWNER"
require_rg 'SPI_MOSI_GPIO: u8 = 10' "$OWNER"
require_rg 'SPI_MISO_GPIO: u8 = 7' "$OWNER"
require_rg 'STORAGE_SD_CS_GPIO: u8 = 12' "$OWNER"
require_rg 'NATIVE_WIDTH: u16 = 800' "$OWNER"
require_rg 'NATIVE_HEIGHT: u16 = 480' "$OWNER"
require_rg 'LOGICAL_WIDTH: u16 = 480' "$OWNER"
require_rg 'LOGICAL_HEIGHT: u16 = 800' "$OWNER"
require_rg 'ROTATION_DEGREES: u16 = 270' "$OWNER"
require_rg 'STRIP_ROWS: u16 = 40' "$OWNER"
require_rg 'SSD1677_WRITE_RAM_CMD: u8 = 0x24' "$OWNER"
require_rg 'SSD1677_WRITE_PREVIOUS_RAM_CMD: u8 = 0x26' "$OWNER"
require_rg 'SSD1677_DISPLAY_UPDATE_CONTROL_2_CMD: u8 = 0x22' "$OWNER"
require_rg 'SSD1677_MASTER_ACTIVATION_CMD: u8 = 0x20' "$OWNER"
require_rg 'VaachakSpiBusRuntimeOwner' "$OWNER"
require_rg 'VaachakSpiRuntimeUser::Display' "$OWNER"
require_rg 'VaachakSpiTransactionKind::DisplayRefreshMetadata' "$OWNER"
require_rg 'shared_spi_dependency_ready' "$OWNER"
require_rg 'display_user_registered_on_spi' "$OWNER"
require_rg 'operation_metadata_is_safe' "$OWNER"
require_rg 'ownership_ok' "$OWNER"
require_rg 'SSD1677_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$OWNER"
require_rg 'DISPLAY_DRAW_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$OWNER"
require_rg 'DISPLAY_REFRESH_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$OWNER"
require_rg 'DISPLAY_PARTIAL_REFRESH_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$OWNER"
require_rg 'DISPLAY_BUSY_WAIT_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$OWNER"
require_rg 'DISPLAY_SPI_TRANSACTION_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$OWNER"
require_rg 'STORAGE_BEHAVIOR_CHANGED: bool = false' "$OWNER"
require_rg 'READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false' "$OWNER"

require_rg 'struct VaachakDisplayPulpBackend' "$BACKEND"
require_rg 'BACKEND_NAME' "$BACKEND"
require_rg 'PulpCompatibility' "$BACKEND"
require_rg 'ACTIVE_HARDWARE_EXECUTOR: bool = true' "$BACKEND"
require_rg 'ACTIVE_SSD1677_EXECUTOR_OWNER' "$BACKEND"
require_rg 'ACTIVE_DRAW_EXECUTOR_OWNER' "$BACKEND"
require_rg 'ACTIVE_FULL_REFRESH_EXECUTOR_OWNER' "$BACKEND"
require_rg 'ACTIVE_PARTIAL_REFRESH_EXECUTOR_OWNER' "$BACKEND"
require_rg 'ACTIVE_BUSY_WAIT_EXECUTOR_OWNER' "$BACKEND"
require_rg 'ACTIVE_ROTATION_EXECUTOR_OWNER' "$BACKEND"
require_rg 'ACTIVE_STRIP_RENDER_EXECUTOR_OWNER' "$BACKEND"
require_rg 'vendor/pulp-os imported runtime' "$BACKEND"
require_rg 'SSD1677_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$BACKEND"
require_rg 'DRAW_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$BACKEND"
require_rg 'REFRESH_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$BACKEND"
require_rg 'PARTIAL_REFRESH_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$BACKEND"
require_rg 'SPI_TRANSACTION_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$BACKEND"
require_rg 'STORAGE_BEHAVIOR_CHANGED: bool = false' "$BACKEND"
require_rg 'READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false' "$BACKEND"
require_rg 'bridge_ok' "$BACKEND"

require_rg 'struct VaachakDisplayRuntimeOwnershipSmoke' "$SMOKE"
require_rg 'x4-display-runtime-ownership-smoke-ok' "$SMOKE"
require_rg 'VaachakDisplayRuntimeOwner::ownership_ok' "$SMOKE"
require_rg 'VaachakDisplayPulpBackend::bridge_ok' "$SMOKE"
require_rg 'DISPLAY_RUNTIME_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK: bool = true' "$SMOKE"
require_rg 'PULP_COMPATIBILITY_BACKEND_ACTIVE: bool = true' "$SMOKE"
require_rg 'SSD1677_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$SMOKE"
require_rg 'DISPLAY_DRAW_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$SMOKE"
require_rg 'DISPLAY_REFRESH_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$SMOKE"
require_rg 'DISPLAY_PARTIAL_REFRESH_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$SMOKE"
require_rg 'DISPLAY_SPI_TRANSACTION_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$SMOKE"
require_rg 'STORAGE_BEHAVIOR_CHANGED: bool = false' "$SMOKE"
require_rg 'READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false' "$SMOKE"
require_rg 'full_refresh_metadata_is_safe' "$SMOKE"
require_rg 'partial_refresh_metadata_is_safe' "$SMOKE"

require_rg 'Display Runtime Ownership' "$DOC"
require_rg 'display_runtime_owner=ok' "$DOC"
require_rg 'DISPLAY_RUNTIME_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK = true' "$DOC"
require_rg 'PulpCompatibility' "$DOC"
require_rg 'vendor/pulp-os.*imported runtime' "$DOC"
require_rg 'VaachakSpiBusRuntimeOwner::ownership_bridge_ok' "$DOC"
require_rg 'CS GPIO21' "$DOC"
require_rg 'DC GPIO4' "$DOC"
require_rg 'RST GPIO5' "$DOC"
require_rg 'BUSY GPIO6' "$DOC"
require_rg 'SCLK GPIO8' "$DOC"
require_rg 'MOSI GPIO10' "$DOC"
require_rg 'MISO GPIO7' "$DOC"
require_rg 'GPIO12' "$DOC"
require_rg '800x480' "$DOC"
require_rg '480x800' "$DOC"
require_rg 'full-refresh' "$DOC"
require_rg 'partial-refresh' "$DOC"
require_rg 'draw/refresh' "$DOC"
require_rg 'SD/FAT' "$DOC"
require_rg 'reader' "$DOC"
require_rg 'file browser' "$DOC"

require_rg 'SPI_BUS_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK: bool = true' "$SPI_OWNER"
require_rg 'display_user_registered' "$SPI_OWNER"
require_rg 'PulpCompatibility' "$SPI_BACKEND"

if [[ -f "$SD_FAT_OWNER" ]]; then
  require_rg 'ownership_ok' "$SD_FAT_OWNER"
fi
if [[ -f "$PROBE_OWNER" ]]; then
  require_rg 'ownership_ok' "$PROBE_OWNER"
fi

# The display owner/backend/smoke must remain metadata-only and must not call/import concrete display/SPI/storage behavior.
if rg -n '(embedded_graphics|embedded_hal::|embedded_hal_bus|esp_hal::|epd_waveshare|x4_kernel::drivers::display|x4_kernel::drivers::storage|DisplayInterface|SpiDevice|ExclusiveDevice|FrameBuffer|Board::init|speed_up_spi|init_spi|paint_stack\(|draw_packed_pixels\(|set_pixels\(|flush\(|wait_until_idle\(|display\.epd|partial_update|full_update)' "$OWNER" "$BACKEND" "$SMOKE"; then
  fail "display runtime owner must not import or call SSD1677, SPI peripheral, storage, or display execution behavior"
fi

# This slice can name metadata operations, but it must not expose executor functions for display/storage/SPI behavior.
if rg -n '\bfn +(draw|refresh|partial_refresh|full_refresh|flush|paint|set_pixels|wait_until_idle|transfer|toggle|select|deselect|lock|unlock|write|append|delete|remove|rename|truncate|mkdir|create|open|close|format|init_card|init_spi|speed_up_spi|mount|unmount|probe)\b' "$OWNER" "$BACKEND" "$SMOKE"; then
  fail "display runtime owner exposes direct display, SPI, storage, reader, or file-browser behavior"
fi

# Do not edit active runtime/app/vendor/storage/display UI behavior in this overlay.
if [[ -d display_runtime_owner ]]; then
  if find display_runtime_owner -type f | rg -n '(^|/)(vendor/pulp-os|vendor/smol-epub|src/apps|target-xteink-x4/src/vaachak_x4/ui|target-xteink-x4/src/vaachak_x4/io|target-xteink-x4/src/vaachak_x4/display/display_geometry_runtime\.rs)/' >/dev/null; then
    fail "overlay includes vendor/app/ui/io/display runtime behavior files; this slice must only add display runtime ownership files"
  fi
fi

printf '%s\n' 'display_runtime_owner=ok'
