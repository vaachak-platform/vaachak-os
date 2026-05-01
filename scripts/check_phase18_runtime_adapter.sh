#!/usr/bin/env bash
set -euo pipefail

failures=0
warnings=0

ok() { printf 'OK   %s\n' "$*"; }
fail() { printf 'FAIL %s\n' "$*"; failures=$((failures + 1)); }
warn() { printf 'WARN %s\n' "$*"; warnings=$((warnings + 1)); }

exists() {
  if [[ -e "$1" ]]; then ok "exists: $1"; else fail "missing: $1"; fi
}

contains() {
  local desc="$1" pattern="$2"; shift 2
  if rg -n -e "$pattern" "$@" >/tmp/phase18_check_rg.txt 2>/dev/null; then
    ok "$desc"
  else
    fail "$desc"
    printf '      pattern: %s\n' "$pattern"
    printf '      path:    %s\n' "$*"
  fi
}

contains_warn() {
  local desc="$1" pattern="$2"; shift 2
  if rg -n -e "$pattern" "$@" >/tmp/phase18_check_rg.txt 2>/dev/null; then
    ok "$desc"
  else
    warn "$desc"
    printf '      pattern: %s\n' "$pattern"
    printf '      path:    %s\n' "$*"
  fi
}

not_contains() {
  local desc="$1" pattern="$2"; shift 2
  if rg -n -e "$pattern" "$@" >/tmp/phase18_check_rg.txt 2>/dev/null; then
    fail "$desc"
    cat /tmp/phase18_check_rg.txt
  else
    ok "$desc"
  fi
}

echo "Phase 18 runtime adapter check"
echo

exists Cargo.toml
exists target-xteink-x4/Cargo.toml
exists target-xteink-x4/src/main.rs
exists target-xteink-x4/src/runtime/mod.rs
exists target-xteink-x4/src/runtime/pulp_runtime.rs
exists vendor/pulp-os/src/bin/main.rs
exists vendor/pulp-os/Cargo.toml
exists vendor/pulp-os/kernel/Cargo.toml
exists vendor/smol-epub/Cargo.toml
exists docs/phase18/PHASE18_RUNTIME_ADAPTER.md
exists docs/phase18/PHASE18_ACCEPTANCE.md
exists scripts/check_reader_runtime_sync_phase18.sh

if [[ "${PHASE18_RUN_CARGO:-0}" == "1" ]]; then
  if cargo metadata --format-version 1 --no-deps >/tmp/phase18_metadata.json; then
    ok "cargo metadata succeeds"
  else
    fail "cargo metadata fails"
  fi

  if cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf; then
    ok "cargo check target-xteink-x4 succeeds"
  else
    fail "cargo check target-xteink-x4 fails"
  fi

  if cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings; then
    ok "cargo clippy target-xteink-x4 succeeds"
  else
    fail "cargo clippy target-xteink-x4 fails"
  fi
else
  ok "cargo check/clippy skipped inside script; set PHASE18_RUN_CARGO=1 to enable"
fi

contains "crate root main loads runtime module" 'mod\s+runtime\s*;' target-xteink-x4/src/main.rs
contains "runtime mod exposes pulp_runtime boundary" 'pub\s+mod\s+pulp_runtime\s*;' target-xteink-x4/src/runtime/mod.rs

contains "Phase 16 marker is still present in runtime" 'phase16=x4-reader-parity-ok' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "Phase 17 marker is still present in runtime" 'phase17=x4-reader-refactor-ok' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "Phase 18 marker is present in runtime" 'phase18=x4-runtime-adapter-ok' target-xteink-x4/src/runtime/pulp_runtime.rs

contains "target depends on vendored x4-os via pulp-os alias" 'pulp-os\s*=\s*\{[^}]*package\s*=\s*"x4-os"[^}]*path\s*=\s*"\.\./vendor/pulp-os"' target-xteink-x4/Cargo.toml
contains "target has direct x4-kernel path dependency" 'x4-kernel\s*=\s*\{[^}]*path\s*=\s*"\.\./vendor/pulp-os/kernel"' target-xteink-x4/Cargo.toml
contains "target has direct smol-epub path dependency" 'smol-epub\s*=\s*\{[^}]*path\s*=\s*"\.\./vendor/smol-epub"' target-xteink-x4/Cargo.toml
contains "root workspace excludes vendored Pulp workspace" 'exclude\s*=.*vendor/pulp-os' Cargo.toml
contains "root workspace excludes vendored smol-epub workspace" 'exclude\s*=.*vendor/smol-epub' Cargo.toml

not_contains "active runtime has no fake/raw EPUB smoke path" 'run_epub_reader_page_storage_smoke|ZIP container parsed|First readable bytes|ensure_pulp_dir_async' target-xteink-x4/src/runtime/pulp_runtime.rs
not_contains "crate root main has no fake/raw EPUB smoke path" 'run_epub_reader_page_storage_smoke|ZIP container parsed|First readable bytes|ensure_pulp_dir_async' target-xteink-x4/src/main.rs

contains "smol-epub path is present for EPUB ZIP/OPF/HTML reader" 'smol_epub|smol-epub' vendor/pulp-os vendor/smol-epub target-xteink-x4/Cargo.toml Cargo.lock
contains "X4/Pulp bookmark path is present" 'bookmark|Bookmark|bookmarks|Bookmarks' vendor/pulp-os/src vendor/pulp-os/kernel
contains "progress/continue path symbols are present" 'progress|Progress|position|Position|offset|Offset|last_read|LastRead|continue|Continue' vendor/pulp-os/src vendor/pulp-os/kernel
contains_warn "theme/font/preset state symbols are present" 'theme|Theme|preset|Preset|font|Font' vendor/pulp-os/src vendor/pulp-os/kernel
contains_warn "reader footer/menu/action symbols are present" 'footer|Footer|quick|Quick|menu|Menu|action|Action' vendor/pulp-os/src vendor/pulp-os/kernel

if ./scripts/check_reader_runtime_sync_phase18.sh; then
  ok "Phase 18 reader runtime sync check passes"
else
  fail "Phase 18 reader runtime sync check fails"
fi

echo
echo "Phase 18 runtime adapter check complete: failures=${failures} warnings=${warnings}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
