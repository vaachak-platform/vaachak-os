#!/usr/bin/env bash
set -euo pipefail

repo="${1:-.}"
cd "$repo"

if [ ! -f Cargo.toml ]; then
  echo "error: run from vaachak-os repository root or pass repo path" >&2
  exit 1
fi

stamp="$(date +%Y%m%d-%H%M%S)"
backup_dir=".vaachak_pre_github_backups/sleep_bitmap_prefetch_cache_${stamp}"
mkdir -p "$backup_dir"

backup_if_exists() {
  local path="$1"
  if [ -e "$path" ]; then
    mkdir -p "$backup_dir/$(dirname "$path")"
    cp -R "$path" "$backup_dir/$path"
  fi
}

backup_if_exists vendor/pulp-os/kernel/src/kernel/sleep_bitmap.rs
backup_if_exists vendor/pulp-os/kernel/src/kernel/scheduler.rs
backup_if_exists scripts/write_sleep_image_mode.sh
backup_if_exists scripts/verify_sleep_image_mode.sh
backup_if_exists scripts/write_sleep_image_cache_hint.sh
backup_if_exists scripts/clear_sleep_image_cache_hint.sh

mkdir -p scripts vendor/pulp-os/kernel/src/kernel
cp -f sleep_bitmap_prefetch_cache_overlay/files/vendor/pulp-os/kernel/src/kernel/sleep_bitmap.rs vendor/pulp-os/kernel/src/kernel/sleep_bitmap.rs
cp -f sleep_bitmap_prefetch_cache_overlay/files/scripts/write_sleep_image_mode.sh scripts/write_sleep_image_mode.sh
cp -f sleep_bitmap_prefetch_cache_overlay/files/scripts/verify_sleep_image_mode.sh scripts/verify_sleep_image_mode.sh
cp -f sleep_bitmap_prefetch_cache_overlay/files/scripts/write_sleep_image_cache_hint.sh scripts/write_sleep_image_cache_hint.sh
cp -f sleep_bitmap_prefetch_cache_overlay/files/scripts/clear_sleep_image_cache_hint.sh scripts/clear_sleep_image_cache_hint.sh
chmod +x scripts/write_sleep_image_mode.sh scripts/verify_sleep_image_mode.sh scripts/write_sleep_image_cache_hint.sh scripts/clear_sleep_image_cache_hint.sh

python3 - <<'PY'
from pathlib import Path
import re

path = Path("vendor/pulp-os/kernel/src/kernel/scheduler.rs")
text = path.read_text()

start = text.find("    async fn render_daily_sleep_bitmap(&mut self) -> bool {")
end_marker = "    fn wait_for_power_button_release_before_sleep"
end = text.find(end_marker, start)
if start == -1 or end == -1:
    raise SystemExit("could not find render_daily_sleep_bitmap function boundaries")

new_func = r'''    async fn render_daily_sleep_bitmap(&mut self) -> bool {
        use super::sleep_bitmap::{self, SleepImageMode};
        use core::cell::Cell;
        use embassy_time::Instant;

        let total_start = Instant::now();

        let mode_start = Instant::now();
        let mode = sleep_bitmap::read_sleep_image_mode(&self.sd);
        esp_println::println!(
            "sleep image: mode={} mode_read_ms={}",
            mode.name(),
            mode_start.elapsed().as_millis()
        );

        match mode {
            SleepImageMode::NoRedraw => {
                esp_println::println!(
                    "sleep image: no-redraw total_ms={}",
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
            SleepImageMode::DailyMantra
            | SleepImageMode::FastDaily
            | SleepImageMode::StaticBitmap
            | SleepImageMode::Cached => {}
        }

        let bmp_decode_ms = Cell::new(0u64);
        let resolve_start = Instant::now();
        let Some(info) =
            sleep_bitmap::resolve_sleep_bitmap_for_mode_timed(&self.sd, mode, &bmp_decode_ms)
        else {
            esp_println::println!(
                "sleep image: no valid bitmap found mode={} resolve_ms={} bmp_decode_ms={} total_ms={}",
                mode.name(),
                resolve_start.elapsed().as_millis(),
                bmp_decode_ms.get(),
                total_start.elapsed().as_millis()
            );
            return false;
        };

        let cache_key = sleep_bitmap::sleep_bitmap_cache_hint_for_info(&info);
        esp_println::println!(
            "sleep image: bitmap resolved mode={} resolve_ms={} bmp_decode_ms={} cache_key={}",
            mode.name(),
            resolve_start.elapsed().as_millis(),
            bmp_decode_ms.get(),
            cache_key
        );

        if mode == SleepImageMode::Cached && sleep_bitmap::sleep_bitmap_cache_hint_matches(&self.sd, &info) {
            esp_println::println!(
                "sleep image: cached redraw skipped mode={} total_ms={}",
                mode.name(),
                total_start.elapsed().as_millis()
            );
            return true;
        }

        let bmp_prefetch_ms = Cell::new(0u64);
        let prefetched = sleep_bitmap::prefetch_sleep_bitmap_timed(&self.sd, &info, &bmp_prefetch_ms);
        if prefetched.is_some() {
            esp_println::println!(
                "sleep image: bitmap prefetched mode={} bmp_prefetch_ms={}",
                mode.name(),
                bmp_prefetch_ms.get()
            );
        } else {
            esp_println::println!(
                "sleep image: prefetch unavailable mode={} bmp_prefetch_ms={} fallback=streaming",
                mode.name(),
                bmp_prefetch_ms.get()
            );
        }

        let bmp_draw_ms = Cell::new(0u64);
        let ok = Cell::new(true);
        let sd = &self.sd;
        let epd_start = Instant::now();

        match mode {
            SleepImageMode::FastDaily => {
                self.epd
                    .partial_refresh_async(
                        self.strip,
                        &mut self.delay,
                        0,
                        0,
                        800,
                        480,
                        &|s: &mut StripBuffer| {
                            let drawn = if let Some(bitmap) = prefetched.as_ref() {
                                sleep_bitmap::draw_prefetched_sleep_bitmap_strip_timed(
                                    bitmap,
                                    s,
                                    &bmp_draw_ms,
                                )
                            } else {
                                sleep_bitmap::draw_sleep_bitmap_strip_timed(sd, &info, s, &bmp_draw_ms)
                            };
                            if !drawn {
                                ok.set(false);
                            }
                        },
                    )
                    .await;
            }
            SleepImageMode::DailyMantra | SleepImageMode::StaticBitmap | SleepImageMode::Cached => {
                self.epd
                    .full_refresh_async(self.strip, &mut self.delay, &|s: &mut StripBuffer| {
                        let drawn = if let Some(bitmap) = prefetched.as_ref() {
                            sleep_bitmap::draw_prefetched_sleep_bitmap_strip_timed(
                                bitmap,
                                s,
                                &bmp_draw_ms,
                            )
                        } else {
                            sleep_bitmap::draw_sleep_bitmap_strip_timed(sd, &info, s, &bmp_draw_ms)
                        };
                        if !drawn {
                            ok.set(false);
                        }
                    })
                    .await;
            }
            SleepImageMode::TextFallback | SleepImageMode::NoRedraw => {}
        }

        let epd_refresh_ms = epd_start.elapsed().as_millis();
        if ok.get() {
            esp_println::println!(
                "display: sleep bitmap rendered mode={} bmp_prefetch_ms={} bmp_draw_ms={} bmp_decode_ms={} epd_refresh_ms={} total_ms={}",
                mode.name(),
                bmp_prefetch_ms.get(),
                bmp_draw_ms.get(),
                bmp_decode_ms.get(),
                epd_refresh_ms,
                total_start.elapsed().as_millis()
            );
        } else {
            esp_println::println!(
                "display: sleep bitmap render failed mode={} bmp_prefetch_ms={} bmp_draw_ms={} bmp_decode_ms={} epd_refresh_ms={} total_ms={}",
                mode.name(),
                bmp_prefetch_ms.get(),
                bmp_draw_ms.get(),
                bmp_decode_ms.get(),
                epd_refresh_ms,
                total_start.elapsed().as_millis()
            );
        }
        ok.get()
    }

'''

text = text[:start] + new_func + text[end:]
path.write_text(text)
PY

cargo fmt --all

echo "Applied sleep bitmap prefetch cache support. Backup: $backup_dir"
