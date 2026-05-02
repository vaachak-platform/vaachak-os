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

# 1) Remove stale import block robustly, even if formatting changed.
lines = src.splitlines()
new_lines = []
skip = False
removed_import = False

for line in lines:
    if "use super::state_io_shadow_write_acceptance::" in line:
        skip = True
        removed_import = True
        continue
    if skip:
        if line.strip() == "};":
            skip = False
        continue
    new_lines.append(line)

src = "\n".join(new_lines).rstrip() + "\n"

# 2) Remove any previous partial Phase 39M compatibility shim.
start_markers = [
    "/// Phase 39M compatibility shim.",
    "#[derive(Clone, Copy, Debug, Eq, PartialEq)]\npub struct Phase39mArchivedShadowWriteAcceptanceReport",
]
marker = "pub const PHASE_36P_STATE_IO_BACKEND_HANDOFF_CHECKLIST_MARKER"

for start_marker in start_markers:
    while start_marker in src:
        start = src.find(start_marker)
        end = src.find(marker, start)
        if end == -1:
            raise SystemExit("found compatibility shim but could not find Phase 36P marker after it")
        src = src[:start] + src[end:]

# 3) Insert full local compatibility report.
compat = '''/// Phase 39M full compatibility shim.
///
/// The earlier shadow-write acceptance module was archived by Phase 39M. This
/// historical backend checklist still records that older acceptance shape, so it
/// now carries a local side-effect-free compatibility report instead of importing
/// the archived module.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39mArchivedShadowWriteAcceptanceReport {
    pub marker: &'static str,
    pub backend_bound: bool,
    pub storage_behavior_moved: bool,
    pub display_behavior_moved: bool,
    pub input_behavior_moved: bool,
    pub power_behavior_moved: bool,
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
        backend_bound: true,
        storage_behavior_moved: false,
        display_behavior_moved: false,
        input_behavior_moved: false,
        power_behavior_moved: false,
        accepted: true,
    };

'''

idx = src.find(marker)
if idx == -1:
    raise SystemExit("could not find Phase 36P marker insertion point")
src = src[:idx] + compat + src[idx:]

path.write_text(src)
print(f"patched full compatibility shim in {path}; removed_import={removed_import}")

text = path.read_text()
if "use super::state_io_shadow_write_acceptance" in text:
    raise SystemExit("stale archived-module import still present")
for field in [
    "backend_bound",
    "storage_behavior_moved",
    "display_behavior_moved",
    "input_behavior_moved",
    "power_behavior_moved",
    "accepted",
]:
    if field not in text:
        raise SystemExit(f"compatibility field missing: {field}")
PY

"$ROOT/phase39m_backend_handoff_checklist_full_compat_repair_overlay/scripts/check_phase39m_backend_handoff_checklist_full_compat_repair.sh"

echo "phase39m-full-compat-repair=x4-backend-handoff-checklist-full-compat-repair-ok"
echo "Next recommended checks:"
echo "  cargo fmt --all"
echo "  cargo test -p vaachak-core --all-targets"
echo "  cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo "  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
