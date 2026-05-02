#!/usr/bin/env bash
set -euo pipefail

failures=0
ok() { echo "OK   $*"; }
fail() { echo "FAIL $*"; failures=$((failures + 1)); }

for f in \
  docs/phase35_full/PHASE35_FULL_PHYSICAL_EXTRACTION.md \
  docs/phase35_full/PHASE35_FULL_ACCEPTANCE.md \
  docs/phase35_full/PHASE35_FULL_OWNERSHIP_MATRIX.md \
  docs/phase35_full/PHASE35_FULL_RISK_REGISTER.md \
  docs/phase35_full/PHASE35_FULL_DEVICE_TEST_PLAN.md; do
  [[ -f "$f" ]] && ok "exists: $f" || fail "missing: $f"
done

if cargo metadata --format-version 1 --no-deps >/tmp/phase35_full_metadata.json; then ok "cargo metadata works"; else fail "cargo metadata failed"; fi

if [[ "${PHASE35_FULL_RUN_CARGO:-0}" == "1" ]]; then
  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf || fail "cargo check failed"
  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings || fail "cargo clippy failed"
else
  ok "cargo check/clippy skipped inside script; set PHASE35_FULL_RUN_CARGO=1 to enable"
fi

./scripts/check_phase35_full_no_vendor_edits.sh || fail "vendor edit check failed"
./scripts/check_phase35_full_runtime_ownership.sh || fail "runtime ownership check failed"
./scripts/check_phase35_full_no_scaffold_only.sh || fail "no-scaffold-only check failed"

if rg -n 'run_epub_reader_page_storage_smoke|ZIP container parsed|First readable bytes|ensure_pulp_dir_async' target-xteink-x4/src >/tmp/phase35_full_fake_epub.txt 2>/dev/null; then
  fail "fake EPUB smoke code found"
  cat /tmp/phase35_full_fake_epub.txt
else
  ok "fake EPUB smoke code absent"
fi

if [[ "$failures" -ne 0 ]]; then
  echo "Phase 35 Full physical extraction check complete: failures=$failures"
  exit 1
fi

echo "Phase 35 Full physical extraction check complete: failures=0"
