#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
PHASE38S="$ROOT/target-xteink-x4/src/vaachak_x4/runtime/state_io_write_lane_handoff_consolidation.rs"

if [ ! -f "$PHASE38S" ]; then
  echo "missing Phase 38S file: $PHASE38S" >&2
  exit 1
fi

python3 - "$PHASE38S" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
src = path.read_text()

replacements = {
    "ProgressOnly": "Progress",
    "ThemeOnly": "Theme",
    "MetadataOnly": "Metadata",
    "BookmarkOnly": "Bookmark",
    "BookmarkIndexOnly": "BookmarkIndex",
}

new_src = src
for old, new in replacements.items():
    new_src = new_src.replace(old, new)

if new_src != src:
    path.write_text(new_src)
    print(f"patched Phase 38S enum variants in {path}")
else:
    print(f"Phase 38S enum variants already patched in {path}")
PY

"$ROOT/phase38s_handoff_consolidation_clippy_repair_overlay/scripts/check_phase38s_handoff_consolidation_clippy_repair.sh"

echo "phase38s-clippy-repair=x4-write-lane-handoff-clippy-repair-ok"
echo "Next recommended checks:"
echo "  cargo fmt --all"
echo "  cargo test -p vaachak-core --all-targets"
echo "  cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo "  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
