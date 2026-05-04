#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
FILE="$ROOT/target-xteink-x4/src/vaachak_x4/runtime/state_io_runtime_owned_sdfat_writer.rs"

if [ ! -f "$FILE" ]; then
  echo "missing Phase 39F file: $FILE" >&2
  exit 1
fi

python3 - "$FILE" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
src = path.read_text()

old_direct = """        self.record_result(
            self.ops
                .write_record_direct(command.target_path.as_slice(), command.payload),
        )
"""
new_direct = """        let result = self
            .ops
            .write_record_direct(command.target_path.as_slice(), command.payload);
        self.record_result(result)
"""

old_atomic = """        self.record_result(self.ops.write_record_atomic(
            command.target_path.as_slice(),
            command.temp_path.as_slice(),
            command.payload,
        ))
"""
new_atomic = """        let result = self.ops.write_record_atomic(
            command.target_path.as_slice(),
            command.temp_path.as_slice(),
            command.payload,
        );
        self.record_result(result)
"""

changed = False

if old_direct in src:
    src = src.replace(old_direct, new_direct, 1)
    changed = True

if old_atomic in src:
    src = src.replace(old_atomic, new_atomic, 1)
    changed = True

if not changed:
    if "let result = self\n            .ops\n            .write_record_direct" in src and "let result = self.ops.write_record_atomic" in src:
        print(f"Phase 39F borrow repair already present in {path}")
    else:
        raise SystemExit("expected Phase 39F write_direct/write_atomic borrow patterns not found")
else:
    path.write_text(src)
    print(f"patched Phase 39F borrow issue in {path}")

text = path.read_text()
if "self.record_result(\n            self.ops" in text:
    raise SystemExit("old direct overlapping borrow pattern still present")
if "self.record_result(self.ops.write_record_atomic" in text:
    raise SystemExit("old atomic overlapping borrow pattern still present")
if "let result = self" not in text:
    raise SystemExit("local result binding not found after repair")
PY

"$ROOT/phase39f_runtime_owned_sdfat_writer_borrow_repair_overlay/scripts/check_phase39f_runtime_owned_sdfat_writer_borrow_repair.sh"

echo "phase39f-borrow-repair=x4-runtime-owned-sdfat-writer-borrow-repair-ok"
echo "Next recommended checks:"
echo "  cargo fmt --all"
echo "  cargo test -p vaachak-core --all-targets"
echo "  cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo "  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
