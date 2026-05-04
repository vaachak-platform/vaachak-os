#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
FILE="$ROOT/target-xteink-x4/src/vaachak_x4/runtime/state_io_typed_record_sdfat_adapter.rs"

if [ ! -f "$FILE" ]; then
  echo "missing Phase 39E file: $FILE" >&2
  exit 1
fi

python3 - "$FILE" <<'PY'
from pathlib import Path
import sys
import re

path = Path(sys.argv[1])
src = path.read_text()

# Remove unused imports from the grouped import block.
unused = [
    "Phase39dBookId, ",
    "Phase39dTypedWritePreflight, ",
    "Phase39dTypedWriteReport, ",
]
new_src = src
for item in unused:
    new_src = new_src.replace(item, "")

# Handle variants if rustfmt split differently.
new_src = new_src.replace("    Phase39dBookId,\n", "")
new_src = new_src.replace("    Phase39dTypedWritePreflight,\n", "")
new_src = new_src.replace("    Phase39dTypedWriteReport,\n", "")

# Change the wrote_once method from const fn to fn.
new_src = new_src.replace(
    "    pub const fn wrote_once(self) -> bool {\n"
    "        self.calls == 1 && self.last_error == Phase39eBackendError::None\n"
    "    }\n",
    "    pub fn wrote_once(self) -> bool {\n"
    "        self.calls == 1 && self.last_error == Phase39eBackendError::None\n"
    "    }\n",
)

if new_src == src:
    print(f"Phase 39E compile repair may already be applied in {path}")
else:
    path.write_text(new_src)
    print(f"patched Phase 39E compile issues in {path}")

# Sanity checks.
text = path.read_text()
if "Phase39dBookId" in text:
    raise SystemExit("unused Phase39dBookId import still present")
if "Phase39dTypedWritePreflight" in text:
    raise SystemExit("unused Phase39dTypedWritePreflight import still present")
if "Phase39dTypedWriteReport" in text:
    raise SystemExit("unused Phase39dTypedWriteReport import still present")
if "pub const fn wrote_once(self) -> bool" in text:
    raise SystemExit("wrote_once is still const fn")
if "pub fn wrote_once(self) -> bool" not in text:
    raise SystemExit("wrote_once runtime fn not found")
PY

"$ROOT/phase39e_typed_record_sdfat_adapter_compile_repair_overlay/scripts/check_phase39e_typed_record_sdfat_adapter_compile_repair.sh"

echo "phase39e-compile-repair=x4-typed-record-sdfat-adapter-compile-repair-ok"
echo "Next recommended checks:"
echo "  cargo fmt --all"
echo "  cargo test -p vaachak-core --all-targets"
echo "  cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo "  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
