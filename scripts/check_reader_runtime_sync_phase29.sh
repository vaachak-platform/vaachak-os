#!/usr/bin/env bash
set -euo pipefail

vendor_main="vendor/pulp-os/src/bin/main.rs"
actual_runtime="target-xteink-x4/src/runtime/pulp_runtime.rs"
expected="/tmp/phase29_expected_runtime.rs"
actual="/tmp/phase29_actual_runtime.rs"
diff_file="/tmp/phase29_reader_runtime_sync.diff"

if [[ ! -f "$vendor_main" ]]; then echo "FAIL missing $vendor_main"; exit 1; fi
if [[ ! -f "$actual_runtime" ]]; then echo "FAIL missing $actual_runtime"; exit 1; fi

cp "$vendor_main" "$expected"
cp "$actual_runtime" "$actual"
perl -0pi -e 's/\bx4_os::/pulp_os::/g' "$expected"

normalize_file() {
  local f="$1"
  python3 - "$f" <<'PY'
from pathlib import Path
import sys, re
p = Path(sys.argv[1])
lines = p.read_text().replace('\r\n','\n').replace('\r','\n').splitlines()
out = []
for line in lines:
    if '#![no_std]' in line or '#![no_main]' in line:
        continue
    if 'VaachakRuntime::emit_boot_marker' in line:
        continue
    if re.search(r'esp_println::println!\(\s*"phase(16|17|18|19|20|21|22|23|24|25|26|27|28|29)=', line):
        continue
    if re.search(r'phase(16|17|18|19|20|21|22|23|24|25|26|27|28|29)=x4-', line):
        continue
    out.append(line.rstrip())
p.write_text('\n'.join(out) + '\n')
PY
  if command -v rustfmt >/dev/null 2>&1; then
    rustfmt --edition 2024 "$f"
  fi
}

normalize_file "$expected"
normalize_file "$actual"

if diff -u "$expected" "$actual" > "$diff_file"; then
  echo "OK   Phase 29 imported runtime matches vendored Pulp main after allowed Vaachak marker normalization"
  exit 0
fi

echo "FAIL Phase 29 imported runtime drift detected beyond allowed marker normalization"
echo "Inspect full diff: $diff_file"
head -120 "$diff_file"
exit 1
