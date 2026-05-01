#!/usr/bin/env bash
set -euo pipefail

vendor_main="vendor/pulp-os/src/bin/main.rs"
runtime_main="target-xteink-x4/src/runtime/pulp_runtime.rs"
root_main="target-xteink-x4/src/main.rs"

if [[ ! -f "$vendor_main" ]]; then
  echo "FAIL missing $vendor_main"
  exit 1
fi

if [[ ! -f "$runtime_main" ]]; then
  echo "FAIL missing $runtime_main"
  exit 1
fi

if [[ ! -f "$root_main" ]]; then
  echo "FAIL missing $root_main"
  exit 1
fi

expected="/tmp/phase18_expected_runtime.rs"
actual="/tmp/phase18_actual_runtime.rs"
diff_file="/tmp/phase18_reader_runtime_sync.diff"

cp "$vendor_main" "$expected"
cp "$runtime_main" "$actual"

# Expected local alias:
# Cargo dependency key is `pulp-os`, Rust crate name is `pulp_os`.
perl -0pi -e 's/\bx4_os::/pulp_os::/g' "$expected"

# Pulp's bin/main.rs has crate-root attrs. Phase 18 moves runtime into a module,
# so these attrs correctly live in target-xteink-x4/src/main.rs instead.
perl -0pi -e 's/^#!\[no_std\]\n//mg; s/^#!\[no_main\]\n//mg' "$expected"

normalize_file() {
  local f="$1"

  python3 - "$f" <<'PY'
from pathlib import Path
import sys

p = Path(sys.argv[1])
text = p.read_text().replace("\r\n", "\n").replace("\r", "\n")
lines = text.splitlines()

allowed_substrings = [
    "phase16=x4-reader-parity-ok",
    "phase17=x4-reader-refactor-ok",
    "phase18=x4-runtime-adapter-ok",
    "VaachakOS Phase 16",
    "VaachakOS Phase 17",
    "VaachakOS Phase 18",
    "#![no_std]",
    "#![no_main]",
]

out = []
in_leading_attrs = True
for line in lines:
    stripped = line.strip()

    # Runtime module cannot contain crate-level inner attrs; remove them from
    # expected vendored main before comparing.
    if in_leading_attrs and (stripped == "" or stripped.startswith("#![")):
        continue
    in_leading_attrs = False

    if any(s in line for s in allowed_substrings):
        continue
    out.append(line.rstrip())

p.write_text("\n".join(out).strip() + "\n")
PY

  # Normalize rustfmt-driven import ordering and whitespace.
  if command -v rustfmt >/dev/null 2>&1; then
    rustfmt --edition 2024 "$f"
  fi
}

normalize_file "$expected"
normalize_file "$actual"

if diff -u "$expected" "$actual" > "$diff_file"; then
  echo "OK   Phase 18 runtime matches vendored Pulp main after crate alias/phase marker/runtime-module normalization"
  exit 0
fi

echo "FAIL Phase 18 reader runtime drift detected beyond allowed crate alias/phase markers"
echo "Inspect full diff: $diff_file"
echo
head -160 "$diff_file"
exit 1
