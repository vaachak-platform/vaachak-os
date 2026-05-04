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
import re
import sys

path = Path(sys.argv[1])
src = path.read_text()

old_src = src

# Remove the archived module import.
src = re.sub(
    r"use\s+super::state_io_shadow_write_acceptance::\{\s*"
    r"ShadowWriteAcceptanceReport,\s*"
    r"STATE_IO_SHADOW_WRITE_ACCEPTANCE_REPORT,\s*"
    r"\};\s*\n",
    "",
    src,
    count=1,
    flags=re.MULTILINE,
)

compat = '''/// Phase 39M compatibility shim.
///
/// `state_io_shadow_write_acceptance.rs` was archived by Phase 39M. This
/// historical backend checklist still records the older Phase 36O marker, so it
/// now carries a local side-effect-free compatibility report instead of importing
/// the archived module.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39mArchivedShadowWriteAcceptanceReport {
    pub marker: &'static str,
    pub accepted: bool,
}

impl Phase39mArchivedShadowWriteAcceptanceReport {
    pub const fn is_accepted(self) -> bool {
        self.accepted
    }
}

pub type ShadowWriteAcceptanceReport = Phase39mArchivedShadowWriteAcceptanceReport;

pub const STATE_IO_SHADOW_WRITE_ACCEPTANCE_REPORT: ShadowWriteAcceptanceReport =
    ShadowWriteAcceptanceReport {
        marker: "phase36o=x4-state-io-shadow-write-acceptance-ok",
        accepted: true,
    };

'''

if "Phase39mArchivedShadowWriteAcceptanceReport" not in src:
    marker = "pub const PHASE_36P_STATE_IO_BACKEND_HANDOFF_CHECKLIST_MARKER"
    idx = src.find(marker)
    if idx == -1:
        raise SystemExit("could not find Phase 36P marker insertion point")
    src = src[:idx] + compat + src[idx:]

if src != old_src:
    path.write_text(src)
    print(f"patched archive dependency in {path}")
else:
    print(f"Phase 39M archive dependency repair already present in {path}")

text = path.read_text()
if "state_io_shadow_write_acceptance" in text:
    raise SystemExit("archived module import/name still present in checklist")
if "Phase39mArchivedShadowWriteAcceptanceReport" not in text:
    raise SystemExit("local compatibility report missing")
if "STATE_IO_SHADOW_WRITE_ACCEPTANCE_REPORT" not in text:
    raise SystemExit("local shadow acceptance report constant missing")
PY

"$ROOT/phase39m_backend_handoff_checklist_archive_dependency_repair_overlay/scripts/check_phase39m_backend_handoff_checklist_archive_dependency_repair.sh"

echo "phase39m-repair=x4-backend-handoff-checklist-archive-dependency-repair-ok"
echo "Next recommended checks:"
echo "  cargo fmt --all"
echo "  cargo test -p vaachak-core --all-targets"
echo "  cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo "  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
