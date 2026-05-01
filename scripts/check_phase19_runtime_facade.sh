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
  local desc="$1"
  local pattern="$2"
  shift 2
  if rg -n -e "$pattern" "$@" >/tmp/phase19_check_rg.txt 2>/dev/null; then
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
  if rg -n -e "$pattern" "$@" >/tmp/phase19_check_rg.txt 2>/dev/null; then
    fail "$desc"
    cat /tmp/phase19_check_rg.txt
  else
    ok "$desc"
  fi
}

echo "Phase 19 Vaachak runtime facade check"
echo

exists Cargo.toml
exists target-xteink-x4/Cargo.toml
exists target-xteink-x4/src/main.rs
exists target-xteink-x4/src/runtime/mod.rs
exists target-xteink-x4/src/runtime/pulp_runtime.rs
exists target-xteink-x4/src/runtime/vaachak_runtime.rs
exists vendor/pulp-os/src/bin/main.rs
exists vendor/pulp-os/Cargo.toml
exists vendor/pulp-os/kernel/Cargo.toml
exists vendor/smol-epub/Cargo.toml
exists docs/phase19/PHASE19_RUNTIME_FACADE.md
exists docs/phase19/PHASE19_ACCEPTANCE.md
exists scripts/check_reader_runtime_sync_phase19.sh

if [[ "${PHASE19_RUN_CARGO:-0}" == "1" ]]; then
  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings
  ok "cargo check/clippy passed inside script"
else
  ok "cargo check/clippy skipped inside script; set PHASE19_RUN_CARGO=1 to enable"
fi

contains "crate root main loads runtime module" 'mod runtime;' target-xteink-x4/src/main.rs
contains "runtime mod exposes imported Pulp runtime" 'pub mod pulp_runtime;' target-xteink-x4/src/runtime/mod.rs
contains "runtime mod exposes Vaachak facade" 'pub mod vaachak_runtime;' target-xteink-x4/src/runtime/mod.rs

contains "VaachakRuntime facade type is present" 'struct VaachakRuntime' target-xteink-x4/src/runtime/vaachak_runtime.rs
contains "Vaachak facade owns Phase 19 marker method" 'emit_boot_marker' target-xteink-x4/src/runtime/vaachak_runtime.rs
contains "Phase 19 marker is present in Vaachak facade" 'phase19=x4-vaachak-runtime-facade-ok' target-xteink-x4/src/runtime/vaachak_runtime.rs

contains "imported runtime remains the working async entrypoint" 'async fn main\s*\(' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "imported runtime keeps esp-rs runtime entrypoint attribute" '#\[(esp_rtos::main|main)\]' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "imported runtime calls Vaachak facade marker" 'VaachakRuntime::emit_boot_marker' target-xteink-x4/src/runtime/pulp_runtime.rs

not_contains "Vaachak facade does not own duplicate esp entrypoint" '#\[(esp_hal::main|esp_rtos::main|main)\]' target-xteink-x4/src/runtime/vaachak_runtime.rs
not_contains "imported runtime has no module-invalid crate attrs" '^#!\[(no_std|no_main)\]' target-xteink-x4/src/runtime/pulp_runtime.rs

contains "Phase 16 marker is still present in imported runtime" 'phase16=x4-reader-parity-ok' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "Phase 17 marker is still present in imported runtime" 'phase17=x4-reader-refactor-ok' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "Phase 18 marker is still present in imported runtime" 'phase18=x4-runtime-adapter-ok' target-xteink-x4/src/runtime/pulp_runtime.rs

contains "target depends on vendored x4-os via pulp-os alias" 'pulp-os\s*=\s*\{[^}]*package\s*=\s*"x4-os"[^}]*path\s*=\s*"\.\./vendor/pulp-os"' target-xteink-x4/Cargo.toml
contains "target has direct x4-kernel path dependency" 'x4-kernel\s*=\s*\{[^}]*path\s*=\s*"\.\./vendor/pulp-os/kernel"' target-xteink-x4/Cargo.toml
contains "target has direct smol-epub path dependency" 'smol-epub\s*=\s*\{[^}]*path\s*=\s*"\.\./vendor/smol-epub"' target-xteink-x4/Cargo.toml

contains "root workspace excludes vendored Pulp workspace" 'exclude\s*=.*vendor/pulp-os' Cargo.toml
contains "root workspace excludes vendored smol-epub workspace" 'exclude\s*=.*vendor/smol-epub' Cargo.toml

not_contains "active runtime has no fake/raw EPUB smoke path" 'run_epub_reader_page_storage_smoke|ZIP container parsed|First readable bytes|ensure_pulp_dir_async' target-xteink-x4/src/runtime target-xteink-x4/src/main.rs

contains "smol-epub path is present for EPUB ZIP/OPF/HTML reader" 'smol_epub|smol-epub' vendor/pulp-os vendor/smol-epub target-xteink-x4/Cargo.toml Cargo.lock
contains "X4/Pulp bookmark path is present" 'bookmark|Bookmark|bookmarks|Bookmarks' vendor/pulp-os/src vendor/pulp-os/kernel
contains "progress/continue path symbols are present" 'progress|Progress|position|Position|offset|Offset|last_read|LastRead|continue|Continue' vendor/pulp-os/src vendor/pulp-os/kernel
contains "theme/font/preset state symbols are present" 'theme|Theme|preset|Preset|font|Font' vendor/pulp-os/src vendor/pulp-os/kernel
contains "reader footer/menu/action symbols are present" 'footer|Footer|quick|Quick|menu|Menu|action|Action' vendor/pulp-os/src vendor/pulp-os/kernel

if ./scripts/check_reader_runtime_sync_phase19.sh; then
  ok "Phase 19 reader runtime sync check passes"
else
  fail "Phase 19 reader runtime sync check fails"
fi

echo
echo "Phase 19 runtime facade check complete: failures=${failures} warnings=${warnings}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
