#!/usr/bin/env bash
# scripts/check_imported_reader_runtime_sync_phase31.sh

set -euo pipefail

vendor_main="vendor/pulp-os/src/bin/main.rs"
actual_runtime="target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs"

tmp_dir="$(mktemp -d /tmp/phase31_imported_runtime_sync.XXXXXX)"
expected="$tmp_dir/expected_imported_runtime.rs"
actual="$tmp_dir/actual_imported_runtime.rs"
diff_file="$tmp_dir/imported_reader_runtime_sync.diff"

cleanup() {
  if [[ "${PHASE31_KEEP_DIFF:-0}" != "1" ]]; then
    rm -rf "$tmp_dir"
  fi
}
trap cleanup EXIT

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
    'phase26=x4-input-contract-smoke-ok',
    'phase27=x4-display-contract-smoke-ok',
    'phase28=x4-boundary-contract-smoke-ok',
    'phase29=x4-storage-path-helpers-ok',
    'phase31=',
    'vaachak=x4-runtime-ready',
    'VaachakBoot',
    'VaachakRuntime',
    'emit_runtime_ready_marker',
    'storage_path_helpers',
    'StoragePathHelpers',
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
  echo "OK   Phase 31 imported reader runtime matches vendored Pulp main after allowed Vaachak normalization"
  exit 0
fi

echo "FAIL Phase 31 imported reader runtime drift detected beyond allowed Vaachak normalization"
echo "Inspect full diff: $diff_file"
echo "Set PHASE31_KEEP_DIFF=1 to preserve the temp diff after exit."
echo
head -120 "$diff_file"
exit 1
