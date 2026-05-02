#!/usr/bin/env bash
set -euo pipefail

vendor_main="vendor/pulp-os/src/bin/main.rs"
actual_runtime="target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs"

expected="/tmp/phase35_expected_imported_runtime.rs"
actual="/tmp/phase35_actual_imported_runtime.rs"
diff_file="/tmp/phase35_imported_reader_runtime_sync.diff"

if [[ ! -f "$vendor_main" ]]; then
  echo "FAIL missing $vendor_main"
  exit 1
fi

if [[ ! -f "$actual_runtime" ]]; then
  echo "FAIL missing $actual_runtime"
  exit 1
fi

cp "$vendor_main" "$expected"
cp "$actual_runtime" "$actual"

perl -0pi -e 's/\bx4_os::/pulp_os::/g' "$expected"

normalize_file() {
  local f="$1"

  python3 - "$f" <<'PY'
from pathlib import Path
import sys

p = Path(sys.argv[1])
text = p.read_text().replace("\r\n", "\n").replace("\r", "\n")
lines = text.splitlines()

drop_contains = [
    '#![no_std]',
    '#![no_main]',
    'phase16=', 'phase17=', 'phase18=', 'phase19=', 'phase20=',
    'phase21=', 'phase22=', 'phase23=', 'phase24=', 'phase25=',
    'phase26=', 'phase27=', 'phase28=', 'phase29=', 'phase30=',
    'phase31=', 'phase32=', 'phase33=', 'phase34=', 'phase35=',
    'vaachak=x4-runtime-ready',
    'VaachakBoot',
    'VaachakRuntime',
    'emit_runtime_ready_marker',
    'storage_path_helpers', 'VaachakStoragePathHelpers',
    'input_semantics', 'VaachakInputSemantics',
    'display_geometry', 'VaachakDisplayGeometry',
    'storage_state', 'VaachakStorageStateIo', 'VaachakStateIoKind',
]

out = []
for line in lines:
    if any(token in line for token in drop_contains):
        continue
    line = line.replace('crate::vaachak_x4::', 'crate::')
    line = line.replace('super::super::', 'super::')
    out.append(line.rstrip())

p.write_text("\n".join(out) + "\n")
PY

  if command -v rustfmt >/dev/null 2>&1; then
    rustfmt --edition 2024 "$f"
  fi
}

normalize_file "$expected"
normalize_file "$actual"

if diff -u "$expected" "$actual" > "$diff_file"; then
  echo "OK   Phase 35 imported reader runtime matches vendored Pulp main after allowed Vaachak normalization"
  exit 0
fi

echo "FAIL Phase 35 imported reader runtime drift detected beyond allowed Vaachak normalization"
echo "Inspect full diff: $diff_file"
echo
head -120 "$diff_file"
exit 1
