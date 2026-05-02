#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
FILE="$ROOT/target-xteink-x4/src/vaachak_x4/runtime/state_io_backend_handoff_checklist.rs"

if [ ! -f "$FILE" ]; then
  echo "missing backend handoff checklist: $FILE" >&2
  exit 1
fi

python3 - "$FILE" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
src = path.read_text()

if "pub const fn phase36o_marker()" not in src:
    anchor = "pub const STATE_IO_SHADOW_WRITE_ACCEPTANCE_REPORT: ShadowWriteAcceptanceReport ="
    idx = src.find(anchor)
    if idx == -1:
        raise SystemExit("could not find STATE_IO_SHADOW_WRITE_ACCEPTANCE_REPORT anchor")

    # Insert after the constant block ending with the first `};` after the anchor.
    end = src.find("};", idx)
    if end == -1:
        raise SystemExit("could not find end of STATE_IO_SHADOW_WRITE_ACCEPTANCE_REPORT constant")
    end += len("};")

    compat = '''

pub const fn phase36o_marker() -> &'static str {
    "phase36o=x4-state-io-shadow-write-acceptance-ok"
}

pub const fn phase36o_acceptance_report() -> ShadowWriteAcceptanceReport {
    STATE_IO_SHADOW_WRITE_ACCEPTANCE_REPORT
}
'''
    src = src[:end] + compat + src[end:]
    path.write_text(src)
    print(f"added local phase36o compatibility functions to {path}")
else:
    print(f"phase36o compatibility functions already present in {path}")

text = path.read_text()
if "use super::state_io_shadow_write_acceptance" in text:
    raise SystemExit("stale archived-module import still present")
if "pub const fn phase36o_marker()" not in text:
    raise SystemExit("phase36o_marker compatibility function missing")
if "pub const fn phase36o_acceptance_report()" not in text:
    raise SystemExit("phase36o_acceptance_report compatibility function missing")
if "STATE_IO_SHADOW_WRITE_ACCEPTANCE_REPORT" not in text:
    raise SystemExit("local shadow acceptance report constant missing")
PY

"$ROOT/phase39m_backend_handoff_checklist_phase36o_function_repair_overlay/scripts/check_phase39m_backend_handoff_checklist_phase36o_function_repair.sh"

echo "phase39m-phase36o-function-repair=x4-backend-handoff-checklist-phase36o-function-repair-ok"
echo "Next recommended checks:"
echo "  cargo fmt --all"
echo "  cargo test -p vaachak-core --all-targets"
echo "  cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo "  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
