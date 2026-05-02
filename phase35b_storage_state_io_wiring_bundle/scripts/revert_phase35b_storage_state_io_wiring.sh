#!/usr/bin/env bash
set -euo pipefail

echo "Reverting Phase 35B storage state runtime wiring scaffold"

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

echo "Phase 35B files removed. Run cargo fmt/check before continuing."
