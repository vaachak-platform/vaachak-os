#!/usr/bin/env bash
set -euo pipefail

fail() { echo "production_repo_hygiene failed: $*" >&2; exit 1; }

[ -f target-xteink-x4/src/vaachak_x4/physical/mod.rs ] || fail "missing physical mod.rs"
[ -f target-xteink-x4/src/vaachak_x4/physical/spi_bus_runtime.rs ] || fail "missing production spi_bus_runtime.rs"
[ -f target-xteink-x4/src/vaachak_x4/imported/x4_reader_runtime.rs ] || fail "missing Vaachak-owned X4 reader runtime"

if rg -n "hardware_runtime_executor_(boot_markers|runtime_use|live_handoff)" target-xteink-x4/src/vaachak_x4 >/tmp/vaachak_handoff_refs.txt; then
  cat /tmp/vaachak_handoff_refs.txt >&2
  fail "deleted hardware runtime executor handoff references remain"
fi

if rg -n "hardware_runtime_backend(_pulp|_takeover|_takeover_cleanup)?|display_runtime_owner|input_runtime_owner|sd_fat_runtime_readonly_owner|storage_probe_mount_runtime_executor_bridge|spi_bus_arbitration_runtime_owner|physical_driver_migration_plan" target-xteink-x4/src/vaachak_x4/physical target-xteink-x4/src/vaachak_x4/imported target-xteink-x4/src/vaachak_x4/boot.rs >/tmp/vaachak_deleted_refs.txt; then
  cat /tmp/vaachak_deleted_refs.txt >&2
  fail "deleted transition module references remain"
fi


if [ -d vendor/pulp-os ]; then
  fail "vendor/pulp-os must not remain in the active repository after runtime retirement"
fi

if rg -n 'pulp-os|pulp_os::|x4-kernel =|package = "x4-os"|vendor/pulp-os' Cargo.toml target-xteink-x4/Cargo.toml target-xteink-x4/src >/tmp/vaachak_vendor_runtime_refs.txt; then
  cat /tmp/vaachak_vendor_runtime_refs.txt >&2
  fail "active vendor Pulp runtime references remain"
fi

if find target-xteink-x4/src/vaachak_x4/contracts -maxdepth 1 -name '*_smoke.rs' | grep -q .; then
  find target-xteink-x4/src/vaachak_x4/contracts -maxdepth 1 -name '*_smoke.rs' >&2
  fail "smoke contract modules remain in production contract tree"
fi

if find . -maxdepth 1 \( -name '*.zip' -o -name '*_fix' -o -name '*_cleanup' -o -name '*_migration' \) | grep -q .; then
  find . -maxdepth 1 \( -name '*.zip' -o -name '*_fix' -o -name '*_cleanup' -o -name '*_migration' \) >&2
  fail "generated overlay artifacts remain in repo root"
fi

echo "production_repo_hygiene=ok"
