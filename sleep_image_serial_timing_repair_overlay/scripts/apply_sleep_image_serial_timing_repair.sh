#!/usr/bin/env bash
set -euo pipefail

repo="${1:-.}"
cd "$repo"

if [ ! -f Cargo.toml ]; then
  echo "error: run from the vaachak-os repository root or pass the repository path" >&2
  exit 1
fi

scheduler="vendor/pulp-os/kernel/src/kernel/scheduler.rs"
sleep_bitmap="vendor/pulp-os/kernel/src/kernel/sleep_bitmap.rs"

if [ ! -f "$scheduler" ]; then
  echo "error: missing $scheduler" >&2
  exit 1
fi
if [ ! -f "$sleep_bitmap" ]; then
  echo "error: missing $sleep_bitmap; apply the direct sleep loader first" >&2
  exit 1
fi

stamp="$(date +%Y%m%d-%H%M%S)"
backup_dir=".vaachak_pre_github_backups/sleep_image_serial_timing_repair_${stamp}"
mkdir -p "$backup_dir/$(dirname "$scheduler")" "$backup_dir/$(dirname "$sleep_bitmap")"
cp "$scheduler" "$backup_dir/$scheduler"
cp "$sleep_bitmap" "$backup_dir/$sleep_bitmap"

python3 - <<'PY'
from pathlib import Path

scheduler = Path("vendor/pulp-os/kernel/src/kernel/scheduler.rs")
sleep_bitmap = Path("vendor/pulp-os/kernel/src/kernel/sleep_bitmap.rs")

# Ensure SleepImageMode has a stable printable name even if the previous overlay
# added the enum without this helper.
text = sleep_bitmap.read_text()
if "pub enum SleepImageMode" not in text:
    raise SystemExit("sleep_bitmap.rs is missing SleepImageMode; apply sleep-image mode support first")
if "pub const fn name(self) -> &'static str" not in text:
    enum_end = text.find("}\n", text.find("pub enum SleepImageMode"))
    if enum_end == -1:
        raise SystemExit("could not locate SleepImageMode enum end")
    insert_at = enum_end + 2
    impl = r'''

impl SleepImageMode {
    pub const fn name(self) -> &'static str {
        match self {
            Self::DailyMantra => "daily",
            Self::StaticBitmap => "static",
            Self::TextFallback => "text",
            Self::Disabled => "off",
        }
    }
}
'''
    text = text[:insert_at] + impl + text[insert_at:]
    sleep_bitmap.write_text(text)

text = scheduler.read_text()
fn_name = "async fn render_daily_sleep_bitmap"
start = text.find(fn_name)
if start == -1:
    raise SystemExit("scheduler.rs is missing render_daily_sleep_bitmap")
brace = text.find("{", start)
if brace == -1:
    raise SystemExit("could not find render_daily_sleep_bitmap body start")

depth = 0
end = None
for i in range(brace, len(text)):
    ch = text[i]
    if ch == "{":
        depth += 1
    elif ch == "}":
        depth -= 1
        if depth == 0:
            end = i + 1
            break
if end is None:
    raise SystemExit("could not find render_daily_sleep_bitmap body end")

replacement = r'''async fn render_daily_sleep_bitmap(&mut self) -> bool {
        use core::cell::Cell;
        use embassy_time::Instant;
        use super::sleep_bitmap::{self, SleepImageMode};

        let total_start = Instant::now();

        let mode_start = Instant::now();
        let mode = sleep_bitmap::read_sleep_image_mode(&self.sd);
        esp_println::println!(
            "sleep image: mode={} mode_read_ms={}",
            mode.name(),
            mode_start.elapsed().as_millis()
        );

        match mode {
            SleepImageMode::Disabled => {
                esp_println::println!(
                    "sleep image: disabled total_ms={}",
                    total_start.elapsed().as_millis()
                );
                return true;
            }
            SleepImageMode::TextFallback => {
                esp_println::println!(
                    "sleep image: text fallback requested total_ms={}",
                    total_start.elapsed().as_millis()
                );
                return false;
            }
            SleepImageMode::DailyMantra | SleepImageMode::StaticBitmap => {}
        }

        let resolve_start = Instant::now();
        let Some(info) = sleep_bitmap::resolve_sleep_bitmap_for_mode(&self.sd, mode) else {
            esp_println::println!(
                "sleep image: no valid bitmap found mode={} resolve_ms={} total_ms={}",
                mode.name(),
                resolve_start.elapsed().as_millis(),
                total_start.elapsed().as_millis()
            );
            return false;
        };
        esp_println::println!(
            "sleep image: bitmap resolved mode={} resolve_ms={}",
            mode.name(),
            resolve_start.elapsed().as_millis()
        );

        let render_start = Instant::now();
        let ok = Cell::new(true);
        let sd = &self.sd;
        self.epd
            .full_refresh_async(self.strip, &mut self.delay, &|s: &mut StripBuffer| {
                if !sleep_bitmap::draw_sleep_bitmap_strip(sd, &info, s) {
                    ok.set(false);
                }
            })
            .await;

        if ok.get() {
            esp_println::println!(
                "display: sleep bitmap rendered mode={} render_ms={} total_ms={}",
                mode.name(),
                render_start.elapsed().as_millis(),
                total_start.elapsed().as_millis()
            );
        } else {
            esp_println::println!(
                "display: sleep bitmap render failed mode={} render_ms={} total_ms={}",
                mode.name(),
                render_start.elapsed().as_millis(),
                total_start.elapsed().as_millis()
            );
        }
        ok.get()
    }'''

text = text[:start] + replacement + text[end:]

# Make fallback rendering visible even if log::info is filtered or deep sleep races the logger.
text = text.replace(
    'info!("display: fallback sleep screen rendered");',
    'esp_println::println!("display: fallback sleep screen rendered");'
)
text = text.replace(
    'info!("display: deep sleep mode 1");',
    'esp_println::println!("display: deep sleep mode 1");'
)
text = text.replace(
    'info!("mcu: entering deep sleep (power button to wake, RTC FAST retained)");',
    'esp_println::println!("mcu: entering deep sleep (power button to wake, RTC FAST retained)");'
)

scheduler.write_text(text)
PY

cargo fmt --all

echo "Applied serial sleep-image timing repair. Backup: $backup_dir"
