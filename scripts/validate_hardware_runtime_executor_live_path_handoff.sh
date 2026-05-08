#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "hardware_runtime_executor_live_path_handoff validation failed: $*" >&2
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
  perl -0ne "BEGIN { \$found = 0 } if (/$pattern/s) { \$found = 1 } END { exit(\$found ? 0 : 1) }" "$file" || fail "missing pattern '$pattern' in $file"
}

SRC="target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor_live_handoff.rs"
SMOKE="target-xteink-x4/src/vaachak_x4/contracts/hardware_runtime_executor_live_handoff_smoke.rs"
BOOT="target-xteink-x4/src/vaachak_x4/boot.rs"
IMPORTED="target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs"
DOC="docs/architecture/hardware-runtime-executor-live-handoff.md"

require_file "$SRC"
require_file "$SMOKE"
require_file "$BOOT"
require_file "$IMPORTED"
require_file "$DOC"

require_text "$SRC" "pub struct VaachakHardwareRuntimeExecutorLiveHandoff"
require_text "$SRC" "hardware_runtime_executor_live_path_handoff=ok"
require_text "$SRC" "VaachakHardwareRuntimeExecutorRuntimeUse::active_runtime_preflight()"
require_text "$SRC" "VaachakHardwareRuntimeExecutorRuntimeUse::runtime_use_ok()"
require_text "$SRC" "VaachakHardwareRuntimeExecutorAcceptance::acceptance_ok()"
require_text "$SRC" "VaachakHardwareExecutorBackend::PulpCompatibility"
require_text "$SRC" "adopt_imported_pulp_reader_runtime_boundary"
require_text "$SRC" "adopt_storage_availability_handoff"
require_text "$SRC" "adopt_display_refresh_handoff"
require_text "$SRC" "adopt_input_runtime_handoff"
require_text "$SRC" "hardware.executor.live_handoff.backend.pulp_compatible"
require_text "$SRC" "hardware.executor.live_handoff.behavior.preserved"

require_text "$SMOKE" "VaachakHardwareRuntimeExecutorLiveHandoffSmoke"
require_text "$SMOKE" "VaachakHardwareRuntimeExecutorLiveHandoff::live_handoff_ok()"

require_text "target-xteink-x4/src/vaachak_x4/physical/mod.rs" "pub mod hardware_runtime_executor_live_handoff;"
require_text "target-xteink-x4/src/vaachak_x4/contracts/mod.rs" "pub mod hardware_runtime_executor_live_handoff_smoke;"

require_text "$BOOT" "emit_hardware_runtime_executor_live_handoff_marker"
require_text "$IMPORTED" "VaachakHardwareRuntimeExecutorLiveHandoff::active_boot_preflight()"
require_text "$IMPORTED" "VaachakHardwareRuntimeExecutorLiveHandoff::adopt_boot_preflight()"
require_text "$IMPORTED" "VaachakHardwareRuntimeExecutorLiveHandoff::adopt_storage_availability_handoff()"
require_text "$IMPORTED" "VaachakHardwareRuntimeExecutorLiveHandoff::adopt_display_refresh_handoff()"
require_text "$IMPORTED" "VaachakHardwareRuntimeExecutorLiveHandoff::adopt_input_runtime_handoff()"
require_text "$IMPORTED" "VaachakHardwareRuntimeExecutorLiveHandoff::adopt_imported_pulp_reader_runtime_boundary()"

require_regex "$IMPORTED" "if\s+crate::vaachak_x4::physical::hardware_runtime_executor_live_handoff::VaachakHardwareRuntimeExecutorLiveHandoff::active_boot_preflight\(\)\s*\{.*emit_hardware_runtime_executor_runtime_use_marker\(\).*emit_hardware_runtime_executor_live_handoff_marker\(\).*\}"

require_regex "$DOC" "PulpCompatibility|Pulp-compatible|pulp_compatible"
require_text "$DOC" "hardware_runtime_executor_live_path_handoff=ok"

require_text "$SRC" "PHYSICAL_SPI_TRANSFER_CHANGED: bool = false"
require_text "$SRC" "CHIP_SELECT_TOGGLING_CHANGED: bool = false"
require_text "$SRC" "SD_MMC_LOW_LEVEL_CHANGED: bool = false"
require_text "$SRC" "FAT_STORAGE_ALGORITHM_CHANGED: bool = false"
require_text "$SRC" "DISPLAY_DRAW_ALGORITHM_CHANGED: bool = false"
require_text "$SRC" "INPUT_DEBOUNCE_NAVIGATION_CHANGED: bool = false"
require_text "$SRC" "READER_FILE_BROWSER_UX_CHANGED: bool = false"
require_text "$SRC" "APP_NAVIGATION_BEHAVIOR_CHANGED: bool = false"
require_text "$SRC" "DESTRUCTIVE_STORAGE_BEHAVIOR_ADDED: bool = false"

if grep -RInE 'pulp_os::apps|FilesApp|ReaderApp|HomeApp|AppManager|SettingsApp|Launcher|QuickMenu|ButtonFeedback' "$SRC" "$SMOKE"; then
  fail "live handoff layer must not import or mutate app/reader/file-browser UX types"
fi

if grep -RInE 'epd\.|ssd1677|partial_refresh\(|full_refresh\(|draw_[a-zA-Z0-9_]*\(|render_glyph\(|read_battery_mv\(|SdStorage::mount|ensure_x4_dir_async|File::|OpenOptions|remove_|rename\(' "$SRC" "$SMOKE"; then
  fail "live handoff layer must not implement display/input/storage algorithms"
fi

if [ -x ./scripts/validate_hardware_runtime_executor_runtime_use.sh ]; then
  ./scripts/validate_hardware_runtime_executor_runtime_use.sh >/dev/null
fi
if [ -x ./scripts/validate_hardware_runtime_executor_runtime_use_cleanup.sh ]; then
  ./scripts/validate_hardware_runtime_executor_runtime_use_cleanup.sh >/dev/null
fi

echo "hardware_runtime_executor_live_path_handoff=ok"
