#!/usr/bin/env bash
set -euo pipefail

if [[ -f .phase_backups/phase22/input_boundary.rs.bak ]]; then
  cp .phase_backups/phase22/input_boundary.rs.bak target-xteink-x4/src/runtime/input_boundary.rs
  echo "Restored input_boundary.rs from Phase 22 backup"
else
  echo "No Phase 22 input_boundary.rs backup found" >&2
fi

python3 - <<'PY'
from pathlib import Path
p = Path("target-xteink-x4/src/runtime/vaachak_runtime.rs")
if p.exists():
    s = p.read_text()
    s = s.replace("        crate::runtime::input_boundary::VaachakInputBoundary::emit_boot_marker();\n", "")
    p.write_text(s)
PY
