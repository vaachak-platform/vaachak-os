#!/usr/bin/env bash
set -euo pipefail

failures=0
fail() { echo "FAIL $*"; failures=$((failures + 1)); }
ok() { echo "OK   $*"; }

# These terms are allowed in docs, but active implementation must not present itself as only a scaffold/probe/no-op bridge.
if rg -n 'no-op|noop|preflight only|scaffold only|not active|not wired|deferred|probe only|path-only' target-xteink-x4/src/vaachak_x4 >/tmp/phase35_full_scaffold.txt 2>/dev/null; then
  fail "active Vaachak implementation still contains scaffold/probe/no-op/deferred language"
  cat /tmp/phase35_full_scaffold.txt
else
  ok "active implementation is not marked as scaffold/probe/no-op/deferred"
fi

for area in storage_state_io input_semantics_runtime display_geometry_runtime input_adc spi_bus ssd1677_display; do
  if rg -n 'todo!\(|unimplemented!\(|panic!\("not implemented|return false|TODO|FIXME' "target-xteink-x4/src/vaachak_x4/physical/${area}.rs" >/tmp/phase35_full_todo.txt 2>/dev/null; then
    fail "$area contains TODO/unimplemented placeholders"
    cat /tmp/phase35_full_todo.txt
  else
    ok "$area has no obvious TODO/unimplemented placeholders"
  fi
done

if [[ "$failures" -ne 0 ]]; then
  echo "Phase 35 Full no-scaffold-only check failed: failures=$failures"
  exit 1
fi

echo "Phase 35 Full no-scaffold-only check complete: failures=0"
