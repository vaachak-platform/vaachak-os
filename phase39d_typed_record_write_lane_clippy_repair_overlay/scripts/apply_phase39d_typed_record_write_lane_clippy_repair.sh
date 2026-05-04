#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
FILE="$ROOT/target-xteink-x4/src/vaachak_x4/runtime/state_io_typed_record_write_lane.rs"

if [ ! -f "$FILE" ]; then
  echo "missing Phase 39D file: $FILE" >&2
  exit 1
fi

python3 - "$FILE" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
src = path.read_text()

old = """    if let Some(book_id) = request.book_id {
        if !book_id.is_hex8() {
            return Phase39dTypedWriteDecision::RejectedInvalidBookId;
        }
    }
"""

new = """    if let Some(book_id) = request.book_id
        && !book_id.is_hex8()
    {
        return Phase39dTypedWriteDecision::RejectedInvalidBookId;
    }
"""

if old in src:
    src = src.replace(old, new, 1)
    path.write_text(src)
    print(f"patched collapsible if in {path}")
elif "if let Some(book_id) = request.book_id\n        && !book_id.is_hex8()" in src:
    print(f"Phase 39D collapsible-if repair already present in {path}")
else:
    raise SystemExit("expected nested book_id validation block not found")
PY

"$ROOT/phase39d_typed_record_write_lane_clippy_repair_overlay/scripts/check_phase39d_typed_record_write_lane_clippy_repair.sh"

echo "phase39d-clippy-repair=x4-typed-record-write-lane-clippy-repair-ok"
echo "Next recommended checks:"
echo "  cargo fmt --all"
echo "  cargo test -p vaachak-core --all-targets"
echo "  cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo "  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
