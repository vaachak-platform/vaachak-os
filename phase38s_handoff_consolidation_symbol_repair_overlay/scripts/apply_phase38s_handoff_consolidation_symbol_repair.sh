#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
PHASE38S="$ROOT/target-xteink-x4/src/vaachak_x4/runtime/state_io_write_lane_handoff_consolidation.rs"
DIR_CACHE="$ROOT/vendor/pulp-os/kernel/src/kernel/dir_cache.rs"

if [ ! -f "$PHASE38S" ]; then
  echo "missing Phase 38S file: $PHASE38S" >&2
  exit 1
fi

python3 - "$PHASE38S" "$DIR_CACHE" <<'PY'
from pathlib import Path
import sys

phase38s = Path(sys.argv[1])
dir_cache = Path(sys.argv[2])

src = phase38s.read_text()
src2 = src.replace("phase38c_live_writes_enabled", "phase38c_writes_enabled")
if src2 != src:
    phase38s.write_text(src2)
    print(f"patched symbol name in {phase38s}")
else:
    print(f"phase38s symbol already patched in {phase38s}")

if dir_cache.exists():
    dsrc = dir_cache.read_text()
    needle = "fn phase38i_is_epub_or_epu_name(name: &[u8]) -> bool {"
    if needle in dsrc and "#[allow(dead_code)]\nfn phase38i_is_epub_or_epu_name" not in dsrc:
        dsrc = dsrc.replace(needle, "#[allow(dead_code)]\n" + needle, 1)
        dir_cache.write_text(dsrc)
        print(f"added dead_code allow to {dir_cache}")
    else:
        print(f"dir_cache helper allow already present or helper absent in {dir_cache}")
PY

"$ROOT/phase38s_handoff_consolidation_symbol_repair_overlay/scripts/check_phase38s_handoff_consolidation_symbol_repair.sh"

echo "phase38s-repair=x4-write-lane-handoff-symbol-repair-ok"
echo "Next recommended checks:"
echo "  cargo fmt --all"
echo "  cargo test -p vaachak-core --all-targets"
echo "  cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo "  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
