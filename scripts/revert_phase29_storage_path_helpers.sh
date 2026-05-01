#!/usr/bin/env bash
set -euo pipefail

echo "Phase 29 revert helper"
echo "This removes the storage_path_helpers module and restores most recent Phase 29 backups if present."
rm -f target-xteink-x4/src/runtime/storage_path_helpers.rs
python3 - <<'PY'
from pathlib import Path
modrs = Path('target-xteink-x4/src/runtime/mod.rs')
if modrs.exists():
    lines = [l for l in modrs.read_text().splitlines() if l.strip() != 'pub mod storage_path_helpers;']
    modrs.write_text('\n'.join(lines) + '\n')
PY
latest_vaachak=$(ls -t .phase_backups/phase29/vaachak_runtime.rs.* 2>/dev/null | head -1 || true)
latest_pulp=$(ls -t .phase_backups/phase29/pulp_runtime.rs.* 2>/dev/null | head -1 || true)
latest_mod=$(ls -t .phase_backups/phase29/mod.rs.* 2>/dev/null | head -1 || true)
[[ -n "$latest_vaachak" ]] && cp "$latest_vaachak" target-xteink-x4/src/runtime/vaachak_runtime.rs
[[ -n "$latest_pulp" ]] && cp "$latest_pulp" target-xteink-x4/src/runtime/pulp_runtime.rs
[[ -n "$latest_mod" ]] && cp "$latest_mod" target-xteink-x4/src/runtime/mod.rs
