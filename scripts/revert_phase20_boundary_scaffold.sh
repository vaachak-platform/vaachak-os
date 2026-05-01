#!/usr/bin/env bash
set -euo pipefail

if [[ ! -d target-xteink-x4/src/runtime ]]; then
  echo "ERROR: run from vaachak-os repo root" >&2
  exit 1
fi

python3 - <<'PY'
from pathlib import Path

facade = Path("target-xteink-x4/src/runtime/vaachak_runtime.rs")
if facade.exists():
    s = facade.read_text()
    call = "        crate::runtime::display_boundary::VaachakDisplayBoundary::emit_scaffold_marker();\n"
    s = s.replace(call, "")
    facade.write_text(s)

modrs = Path("target-xteink-x4/src/runtime/mod.rs")
if modrs.exists():
    keep = []
    for line in modrs.read_text().splitlines():
        if line.strip() in {
            "pub mod display_boundary;",
            "pub mod input_boundary;",
            "pub mod storage_boundary;",
        }:
            continue
        keep.append(line)
    modrs.write_text("\n".join(keep).rstrip() + "\n")
PY

rm -f target-xteink-x4/src/runtime/display_boundary.rs \
      target-xteink-x4/src/runtime/input_boundary.rs \
      target-xteink-x4/src/runtime/storage_boundary.rs

echo "Reverted Phase 20 boundary scaffold files and marker call"
