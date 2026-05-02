#!/usr/bin/env bash
set -euo pipefail

cat <<'MSG'
Phase 35H-0 revert helper

This phase added a pure Vaachak SPI bus arbitration facade and a silent
runtime preflight call. Review with:

  git diff -- target-xteink-x4/src/vaachak_x4/mod.rs target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs target-xteink-x4/src/vaachak_x4/physical docs/phase35h0 scripts/check_phase35h0_spi_bus_arbitration_facade.sh scripts/check_phase35h0_no_spi_hardware_takeover.sh scripts/check_imported_reader_runtime_sync_phase35b.sh

No automatic destructive action is performed by this helper.
MSG
