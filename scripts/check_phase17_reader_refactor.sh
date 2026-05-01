#!/usr/bin/env bash
set -euo pipefail

failures=0
warnings=0

ok() {
  printf 'OK   %s\n' "$*"
}

fail() {
  printf 'FAIL %s\n' "$*"
  failures=$((failures + 1))
}

warn() {
  printf 'WARN %s\n' "$*"
  warnings=$((warnings + 1))
}

exists() {
  if [[ -e "$1" ]]; then
    ok "exists: $1"
  else
    fail "missing: $1"
  fi
}

contains() {
  local desc="$1"
  local pattern="$2"
  shift 2

  if rg -n -e "$pattern" "$@" >/tmp/phase17_check_rg.txt 2>/dev/null; then
    ok "$desc"
  else
    fail "$desc"
    printf '      pattern: %s\n' "$pattern"
    printf '      path:    %s\n' "$*"
  fi
}

contains_warn() {
  local desc="$1"
  local pattern="$2"
  shift 2

  if rg -n -e "$pattern" "$@" >/tmp/phase17_check_rg.txt 2>/dev/null; then
    ok "$desc"
  else
    warn "$desc"
    printf '      pattern: %s\n' "$pattern"
    printf '      path:    %s\n' "$*"
  fi
}

not_contains() {
  local desc="$1"
  local pattern="$2"
  shift 2

  if rg -n -e "$pattern" "$@" >/tmp/phase17_check_rg.txt 2>/dev/null; then
    fail "$desc"
    cat /tmp/phase17_check_rg.txt
  else
    ok "$desc"
  fi
}

run_cmd() {
  local desc="$1"
  shift
  if "$@"; then
    ok "$desc"
  else
    fail "$desc"
  fi
}

echo "Phase 17 reader refactor check"
echo

exists Cargo.toml
exists target-xteink-x4/Cargo.toml
exists target-xteink-x4/src/main.rs
exists vendor/pulp-os/src/bin/main.rs
exists vendor/pulp-os/Cargo.toml
exists vendor/pulp-os/kernel/Cargo.toml
exists vendor/smol-epub/Cargo.toml
exists docs/phase17/READER_RUNTIME_BOUNDARY.md
exists docs/phase17/PHASE17_REFACTOR_CHECKLIST.md
exists scripts/check_reader_runtime_sync.sh

run_cmd "cargo metadata succeeds" cargo metadata --format-version 1 --no-deps >/tmp/phase17_metadata.json

if [[ "${PHASE17_RUN_CARGO:-0}" == "1" ]]; then
  run_cmd "target-xteink-x4 cargo check succeeds" \
    cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
  run_cmd "target-xteink-x4 clippy succeeds" \
    cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings
else
  ok "cargo check/clippy skipped inside script; set PHASE17_RUN_CARGO=1 to enable"
fi

contains \
  "Phase 16 marker is still present" \
  'phase16=x4-reader-parity-ok' \
  target-xteink-x4/src/main.rs

contains \
  "Phase 17 marker is present" \
  'phase17=x4-reader-refactor-ok' \
  target-xteink-x4/src/main.rs

contains \
  "target depends on vendored x4-os via pulp-os alias" \
  'pulp-os\s*=\s*\{[^}]*package\s*=\s*"x4-os"[^}]*path\s*=\s*"\.\./vendor/pulp-os"' \
  target-xteink-x4/Cargo.toml

contains \
  "target has direct x4-kernel path dependency" \
  'x4-kernel\s*=\s*\{[^}]*path\s*=\s*"\.\./vendor/pulp-os/kernel"' \
  target-xteink-x4/Cargo.toml

contains \
  "target has direct smol-epub path dependency" \
  'smol-epub\s*=\s*\{[^}]*path\s*=\s*"\.\./vendor/smol-epub"' \
  target-xteink-x4/Cargo.toml

contains \
  "root workspace excludes vendored Pulp workspace" \
  'exclude\s*=.*vendor/pulp-os' \
  Cargo.toml

contains \
  "root workspace excludes vendored smol-epub workspace" \
  'exclude\s*=.*vendor/smol-epub' \
  Cargo.toml

not_contains \
  "active target main has no fake/raw EPUB smoke path" \
  'run_epub_reader_page_storage_smoke|ZIP container parsed|First readable bytes|ensure_pulp_dir_async' \
  target-xteink-x4/src/main.rs

contains \
  "smol-epub path is present for EPUB ZIP/OPF/HTML reader" \
  'smol_epub|smol-epub' \
  vendor/pulp-os vendor/smol-epub target-xteink-x4/Cargo.toml Cargo.lock

contains \
  "X4/Pulp bookmark path is present" \
  'bookmark|Bookmark|bookmarks|Bookmarks' \
  vendor/pulp-os/src vendor/pulp-os/kernel

contains \
  "progress/continue path symbols are present" \
  'progress|Progress|position|Position|offset|Offset|last_read|LastRead|continue|Continue' \
  vendor/pulp-os/src vendor/pulp-os/kernel

contains_warn \
  "theme/font/preset state symbols are present" \
  'theme|Theme|preset|Preset|font|Font' \
  vendor/pulp-os/src vendor/pulp-os/kernel

contains_warn \
  "reader footer/menu/action symbols are present" \
  'footer|Footer|quick|Quick|menu|Menu|action|Action' \
  vendor/pulp-os/src vendor/pulp-os/kernel

if ls target-xteink-x4/src/main.rs.phase15a-backup.* target-xteink-x4/src/main.rs.bak-phase* >/dev/null 2>&1; then
  warn "old main.rs backup files remain in active source directory"
else
  ok "active source directory has no old main.rs phase backup files"
fi

if ./scripts/check_reader_runtime_sync.sh; then
  ok "reader runtime sync check passes"
else
  fail "reader runtime sync check fails"
fi

echo
echo "Phase 17 reader refactor check complete: failures=${failures} warnings=${warnings}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
