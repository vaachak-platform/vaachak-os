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
  if rg -n -e "$pattern" "$@" >/tmp/phase26_check_rg.txt 2>/dev/null; then
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
  if rg -n -e "$pattern" "$@" >/tmp/phase26_check_rg.txt 2>/dev/null; then
    fail "$desc"
    cat /tmp/phase26_check_rg.txt
  else
    ok "$desc"
  fi
}

echo "Phase 26 Vaachak input contract smoke check"
echo

exists Cargo.toml
exists target-xteink-x4/Cargo.toml
exists target-xteink-x4/src/main.rs
exists target-xteink-x4/src/runtime/mod.rs
exists target-xteink-x4/src/runtime/pulp_runtime.rs
exists target-xteink-x4/src/runtime/vaachak_runtime.rs
exists target-xteink-x4/src/runtime/display_boundary.rs
exists target-xteink-x4/src/runtime/input_boundary.rs
exists target-xteink-x4/src/runtime/storage_boundary.rs
exists target-xteink-x4/src/runtime/boundary_contract.rs
exists target-xteink-x4/src/runtime/storage_state_contract.rs
exists target-xteink-x4/src/runtime/input_contract_smoke.rs
exists vendor/pulp-os/src/bin/main.rs
exists vendor/pulp-os/Cargo.toml
exists vendor/pulp-os/kernel/Cargo.toml
exists vendor/smol-epub/Cargo.toml
exists docs/phase26/PHASE26_INPUT_CONTRACT_SMOKE.md
exists docs/phase26/PHASE26_ACCEPTANCE.md
exists scripts/check_reader_runtime_sync_phase26.sh

if [[ "${PHASE26_RUN_CARGO:-0}" == "1" ]]; then
  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings
  ok "cargo check/clippy passed inside script"
else
  ok "cargo check/clippy skipped inside script; set PHASE26_RUN_CARGO=1 to enable"
fi

contains "crate root main loads runtime module" 'mod runtime;' target-xteink-x4/src/main.rs
contains "runtime mod exposes imported Pulp runtime" 'pub mod pulp_runtime;' target-xteink-x4/src/runtime/mod.rs
contains "runtime mod exposes Vaachak facade" 'pub mod vaachak_runtime;' target-xteink-x4/src/runtime/mod.rs
contains "runtime mod exposes input boundary" 'pub mod input_boundary;' target-xteink-x4/src/runtime/mod.rs
contains "runtime mod exposes input contract smoke" 'pub mod input_contract_smoke;' target-xteink-x4/src/runtime/mod.rs

contains "input contract smoke type is present" 'struct VaachakInputContractSmoke' target-xteink-x4/src/runtime/input_contract_smoke.rs
contains "input contract smoke has Phase 26 marker" 'phase26=x4-input-contract-smoke-ok' target-xteink-x4/src/runtime/input_contract_smoke.rs
contains "Vaachak facade emits Phase 26 marker through input contract smoke" 'VaachakInputContractSmoke::emit_contract_marker' target-xteink-x4/src/runtime/vaachak_runtime.rs

contains "input contract records imported runtime behavior owner" 'IMPLEMENTATION_OWNER.*vendor/pulp-os' target-xteink-x4/src/runtime/input_contract_smoke.rs
contains "input contract records physical ADC reads not moved" 'PHYSICAL_ADC_READS_MOVED_IN_PHASE26.*false' target-xteink-x4/src/runtime/input_contract_smoke.rs
contains "input contract records ladder calibration not moved" 'BUTTON_LADDER_CALIBRATION_MOVED_IN_PHASE26.*false' target-xteink-x4/src/runtime/input_contract_smoke.rs
contains "input contract records debounce/repeat not moved" 'DEBOUNCE_REPEAT_HANDLING_MOVED_IN_PHASE26.*false' target-xteink-x4/src/runtime/input_contract_smoke.rs
contains "input contract records event routing not moved" 'BUTTON_EVENT_ROUTING_MOVED_IN_PHASE26.*false' target-xteink-x4/src/runtime/input_contract_smoke.rs

contains "input contract references X4 row1 ADC pin from boundary" 'VaachakInputBoundary::ROW1_ADC_GPIO' target-xteink-x4/src/runtime/input_contract_smoke.rs
contains "input contract references X4 row2 ADC pin from boundary" 'VaachakInputBoundary::ROW2_ADC_GPIO' target-xteink-x4/src/runtime/input_contract_smoke.rs
contains "input contract references X4 power button pin from boundary" 'VaachakInputBoundary::POWER_BUTTON_GPIO' target-xteink-x4/src/runtime/input_contract_smoke.rs

contains "input contract requires Back role" 'VaachakButtonRole::Back' target-xteink-x4/src/runtime/input_contract_smoke.rs
contains "input contract requires Select role" 'VaachakButtonRole::Select' target-xteink-x4/src/runtime/input_contract_smoke.rs
contains "input contract requires Up role" 'VaachakButtonRole::Up' target-xteink-x4/src/runtime/input_contract_smoke.rs
contains "input contract requires Down role" 'VaachakButtonRole::Down' target-xteink-x4/src/runtime/input_contract_smoke.rs
contains "input contract requires Left role" 'VaachakButtonRole::Left' target-xteink-x4/src/runtime/input_contract_smoke.rs
contains "input contract requires Right role" 'VaachakButtonRole::Right' target-xteink-x4/src/runtime/input_contract_smoke.rs
contains "input contract requires Power role" 'VaachakButtonRole::Power' target-xteink-x4/src/runtime/input_contract_smoke.rs

contains "input contract defines library/reader/system context" 'enum VaachakInputContext' target-xteink-x4/src/runtime/input_contract_smoke.rs
contains "input contract defines semantic reader actions" 'enum VaachakReaderInputAction' target-xteink-x4/src/runtime/input_contract_smoke.rs
contains "input contract validates expected physical input pins" 'is_expected_physical_input_pin' target-xteink-x4/src/runtime/input_contract_smoke.rs
contains "input contract validates supported button roles" 'is_button_role_supported' target-xteink-x4/src/runtime/input_contract_smoke.rs
contains "input contract maps roles to semantic actions" 'action_for_role' target-xteink-x4/src/runtime/input_contract_smoke.rs
contains "input contract exposes smoke validator" 'smoke_validate_contract' target-xteink-x4/src/runtime/input_contract_smoke.rs

contains "input contract maps Back to library" 'BackToLibrary' target-xteink-x4/src/runtime/input_contract_smoke.rs
contains "input contract maps Select/Open" 'SelectOrOpen' target-xteink-x4/src/runtime/input_contract_smoke.rs
contains "input contract maps Next Page" 'NextPage' target-xteink-x4/src/runtime/input_contract_smoke.rs
contains "input contract maps Previous Page" 'PreviousPage' target-xteink-x4/src/runtime/input_contract_smoke.rs
contains "input contract maps Bookmark/Menu" 'BookmarkOrMenu' target-xteink-x4/src/runtime/input_contract_smoke.rs

contains "Phase 16 marker is still present in imported runtime" 'phase16=x4-reader-parity-ok' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "Phase 17 marker is still present in imported runtime" 'phase17=x4-reader-refactor-ok' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "Phase 18 marker is still present in imported runtime" 'phase18=x4-runtime-adapter-ok' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "Phase 19 marker is still present in Vaachak facade" 'phase19=x4-vaachak-runtime-facade-ok' target-xteink-x4/src/runtime/vaachak_runtime.rs
contains "Phase 20 marker is still present" 'phase20=x4-boundary-scaffold-ok' target-xteink-x4/src/runtime/display_boundary.rs target-xteink-x4/src/runtime/vaachak_runtime.rs
contains "Phase 21 marker is still present" 'phase21=x4-storage-boundary-ok' target-xteink-x4/src/runtime/storage_boundary.rs target-xteink-x4/src/runtime/vaachak_runtime.rs
contains "Phase 22 marker is still present" 'phase22=x4-input-boundary-ok' target-xteink-x4/src/runtime/input_boundary.rs target-xteink-x4/src/runtime/vaachak_runtime.rs
contains "Phase 23 marker is still present" 'phase23=x4-display-boundary-ok' target-xteink-x4/src/runtime/display_boundary.rs target-xteink-x4/src/runtime/vaachak_runtime.rs
contains "Phase 24 marker is still present" 'phase24=x4-boundary-contract-ok' target-xteink-x4/src/runtime/boundary_contract.rs target-xteink-x4/src/runtime/vaachak_runtime.rs
contains "Phase 25 marker is still present" 'phase25=x4-storage-contract-smoke-ok' target-xteink-x4/src/runtime/storage_state_contract.rs target-xteink-x4/src/runtime/vaachak_runtime.rs

contains "imported runtime still calls Vaachak facade marker" 'VaachakRuntime::emit_boot_marker' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "imported runtime remains the working async entrypoint" 'async fn main\s*\(' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "imported runtime keeps esp-rs runtime entrypoint attribute" '#\[(esp_rtos::main|main)\]' target-xteink-x4/src/runtime/pulp_runtime.rs

not_contains "input contract does not perform ADC sampling or hardware init" 'Adc::|read_oneshot|Input::new|Pull::|Delay::new|SPI2|with_dma|SdCard::new' target-xteink-x4/src/runtime/input_contract_smoke.rs
not_contains "active runtime has no fake/raw EPUB smoke path" 'run_epub_reader_page_storage_smoke|ZIP container parsed|First readable bytes|ensure_pulp_dir_async' target-xteink-x4/src/runtime target-xteink-x4/src/main.rs
not_contains "vendored runtime files were not edited for Phase 26" 'phase26=x4-input-contract-smoke-ok' vendor/pulp-os vendor/smol-epub

contains "target depends on vendored x4-os via pulp-os alias" 'pulp-os\s*=\s*\{[^}]*package\s*=\s*"x4-os"[^}]*path\s*=\s*"\.\./vendor/pulp-os"' target-xteink-x4/Cargo.toml
contains "target has direct x4-kernel path dependency" 'x4-kernel\s*=\s*\{[^}]*path\s*=\s*"\.\./vendor/pulp-os/kernel"' target-xteink-x4/Cargo.toml
contains "target has direct smol-epub path dependency" 'smol-epub\s*=\s*\{[^}]*path\s*=\s*"\.\./vendor/smol-epub"' target-xteink-x4/Cargo.toml
contains "root workspace excludes vendored Pulp workspace" 'exclude\s*=.*vendor/pulp-os' Cargo.toml
contains "root workspace excludes vendored smol-epub workspace" 'exclude\s*=.*vendor/smol-epub' Cargo.toml

contains "smol-epub path is present for EPUB ZIP/OPF/HTML reader" 'smol_epub|smol-epub' vendor/pulp-os vendor/smol-epub target-xteink-x4/Cargo.toml Cargo.lock
contains "X4/Pulp bookmark path is present" 'bookmark|Bookmark|bookmarks|Bookmarks' vendor/pulp-os/src vendor/pulp-os/kernel
contains "progress/continue path symbols are present" 'progress|Progress|position|Position|offset|Offset|last_read|LastRead|continue|Continue' vendor/pulp-os/src vendor/pulp-os/kernel
contains "theme/font/preset state symbols are present" 'theme|Theme|preset|Preset|font|Font' vendor/pulp-os/src vendor/pulp-os/kernel
contains "reader footer/menu/action symbols are present" 'footer|Footer|quick|Quick|menu|Menu|action|Action' vendor/pulp-os/src vendor/pulp-os/kernel

if ./scripts/check_reader_runtime_sync_phase26.sh; then
  ok "Phase 26 reader runtime sync check passes"
else
  fail "Phase 26 reader runtime sync check fails"
fi

echo
echo "Phase 26 input contract smoke check complete: failures=${failures} warnings=${warnings}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
