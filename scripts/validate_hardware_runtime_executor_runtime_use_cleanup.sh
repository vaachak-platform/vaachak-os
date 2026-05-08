#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="${1:-$(pwd)}"
cd "$REPO_ROOT"

fail() {
  printf 'hardware_runtime_executor_runtime_use_cleanup validation failed: %s\n' "$1" >&2
  exit 1
}

require_file() {
  local file="$1"
  [[ -f "$file" ]] || fail "missing file $file"
}

require_text() {
  local file="$1"
  local text="$2"
  require_file "$file"
  grep -Fq "$text" "$file" || fail "missing text '$text' in $file"
}

require_absent_regex() {
  local file="$1"
  local regex="$2"
  require_file "$file"
  if grep -Eq "$regex" "$file"; then
    fail "unexpected pattern '$regex' in $file"
  fi
}

PHYSICAL_MOD='target-xteink-x4/src/vaachak_x4/physical/mod.rs'
CONTRACTS_MOD='target-xteink-x4/src/vaachak_x4/contracts/mod.rs'
RUNTIME_USE='target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor_runtime_use.rs'
RUNTIME_USE_SMOKE='target-xteink-x4/src/vaachak_x4/contracts/hardware_runtime_executor_runtime_use_smoke.rs'
CLEANUP='target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor_runtime_use_cleanup.rs'
CLEANUP_SMOKE='target-xteink-x4/src/vaachak_x4/contracts/hardware_runtime_executor_runtime_use_cleanup_smoke.rs'
DOC='docs/architecture/hardware-runtime-executor-runtime-use-cleanup.md'
RUNTIME_USE_DOC='docs/architecture/hardware-runtime-executor-runtime-use.md'
VALIDATOR='scripts/validate_hardware_runtime_executor_runtime_use_cleanup.sh'
RUNTIME_USE_VALIDATOR='scripts/validate_hardware_runtime_executor_runtime_use.sh'
CLEANUP_SCRIPT='scripts/cleanup_hardware_runtime_executor_runtime_use_artifacts.sh'
BOOT='target-xteink-x4/src/vaachak_x4/boot.rs'
IMPORTED_RUNTIME='target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs'

for f in \
  "$PHYSICAL_MOD" "$CONTRACTS_MOD" "$RUNTIME_USE" "$RUNTIME_USE_SMOKE" \
  "$CLEANUP" "$CLEANUP_SMOKE" "$DOC" "$RUNTIME_USE_DOC" "$VALIDATOR" \
  "$RUNTIME_USE_VALIDATOR" "$CLEANUP_SCRIPT" "$BOOT" "$IMPORTED_RUNTIME"; do
  require_file "$f"
done

require_text "$PHYSICAL_MOD" 'pub mod hardware_runtime_executor_runtime_use;'
require_text "$PHYSICAL_MOD" 'pub mod hardware_runtime_executor_runtime_use_cleanup;'
require_text "$CONTRACTS_MOD" 'pub mod hardware_runtime_executor_runtime_use_smoke;'
require_text "$CONTRACTS_MOD" 'pub mod hardware_runtime_executor_runtime_use_cleanup_smoke;'

require_text "$RUNTIME_USE" 'VaachakHardwareRuntimeExecutorRuntimeUse'
require_text "$RUNTIME_USE" 'hardware_runtime_executor_runtime_use=ok'
require_text "$RUNTIME_USE" 'RUNTIME_USE_SITE_COUNT: usize = 10'
require_text "$RUNTIME_USE" 'ACCEPTANCE_PREFLIGHT_CALL_ANCHOR'
require_text "$RUNTIME_USE" 'BOOT_MARKERS_PREFLIGHT_CALL_ANCHOR'
require_text "$RUNTIME_USE" 'VaachakHardwareRuntimeExecutorWiring::route_path'
require_text "$RUNTIME_USE" 'VaachakHardwareRuntimeExecutor::entry_for'
require_text "$RUNTIME_USE" 'emit_runtime_use_marker'

require_text "$RUNTIME_USE_SMOKE" 'VaachakHardwareRuntimeExecutorRuntimeUseSmoke'
require_text "$RUNTIME_USE_SMOKE" 'hardware_runtime_executor_runtime_use_smoke=ok'
require_text "$RUNTIME_USE_SMOKE" 'REQUIRED_RUNTIME_USE_SITE_COUNT: usize = 10'

require_text "$CLEANUP" 'VaachakHardwareRuntimeExecutorRuntimeUseCleanup'
require_text "$CLEANUP" 'hardware_runtime_executor_runtime_use_cleanup=ok'
require_text "$CLEANUP" 'RUNTIME_USE_VALIDATOR_FIX_FOLDED_IN: bool = true;'
require_text "$CLEANUP" 'REQUIRED_RUNTIME_USE_SITE_COUNT: usize = 10;'
require_text "$CLEANUP" 'VaachakHardwareRuntimeExecutorRuntimeUse::runtime_use_ok()'
require_text "$CLEANUP" 'cleanup_ok'

require_text "$CLEANUP_SMOKE" 'VaachakHardwareRuntimeExecutorRuntimeUseCleanupSmoke'
require_text "$CLEANUP_SMOKE" 'VaachakHardwareRuntimeExecutorRuntimeUseCleanup::cleanup_ok()'
require_text "$CLEANUP_SMOKE" 'VaachakHardwareRuntimeExecutorRuntimeUse::runtime_use_ok()'

require_text "$CLEANUP_SCRIPT" 'hardware_runtime_executor_runtime_use_cleanup_artifacts=ok'
require_text "$CLEANUP_SCRIPT" 'hardware_runtime_executor_runtime_use_validator_fix'
require_text "$CLEANUP_SCRIPT" 'rm -rf -- "$path"'

require_text "$DOC" 'hardware_runtime_executor_runtime_use_cleanup=ok'
require_text "$DOC" 'hardware_runtime_executor_runtime_use=ok'
require_text "$DOC" 'PulpCompatibility'
require_text "$DOC" 'cargo fmt --all'
require_text "$DOC" 'cargo build'
require_text "$RUNTIME_USE_DOC" 'hardware_runtime_executor_runtime_use_cleanup=ok'

require_text "$RUNTIME_USE_VALIDATOR" 'hardware_runtime_executor_runtime_use=ok'
require_text "$VALIDATOR" 'hardware_runtime_executor_runtime_use_cleanup=ok'

require_text "$BOOT" 'emit_hardware_runtime_executor_runtime_use_marker'
require_text "$IMPORTED_RUNTIME" 'emit_hardware_runtime_executor_runtime_use_marker'
require_text "$IMPORTED_RUNTIME" 'active_runtime_preflight'
require_text "$IMPORTED_RUNTIME" 'adopt_boot_executor_preflight'
require_text "$IMPORTED_RUNTIME" 'adopt_input_task_handoff'

for file in "$RUNTIME_USE" "$CLEANUP" "$CLEANUP_SMOKE"; do
  require_absent_regex "$file" 'pub[[:space:]]+(const[[:space:]]+)?fn[[:space:]]+(transfer|toggle|mount|probe|format|write|append|delete|rename|mkdir|draw|refresh|scan|debounce)[[:space:]]*\('
  require_absent_regex "$file" '(embedded_sdmmc|embedded_hal::|embedded_hal_bus|x4_kernel::drivers::storage|x4_kernel::drivers::display|x4_kernel::drivers::input|BlockDevice|VolumeManager|SpiDevice|ExclusiveDevice|draw_packed_pixels|paint_stack\(|set_pixels\(|wait_until_idle\()'
done

python3 - <<'PY'
from pathlib import Path

def compact(s: str) -> str:
    return ''.join(s.split())

runtime_use_path = Path('target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor_runtime_use.rs')
runtime_use = runtime_use_path.read_text()
runtime_compact = compact(runtime_use)
for call in [
    'VaachakHardwareRuntimeExecutorAcceptance::acceptance_ok()',
    'VaachakHardwareRuntimeExecutorBootMarkers::boot_markers_ok()',
]:
    if compact(call) not in runtime_compact:
        raise SystemExit(f'hardware_runtime_executor_runtime_use_cleanup validation failed: missing preflight call {call}')

cleanup = Path('target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor_runtime_use_cleanup.rs').read_text()
cleanup_compact = compact(cleanup)
for call in [
    'VaachakHardwareRuntimeExecutorRuntimeUse::runtime_use_ok()',
    'VaachakHardwareRuntimeExecutorAcceptance::acceptance_ok()',
    'VaachakHardwareRuntimeExecutorBootMarkers::boot_markers_ok()',
]:
    if compact(call) not in cleanup_compact:
        raise SystemExit(f'hardware_runtime_executor_runtime_use_cleanup validation failed: missing cleanup preflight call {call}')

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
for source_name, source in [('runtime-use', runtime_use), ('cleanup', cleanup)]:
    source_compact = compact(source)
    for name in required_false:
        needle = f'pub const {name}: bool = false;'
        if compact(needle) not in source_compact:
            raise SystemExit(f'hardware_runtime_executor_runtime_use_cleanup validation failed: {source_name} {name} is not explicitly false')

runtime = Path('target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs').read_text()
call_count = runtime.count('VaachakHardwareRuntimeExecutorRuntimeUse::')
if call_count < 11:
    raise SystemExit(f'hardware_runtime_executor_runtime_use_cleanup validation failed: expected at least 11 runtime-use callsites, found {call_count}')
for anchor in ['Board::init(peripherals)', 'SdStorage::mount(card).await', 'InputDriver::new(board.input)']:
    if anchor not in runtime:
        raise SystemExit(f'hardware_runtime_executor_runtime_use_cleanup validation failed: existing Pulp runtime anchor missing: {anchor}')
PY

printf '%s\n' 'hardware_runtime_executor_runtime_use_cleanup=ok'
