#!/usr/bin/env bash
set -euo pipefail

failures=0
ok() { printf 'OK   %s\n' "$*"; }
fail() { printf 'FAIL %s\n' "$*"; failures=$((failures + 1)); }
exists() { [[ -e "$1" ]] && ok "exists: $1" || fail "missing: $1"; }
contains() {
  local desc="$1" pattern="$2"; shift 2
  if rg -n -e "$pattern" "$@" >/tmp/phase35_full_rg.txt 2>/dev/null; then ok "$desc"; else fail "$desc"; printf '      pattern: %s\n      path: %s\n' "$pattern" "$*"; fi
}
not_contains() {
  local desc="$1" pattern="$2"; shift 2
  if rg -n -e "$pattern" "$@" >/tmp/phase35_full_rg.txt 2>/dev/null; then fail "$desc"; cat /tmp/phase35_full_rg.txt; else ok "$desc"; fi
}

exists target-xteink-x4/src/vaachak_x4/physical/mod.rs
exists target-xteink-x4/src/vaachak_x4/physical/runtime.rs
exists target-xteink-x4/src/vaachak_x4/physical/storage_state_io.rs
exists target-xteink-x4/src/vaachak_x4/physical/input_semantics_runtime.rs
exists target-xteink-x4/src/vaachak_x4/physical/display_geometry_runtime.rs
exists target-xteink-x4/src/vaachak_x4/physical/input_adc.rs
exists target-xteink-x4/src/vaachak_x4/physical/spi_bus.rs
exists target-xteink-x4/src/vaachak_x4/physical/ssd1677_display.rs
exists target-xteink-x4/src/vaachak_x4/apps/mod.rs
exists target-xteink-x4/src/vaachak_x4/apps/reader.rs
exists target-xteink-x4/src/vaachak_x4/apps/app_manager.rs

contains "boot marker is physical-runtime-owned" 'vaachak=x4-physical-runtime-owned' target-xteink-x4/src
not_contains "normal boot does not emit old phase markers" 'println!\("phase(16|17|18|19|20|21|22|23|24|25|26|27|28|29|30|31|32|33|34|35|35b)=' target-xteink-x4/src

contains "storage state IO active module has read/write" 'read_.*state|write_.*state|Progress|Bookmark|Theme|Metadata|BMIDX' target-xteink-x4/src/vaachak_x4/physical/storage_state_io.rs
contains "input semantics active runtime exists" 'Back|Select|Open|Up|Down|Left|Right|Power|Next|Previous|Bookmark|Menu' target-xteink-x4/src/vaachak_x4/physical/input_semantics_runtime.rs
contains "display geometry active runtime exists" '800|480|rotation|portrait|strip|geometry|bounds' target-xteink-x4/src/vaachak_x4/physical/display_geometry_runtime.rs
contains "input ADC/debounce active module exists" 'GPIO1|GPIO2|GPIO3|ADC|debounce|threshold|repeat|hold|read_oneshot' target-xteink-x4/src/vaachak_x4/physical/input_adc.rs
contains "SD/SPI arbitration active module exists" 'GPIO8|GPIO10|GPIO7|GPIO21|GPIO12|transaction|arbitr|shared|spi' target-xteink-x4/src/vaachak_x4/physical/spi_bus.rs
contains "SSD1677 active module exists" 'SSD1677|0x24|0x26|0x22|0x20|refresh|strip|busy|write_cmd|write_data' target-xteink-x4/src/vaachak_x4/physical/ssd1677_display.rs
contains "reader app internals are Vaachak-owned" 'Reader|EPUB|EPU|TXT|bookmark|progress|continue|theme|footer|menu' target-xteink-x4/src/vaachak_x4/apps

not_contains "active target no longer uses Pulp black-box runtime/app manager" 'pulp_os::apps|pulp_os::app|x4_kernel::|vendor/pulp-os/src/apps' target-xteink-x4/src/vaachak_x4

if [[ "$failures" -ne 0 ]]; then
  echo "Phase 35 Full runtime ownership check failed: failures=$failures"
  exit 1
fi

echo "Phase 35 Full runtime ownership check complete: failures=0"
