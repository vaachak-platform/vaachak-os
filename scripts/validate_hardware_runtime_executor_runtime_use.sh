#!/usr/bin/env bash
set -euo pipefail

fail() {
  printf '%s\n' "hardware_runtime_executor_runtime_use validation failed: $*" >&2
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

require_absent_regex() {
  local file="$1"
  local regex="$2"
  if grep -Eq "$regex" "$file"; then
    fail "unexpected pattern '$regex' in $file"
  fi
}

RUNTIME_USE="target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor_runtime_use.rs"
SMOKE="target-xteink-x4/src/vaachak_x4/contracts/hardware_runtime_executor_runtime_use_smoke.rs"
DOC="docs/architecture/hardware-runtime-executor-runtime-use.md"
PHYSICAL_MOD="target-xteink-x4/src/vaachak_x4/physical/mod.rs"
CONTRACTS_MOD="target-xteink-x4/src/vaachak_x4/contracts/mod.rs"
BOOT="target-xteink-x4/src/vaachak_x4/boot.rs"
RUNTIME="target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs"

require_file "$RUNTIME_USE"
require_file "$SMOKE"
require_file "$DOC"
require_file "$PHYSICAL_MOD"
require_file "$CONTRACTS_MOD"
require_file "$BOOT"
require_file "$RUNTIME"

require_text "$PHYSICAL_MOD" "pub mod hardware_runtime_executor_runtime_use;"
require_text "$CONTRACTS_MOD" "pub mod hardware_runtime_executor_runtime_use_smoke;"

require_text "$RUNTIME_USE" "VaachakHardwareRuntimeExecutorRuntimeUse"
require_text "$RUNTIME_USE" "hardware_runtime_executor_runtime_use=ok"
require_text "$RUNTIME_USE" "RUNTIME_USE_SITE_COUNT: usize = 10"
require_text "$RUNTIME_USE" "VaachakHardwareExecutorBackend::PulpCompatibility"
require_text "$RUNTIME_USE" "VaachakHardwareRuntimeExecutorAcceptance"
require_text "$RUNTIME_USE" "VaachakHardwareRuntimeExecutorBootMarkers"
require_text "$RUNTIME_USE" "VaachakHardwareRuntimeExecutorWiring::route_path"
require_text "$RUNTIME_USE" "VaachakHardwareRuntimeExecutor::entry_for"
require_text "$RUNTIME_USE" "emit_runtime_use_marker"

require_text "$SMOKE" "VaachakHardwareRuntimeExecutorRuntimeUseSmoke"
require_text "$SMOKE" "hardware_runtime_executor_runtime_use_smoke=ok"
require_text "$SMOKE" "REQUIRED_RUNTIME_USE_SITE_COUNT: usize = 10"
require_text "$SMOKE" "smoke_ok"

require_text "$BOOT" "emit_hardware_runtime_executor_runtime_use_marker"
require_text "$RUNTIME" "emit_hardware_runtime_executor_runtime_use_marker"
require_text "$RUNTIME" "active_runtime_preflight"
require_text "$RUNTIME" "adopt_boot_executor_preflight"
require_text "$RUNTIME" "adopt_board_spi_ownership_handoff"
require_text "$RUNTIME" "adopt_display_init_handoff"
require_text "$RUNTIME" "adopt_display_refresh_handoff"
require_text "$RUNTIME" "adopt_storage_card_detect_handoff"
require_text "$RUNTIME" "adopt_storage_mount_handoff"
require_text "$RUNTIME" "adopt_storage_directory_listing_handoff"
require_text "$RUNTIME" "adopt_reader_file_open_handoff"
require_text "$RUNTIME" "adopt_input_driver_init_handoff"
require_text "$RUNTIME" "adopt_input_task_handoff"

require_text "$DOC" "hardware_runtime_executor_runtime_use=ok"
require_text "$DOC" "PulpCompatibility"
require_text "$DOC" "Runtime-use call sites"

# The runtime-use layer must not introduce low-level hardware/FAT/display/input executors.
require_absent_regex "$RUNTIME_USE" "fn[[:space:]]+(transfer|toggle|mount|probe|format|write|append|delete|rename|mkdir|draw|refresh|scan|debounce)[[:space:]]*\\("
require_absent_regex "$RUNTIME_USE" "struct[[:space:]]+(SpiTransfer|SdCard|FatVolume|Ssd1677|InputDriver)"

# The wiring must stay out of app and vendor behavior for this slice.
if grep -R "hardware_runtime_executor_runtime_use" src target-xteink-x4/src/apps vendor/pulp-os 2>/dev/null | grep -v "target-xteink-x4/src/vaachak_x4" >/dev/null; then
  fail "runtime-use wiring leaked into app/vendor behavior"
fi

python3 - <<'PY'
from pathlib import Path
runtime_use = Path('target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor_runtime_use.rs').read_text()
compact = ''.join(runtime_use.split())

# Check the real preflight calls in a whitespace-insensitive way. The source may
# contain stable anchor constants too, but the functional calls must remain in
# the report path.
required_calls = [
    'VaachakHardwareRuntimeExecutorAcceptance::acceptance_ok()',
    'VaachakHardwareRuntimeExecutorBootMarkers::boot_markers_ok()',
]
for call in required_calls:
    if ''.join(call.split()) not in compact:
        raise SystemExit(f"hardware_runtime_executor_runtime_use validation failed: missing preflight call {call}")

required_false = [
    'PHYSICAL_SPI_TRANSFER_MOVED',
    'CHIP_SELECT_TOGGLING_MOVED',
    'SD_MMC_EXECUTOR_MOVED',
    'FAT_EXECUTOR_REWRITTEN',
    'DISPLAY_DRAW_ALGORITHM_REWRITTEN',
    'INPUT_DEBOUNCE_NAVIGATION_REWRITTEN',
    'READER_FILE_BROWSER_UX_CHANGED',
    'APP_NAVIGATION_BEHAVIOR_CHANGED',
    'FAT_DESTRUCTIVE_BEHAVIOR_INTRODUCED',
]
for name in required_false:
    needle = f'pub const {name}: bool = false;'
    if ''.join(needle.split()) not in compact:
        raise SystemExit(f"hardware_runtime_executor_runtime_use validation failed: {name} is not explicitly false")

runtime = Path('target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs').read_text()
call_count = runtime.count('VaachakHardwareRuntimeExecutorRuntimeUse::')
if call_count < 11:
    raise SystemExit(f"hardware_runtime_executor_runtime_use validation failed: expected at least 11 runtime-use callsites, found {call_count}")

if 'Board::init(peripherals)' not in runtime or 'SdStorage::mount(card).await' not in runtime or 'InputDriver::new(board.input)' not in runtime:
    raise SystemExit('hardware_runtime_executor_runtime_use validation failed: existing Pulp runtime behavior anchors missing')
PY

printf '%s\n' 'hardware_runtime_executor_runtime_use=ok'
