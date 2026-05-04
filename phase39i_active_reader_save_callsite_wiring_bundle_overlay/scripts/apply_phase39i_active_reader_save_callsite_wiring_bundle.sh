#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OVERLAY="$ROOT/phase39i_active_reader_save_callsite_wiring_bundle_overlay"

READER_MOD="$ROOT/vendor/pulp-os/src/apps/reader/mod.rs"
READER_HELPER_SRC="$OVERLAY/replaceable/vendor/pulp-os/src/apps/reader/typed_state_wiring.rs"
READER_HELPER_DST="$ROOT/vendor/pulp-os/src/apps/reader/typed_state_wiring.rs"

TARGET_SRC="$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/state_io_active_reader_save_callsite_wiring.rs"
TARGET_DST="$ROOT/target-xteink-x4/src/vaachak_x4/runtime/state_io_active_reader_save_callsite_wiring.rs"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"

if [ ! -f "$READER_MOD" ]; then
  echo "missing active reader module: $READER_MOD" >&2
  exit 1
fi

if [ ! -f "$READER_HELPER_SRC" ]; then
  echo "missing helper source: $READER_HELPER_SRC" >&2
  exit 1
fi

if [ ! -f "$TARGET_SRC" ]; then
  echo "missing target source: $TARGET_SRC" >&2
  exit 1
fi

if [ ! -f "$RUNTIME_MOD" ]; then
  echo "missing runtime module file: $RUNTIME_MOD" >&2
  exit 1
fi

mkdir -p "$(dirname "$READER_HELPER_DST")"
cp -v "$READER_HELPER_SRC" "$READER_HELPER_DST"

mkdir -p "$(dirname "$TARGET_DST")"
cp -v "$TARGET_SRC" "$TARGET_DST"

python3 - "$READER_MOD" "$RUNTIME_MOD" <<'PY'
from pathlib import Path
import re
import sys

reader = Path(sys.argv[1])
runtime_mod = Path(sys.argv[2])

src = reader.read_text()

if "mod typed_state_wiring;" not in src:
    # Insert near the KernelHandle import so it remains clearly reader-local.
    needle = "use crate::kernel::KernelHandle;\n"
    if needle in src:
        src = src.replace(needle, needle + "\nmod typed_state_wiring;\n", 1)
    else:
        # Fallback: insert before first impl ReaderApp.
        needle = "impl ReaderApp {"
        if needle not in src:
            raise SystemExit("could not find insertion point for typed_state_wiring module")
        src = src.replace(needle, "mod typed_state_wiring;\n\n" + needle, 1)

# Route active state directory creation through the Phase 39I facade.
src = re.sub(
    r"\bk\s*\.\s*ensure_app_subdir\s*\(\s*reader_state::STATE_DIR\s*\)",
    "typed_state_wiring::ensure_state_dir(k)",
    src,
)

# Route active reader subdir writes through the Phase 39I facade. This keeps the
# original arguments and return type while adding a single typed-state seam.
src = re.sub(
    r"\bk\s*\.\s*write_app_subdir\s*\(",
    "typed_state_wiring::write_app_subdir(k, ",
    src,
)

reader.write_text(src)

export = "pub mod state_io_active_reader_save_callsite_wiring;"
rt = runtime_mod.read_text()
if export not in rt:
    rt = rt.rstrip() + "\n\n" + export + "\n"
    runtime_mod.write_text(rt)
PY

"$OVERLAY/scripts/check_phase39i_active_reader_save_callsite_wiring_bundle.sh"

echo "phase39i=x4-active-reader-save-callsite-wiring-bundle-ok"
echo "Phase 39I overlay applied. Next recommended checks:"
echo "  cargo fmt --all"
echo "  cargo test -p vaachak-core --all-targets"
echo "  cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo "  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
