#!/usr/bin/env bash
set -euo pipefail

failures=0

ok() { printf 'OK   %s\n' "$*"; }
fail() { printf 'FAIL %s\n' "$*"; failures=$((failures + 1)); }

exists() {
  if [[ -e "$1" ]]; then ok "exists: $1"; else fail "missing: $1"; fi
}

contains() {
  local desc="$1"
  local pattern="$2"
  shift 2
  if rg -n -e "$pattern" "$@" >/tmp/phase35d2_guard_rg.txt 2>/dev/null; then
    ok "$desc"
  else
    fail "$desc"
    printf '      pattern: %s\n' "$pattern"
    printf '      path:    %s\n' "$*"
  fi
}

not_contains() {
  local desc="$1"
  local pattern="$2"
  shift 2
  if rg -n -e "$pattern" "$@" >/tmp/phase35d2_guard_rg.txt 2>/dev/null; then
    fail "$desc"
    cat /tmp/phase35d2_guard_rg.txt
  else
    ok "$desc"
  fi
}

echo "Phase 35D-2 boot preflight allocation guard"
echo

runtime="target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs"
storage_bridge="target-xteink-x4/src/vaachak_x4/io/storage_state_runtime.rs"
reader_bridge="target-xteink-x4/src/vaachak_x4/io/reader_state_runtime.rs"

exists "$runtime"
exists "$storage_bridge"
exists "$reader_bridge"
exists docs/phase35d2/PHASE35D2_BOOT_PREFLIGHT_ALLOCATION_GUARD.md
exists docs/phase35d2/PHASE35D2_ACCEPTANCE.md
exists docs/phase35d2/PHASE35D2_NOTES.md

contains "storage bridge declares pre-heap preflight allocation policy" \
  'PRE_HEAP_RUNTIME_PREFLIGHT_ALLOCATES: bool = false|ALLOC_RUNTIME_PREFLIGHT_REQUIRES_HEAP: bool = true' \
  "$storage_bridge"

contains "reader-state bridge declares heap requirement" \
  'REQUIRES_HEAP_ALLOCATOR_IN_PHASE35D2: bool = true' \
  "$reader_bridge"

python3 - "$runtime" "$storage_bridge" <<'PY' >/tmp/phase35d2_order.txt
from pathlib import Path
import re
import sys

runtime = Path(sys.argv[1]).read_text().splitlines()
storage = Path(sys.argv[2]).read_text()
failures = []

def first_line(pattern: str):
    rx = re.compile(pattern)
    for idx, line in enumerate(runtime, 1):
        if rx.search(line):
            return idx
    return None

early = first_line(r'active_runtime_preflight\(')
heap = first_line(r'heap_allocator!\(#\[ram\(reclaimed\)\] size: 64_000\)')
alloc = first_line(r'active_runtime_alloc_preflight\(')

if early is None:
    failures.append("missing active_runtime_preflight call")
if heap is None:
    failures.append("missing reclaimed heap allocator setup")
if alloc is None:
    failures.append("missing active_runtime_alloc_preflight call")
if early is not None and heap is not None and not early < heap:
    failures.append(f"active_runtime_preflight line {early} must be before heap line {heap}")
if heap is not None and alloc is not None and not heap < alloc:
    failures.append(f"active_runtime_alloc_preflight line {alloc} must be after heap line {heap}")

preheap_body = re.search(
    r'pub fn active_runtime_preflight\(\) -> bool \{(?P<body>.*?)\n    \}',
    storage,
    re.S,
)
if not preheap_body:
    failures.append("missing storage active_runtime_preflight function body")
else:
    body = preheap_body.group("body")
    blocked = [
        "VaachakReaderStateRuntimeBridge",
        "active_runtime_alloc_preflight",
        "String",
        "Vec",
        "alloc::",
        "to_string",
        "Box",
        "format!",
    ]
    for token in blocked:
        if token in body:
            failures.append(f"pre-heap preflight body contains allocation/cross-bridge token: {token}")

if failures:
    for failure in failures:
        print(f"FAIL {failure}")
    sys.exit(1)

print("OK   pre-heap and post-heap preflight order is valid")
print("OK   pre-heap preflight body is allocation-free")
PY

if cat /tmp/phase35d2_order.txt; then
  ok "boot preflight ordering check ran"
else
  fail "boot preflight ordering check failed"
fi

not_contains "storage preflight report does not include reader-state facade status" \
  'reader_state_facade_ok' \
  "$storage_bridge"

not_contains "active source has no fake/raw EPUB smoke path" \
  'run_epub_reader_page_storage_smoke|ZIP container parsed|First readable bytes|ensure_pulp_dir_async' \
  target-xteink-x4/src

echo
echo "Phase 35D-2 boot preflight allocation guard complete: failures=${failures}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
