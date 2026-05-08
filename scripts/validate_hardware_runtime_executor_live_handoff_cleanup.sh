#!/bin/sh
set -eu

fail() {
  echo "hardware_runtime_executor_live_handoff_cleanup validation failed: $1" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing file $1"
}

require_text() {
  file="$1"
  text="$2"
  if ! grep -Fq "$text" "$file"; then
    fail "missing text '$text' in $file"
  fi
}

require_regex() {
  file="$1"
  regex="$2"
  perl -0ne "BEGIN { \$found = 0 } if (/$regex/s) { \$found = 1 } END { exit(\$found ? 0 : 1) }" "$file" || fail "missing pattern '$regex' in $file"
}

require_regex_absent() {
  file="$1"
  regex="$2"
  if grep -Eq "$regex" "$file"; then
    fail "unexpected pattern '$regex' in $file"
  fi
}

PHYS="target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor_live_handoff_cleanup.rs"
SMOKE="target-xteink-x4/src/vaachak_x4/contracts/hardware_runtime_executor_live_handoff_cleanup_smoke.rs"
DOC="docs/architecture/hardware-runtime-executor-live-handoff-cleanup.md"
LIVE_DOC="docs/architecture/hardware-runtime-executor-live-handoff.md"
PHYS_MOD="target-xteink-x4/src/vaachak_x4/physical/mod.rs"
CONTRACTS_MOD="target-xteink-x4/src/vaachak_x4/contracts/mod.rs"
BOOT="target-xteink-x4/src/vaachak_x4/boot.rs"
PULP_RUNTIME="target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs"
LIVE="target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor_live_handoff.rs"
RUNTIME_USE="target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor_runtime_use.rs"

require_file "$PHYS"
require_file "$SMOKE"
require_file "$DOC"
require_file "$LIVE_DOC"
require_file "$LIVE"
require_file "$RUNTIME_USE"
require_file "scripts/cleanup_hardware_runtime_executor_live_handoff_artifacts.sh"

require_text "$PHYS_MOD" "pub mod hardware_runtime_executor_live_handoff_cleanup;"
require_text "$CONTRACTS_MOD" "pub mod hardware_runtime_executor_live_handoff_cleanup_smoke;"

require_text "$PHYS" "HARDWARE_RUNTIME_EXECUTOR_LIVE_HANDOFF_CLEANUP_MARKER"
require_text "$PHYS" "hardware_runtime_executor_live_handoff_cleanup=ok"
require_text "$PHYS" "VaachakHardwareRuntimeExecutorLiveHandoff::live_handoff_ok()"
require_text "$PHYS" "VaachakHardwareRuntimeExecutorRuntimeUse::runtime_use_ok()"
require_regex "$PHYS" "VaachakHardwareRuntimeExecutorAcceptance::[[:space:]]*acceptance_ok\\(\\)"
require_text "$PHYS" "VaachakHardwareExecutorPulpBackend::backend_ok()"
require_text "$PHYS" "READER_FILE_BROWSER_UX_CHANGED: bool = false"
require_text "$PHYS" "APP_NAVIGATION_BEHAVIOR_CHANGED: bool = false"
require_text "$PHYS" "DESTRUCTIVE_STORAGE_BEHAVIOR_ADDED: bool = false"
require_text "$SMOKE" "live_handoff_cleanup_ok()"
require_text "$DOC" "hardware_runtime_executor_live_handoff_cleanup=ok"
require_text "$LIVE_DOC" "hardware-runtime-executor-live-handoff-cleanup.md"
require_regex "$LIVE_DOC" "PulpCompatibility|Pulp-compatible|pulp_compatible"

require_text "$BOOT" "emit_hardware_runtime_executor_live_handoff_marker"
require_text "$PULP_RUNTIME" "VaachakHardwareRuntimeExecutorLiveHandoff::active_boot_preflight()"
require_text "$PULP_RUNTIME" "adopt_storage_availability_handoff()"
require_text "$PULP_RUNTIME" "adopt_display_refresh_handoff()"
require_text "$PULP_RUNTIME" "adopt_input_runtime_handoff()"
require_text "$PULP_RUNTIME" "adopt_imported_pulp_reader_runtime_boundary()"

require_text "$LIVE" "hardware_runtime_executor_live_path_handoff=ok"
require_regex "$RUNTIME_USE" "VaachakHardwareRuntimeExecutorAcceptance::[[:space:]]*acceptance_ok\\(\\)"

require_regex_absent "$PHYS" "SdStorage::mount|speed_up_spi|InputDriver::new|epd\.init|kernel\.run|ensure_x4_dir_async"
require_regex_absent "$PHYS" "fn[[:space:]]+(draw|refresh|mount|probe|read|write|append|delete|rename|mkdir)"

if grep -RInE "write|append|delete|rename|mkdir|format" \
  target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor_live_handoff_cleanup.rs \
  target-xteink-x4/src/vaachak_x4/contracts/hardware_runtime_executor_live_handoff_cleanup_smoke.rs \
  | grep -Ev "DESTRUCTIVE_STORAGE_BEHAVIOR_ADDED|destructive_storage_behavior_added|does not change|Behavior preservation|not change|unchanged" >/dev/null; then
  fail "cleanup introduced destructive storage vocabulary outside guarded documentation/flags"
fi

echo "hardware_runtime_executor_live_handoff_cleanup=ok"
