#!/usr/bin/env bash
set -euo pipefail

vendor_main="vendor/pulp-os/src/bin/main.rs"
actual_runtime="target-xteink-x4/src/runtime/pulp_runtime.rs"
expected="/tmp/phase25_expected_runtime.rs"
actual="/tmp/phase25_actual_runtime.rs"
diff_file="/tmp/phase25_reader_runtime_sync.diff"

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
    'phase16=x4-reader-parity-ok',
    'phase17=x4-reader-refactor-ok',
    'phase18=x4-runtime-adapter-ok',
    'phase19=x4-vaachak-runtime-facade-ok',
    'phase20=x4-boundary-scaffold-ok',
    'phase21=x4-storage-boundary-ok',
    'phase22=x4-input-boundary-ok',
    'phase23=x4-display-boundary-ok',
    'phase24=x4-boundary-contract-ok',
    'phase25=x4-storage-contract-smoke-ok',
    'VaachakOS Phase 16',
    'VaachakOS Phase 17',
    'VaachakOS Phase 18',
    'VaachakOS Phase 19',
    'VaachakOS Phase 20',
    'VaachakOS Phase 21',
    'VaachakOS Phase 22',
    'VaachakOS Phase 23',
    'VaachakOS Phase 24',
    'VaachakOS Phase 25',
    'VaachakRuntime::emit_boot_marker',
    'crate::runtime::vaachak_runtime::VaachakRuntime::emit_boot_marker',
]
out = []
for line in lines:
    if any(token in line for token in drop_contains):
        continue
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
  echo "OK   Phase 25 imported runtime matches vendored Pulp main after allowed Vaachak marker normalization"
  exit 0
fi

echo "FAIL Phase 25 imported runtime drift detected beyond allowed Vaachak marker normalization"
echo "Inspect full diff: $diff_file"
echo
head -120 "$diff_file"
exit 1
