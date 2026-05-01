#!/usr/bin/env bash
set -euo pipefail

vendor_main="vendor/pulp-os/src/bin/main.rs"
target_main="target-xteink-x4/src/main.rs"

if [[ ! -f "$vendor_main" ]]; then
  echo "FAIL missing $vendor_main"
  exit 1
fi

if [[ ! -f "$target_main" ]]; then
  echo "FAIL missing $target_main"
  exit 1
fi

expected="/tmp/phase17_expected_main.rs"
actual="/tmp/phase17_actual_main.rs"
diff_file="/tmp/phase17_reader_runtime_sync.diff"

cp "$vendor_main" "$expected"
cp "$target_main" "$actual"

# Expected local alias:
# Cargo dependency key is `pulp-os`, Rust crate name is `pulp_os`.
perl -0pi -e 's/\bx4_os::/pulp_os::/g' "$expected"

normalize_file() {
  local f="$1"

  python3 - "$f" <<'PY'
from pathlib import Path
import sys

p = Path(sys.argv[1])
lines = p.read_text().replace("\r\n", "\n").replace("\r", "\n").splitlines()

allowed_substrings = [
    "phase16=x4-reader-parity-ok",
    "phase17=x4-reader-refactor-ok",
    "VaachakOS Phase 16",
    "VaachakOS Phase 17",
]

out = []
for line in lines:
    if any(s in line for s in allowed_substrings):
        continue
    out.append(line.rstrip())

p.write_text("\n".join(out) + "\n")
PY

  # Normalize rustfmt-driven import ordering too.
  # This avoids false positives where cargo fmt moved static_cell below pulp_os.
  if command -v rustfmt >/dev/null 2>&1; then
    rustfmt --edition 2024 "$f"
  fi
}

normalize_file "$expected"
normalize_file "$actual"

if diff -u "$expected" "$actual" > "$diff_file"; then
  echo "OK   reader runtime matches vendored Pulp main after allowed crate alias/phase markers/rustfmt"
  exit 0
fi

echo "FAIL reader runtime drift detected beyond allowed crate alias/phase markers"
echo "Inspect full diff: $diff_file"
echo
head -120 "$diff_file"
exit 1
