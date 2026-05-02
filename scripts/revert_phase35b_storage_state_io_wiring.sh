#!/usr/bin/env bash
set -euo pipefail

echo "Reverting Phase 35B storage state runtime wiring scaffold"

backup_dir=".phase_backups/phase35b/revert-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$backup_dir"

backup_if_exists() {
  local path="$1"
  if [[ -e "$path" ]]; then
    mkdir -p "$backup_dir/$(dirname "$path")"
    cp -a "$path" "$backup_dir/$path"
    echo "Backed up $path to $backup_dir/$path"
  fi
}

backup_if_exists target-xteink-x4/src/vaachak_x4/io/storage_state_runtime.rs
backup_if_exists target-xteink-x4/src/vaachak_x4/io/mod.rs
backup_if_exists target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs
backup_if_exists docs/phase35b
backup_if_exists scripts/check_imported_reader_runtime_sync_phase35b.sh
backup_if_exists scripts/check_phase35b_storage_state_io_wiring.sh
backup_if_exists scripts/check_phase35b_no_vendor_or_hardware_regression.sh
backup_if_exists scripts/revert_phase35b_storage_state_io_wiring.sh

rm -f target-xteink-x4/src/vaachak_x4/io/storage_state_runtime.rs

if [[ -f target-xteink-x4/src/vaachak_x4/io/mod.rs ]]; then
  python3 - <<'PY'
from pathlib import Path
p = Path("target-xteink-x4/src/vaachak_x4/io/mod.rs")
s = p.read_text()
s = "\n".join(
    line for line in s.splitlines()
    if "storage_state_runtime" not in line
) + "\n"
p.write_text(s)
PY
fi

if [[ -f target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs ]]; then
  python3 - <<'PY'
from pathlib import Path
p = Path("target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs")
s = p.read_text()
lines = []
for line in s.splitlines():
    if "storage_state_runtime" in line or "VaachakStorageStateRuntimeBridge" in line or "active_runtime_preflight" in line:
        continue
    lines.append(line)
p.write_text("\n".join(lines) + "\n")
PY
fi

rm -rf docs/phase35b
rm -f scripts/check_imported_reader_runtime_sync_phase35b.sh
rm -f scripts/check_phase35b_storage_state_io_wiring.sh
rm -f scripts/check_phase35b_no_vendor_or_hardware_regression.sh
rm -f scripts/revert_phase35b_storage_state_io_wiring.sh

echo "Phase 35B files removed after backup. Run cargo fmt/check before continuing."
