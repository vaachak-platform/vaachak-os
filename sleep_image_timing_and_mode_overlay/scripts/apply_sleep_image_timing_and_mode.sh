#!/usr/bin/env bash
set -euo pipefail

repo="${1:-.}"
cd "$repo"

if [ ! -f Cargo.toml ]; then
  echo "error: run from the vaachak-os repository root or pass the repository path" >&2
  exit 1
fi

stamp="$(date +%Y%m%d-%H%M%S)"
backup_dir=".vaachak_pre_github_backups/sleep_image_timing_and_mode_${stamp}"
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

cp -f sleep_image_timing_and_mode_overlay/files/scripts/write_sleep_image_mode.sh scripts/write_sleep_image_mode.sh
cp -f sleep_image_timing_and_mode_overlay/files/scripts/verify_sleep_image_mode.sh scripts/verify_sleep_image_mode.sh
chmod +x scripts/write_sleep_image_mode.sh scripts/verify_sleep_image_mode.sh

python3 - <<'PY'
from pathlib import Path

sleep_bitmap = Path("vendor/pulp-os/kernel/src/kernel/sleep_bitmap.rs")
if not sleep_bitmap.exists():
    raise SystemExit("missing sleep_bitmap.rs; apply the direct sleep loader first")
text = sleep_bitmap.read_text()

# Add mode model and parser after constants.
if "pub enum SleepImageMode" not in text:
    marker = "pub const ROOT_SLEEP_BITMAP: SleepBitmapCandidate = SleepBitmapCandidate::root(\"sleep.bmp\");\n"
    insert = marker + """
pub const SLEEP_IMAGE_MODE_FILE: SleepBitmapCandidate = SleepBitmapCandidate::root("SLPMODE.TXT");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SleepImageMode {
    DailyMantra,
    StaticBitmap,
    TextFallback,
    Disabled,
}

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

"""
    if marker not in text:
        raise SystemExit("sleep_bitmap.rs root sleep bitmap marker not found")
    text = text.replace(marker, insert)

# Add mode-aware resolver before the existing resolver.
if "pub fn read_sleep_image_mode" not in text:
    marker = "pub fn resolve_sleep_bitmap(sd: &SdStorage) -> Option<SleepBitmapInfo> {\n"
    insert = """
pub fn read_sleep_image_mode(sd: &SdStorage) -> SleepImageMode {
    let mut buf = [0u8; 32];
    let Ok((_size, n)) = read_start(sd, SLEEP_IMAGE_MODE_FILE, &mut buf) else {
        return SleepImageMode::DailyMantra;
    };
    parse_sleep_image_mode(&buf[..n]).unwrap_or(SleepImageMode::DailyMantra)
}

pub fn resolve_sleep_bitmap_for_mode(sd: &SdStorage, mode: SleepImageMode) -> Option<SleepBitmapInfo> {
    match mode {
        SleepImageMode::DailyMantra => resolve_sleep_bitmap(sd),
        SleepImageMode::StaticBitmap => probe_sleep_bitmap(sd, ROOT_SLEEP_BITMAP),
        SleepImageMode::TextFallback | SleepImageMode::Disabled => None,
    }
}

""" + marker
    if marker not in text:
        raise SystemExit("sleep_bitmap.rs resolve_sleep_bitmap anchor not found")
    text = text.replace(marker, insert)

# Add parser before weekday parser.
if "pub fn parse_sleep_image_mode" not in text:
    marker = "pub fn parse_weekday_key(data: &[u8]) -> Option<WeekdayKey> {\n"
    insert = """
pub fn parse_sleep_image_mode(data: &[u8]) -> Option<SleepImageMode> {
    let mut lower = [0u8; 32];
    let n = data.len().min(lower.len());
    for i in 0..n {
        lower[i] = data[i].to_ascii_lowercase();
    }
    let s = &lower[..n];

    if contains(s, b"daily") || contains(s, b"mantra") {
        Some(SleepImageMode::DailyMantra)
    } else if contains(s, b"static") || contains(s, b"sleep") {
        Some(SleepImageMode::StaticBitmap)
    } else if contains(s, b"text") || contains(s, b"fallback") {
        Some(SleepImageMode::TextFallback)
    } else if contains(s, b"off") || contains(s, b"none") || contains(s, b"disabled") {
        Some(SleepImageMode::Disabled)
    } else {
        None
    }
}

""" + marker
    if marker not in text:
        raise SystemExit("sleep_bitmap.rs weekday parser anchor not found")
    text = text.replace(marker, insert)

# Add focused tests before final cfg(test) module end if a test module exists.
if "sleep_image_mode_parser_accepts_supported_modes" not in text:
    text = text.replace(
        "#[cfg(test)]\nmod tests {\n",
        "#[cfg(test)]\nmod tests {\n",
        1,
    )
    # If direct overlay did not include tests, append a test module.
    if "mod tests" not in text:
        text += """

#[cfg(test)]
mod tests {
    use super::{parse_sleep_image_mode, SleepImageMode};

    #[test]
    fn sleep_image_mode_parser_accepts_supported_modes() {
        assert_eq!(parse_sleep_image_mode(b"daily\n"), Some(SleepImageMode::DailyMantra));
        assert_eq!(parse_sleep_image_mode(b"static"), Some(SleepImageMode::StaticBitmap));
        assert_eq!(parse_sleep_image_mode(b"text"), Some(SleepImageMode::TextFallback));
        assert_eq!(parse_sleep_image_mode(b"off"), Some(SleepImageMode::Disabled));
    }
}
"""
    else:
        # Insert after the opening tests block.
        text = text.replace(
            "#[cfg(test)]\nmod tests {\n",
            "#[cfg(test)]\nmod tests {\n    use super::{parse_sleep_image_mode, SleepImageMode};\n\n    #[test]\n    fn sleep_image_mode_parser_accepts_supported_modes() {\n        assert_eq!(parse_sleep_image_mode(b\"daily\\n\"), Some(SleepImageMode::DailyMantra));\n        assert_eq!(parse_sleep_image_mode(b\"static\"), Some(SleepImageMode::StaticBitmap));\n        assert_eq!(parse_sleep_image_mode(b\"text\"), Some(SleepImageMode::TextFallback));\n        assert_eq!(parse_sleep_image_mode(b\"off\"), Some(SleepImageMode::Disabled));\n    }\n\n",
            1,
        )

sleep_bitmap.write_text(text)

scheduler = Path("vendor/pulp-os/kernel/src/kernel/scheduler.rs")
text = scheduler.read_text()

old = """    async fn render_daily_sleep_bitmap(&mut self) -> bool {
        use core::cell::Cell;
        use super::sleep_bitmap;

        let Some(info) = sleep_bitmap::resolve_sleep_bitmap(&self.sd) else {
            info!("sleep image: no valid bitmap found; using fallback text");
            return false;
        };

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
            info!("display: sleep bitmap rendered");
        } else {
            info!("display: sleep bitmap render failed");
        }
        ok.get()
    }

"""
new = """    async fn render_daily_sleep_bitmap(&mut self) -> bool {
        use core::cell::Cell;
        use embassy_time::Instant;
        use super::sleep_bitmap::{self, SleepImageMode};

        let mode_start = Instant::now();
        let mode = sleep_bitmap::read_sleep_image_mode(&self.sd);
        info!(
            "sleep image: mode={} mode_read_ms={}",
            mode.name(),
            mode_start.elapsed().as_millis()
        );

        match mode {
            SleepImageMode::Disabled => {
                info!("sleep image: disabled; skipping display update");
                return true;
            }
            SleepImageMode::TextFallback => {
                info!("sleep image: text fallback requested");
                return false;
            }
            SleepImageMode::DailyMantra | SleepImageMode::StaticBitmap => {}
        }

        let resolve_start = Instant::now();
        let Some(info) = sleep_bitmap::resolve_sleep_bitmap_for_mode(&self.sd, mode) else {
            info!(
                "sleep image: no valid bitmap found resolve_ms={}; using fallback text",
                resolve_start.elapsed().as_millis()
            );
            return false;
        };
        info!(
            "sleep image: bitmap resolved resolve_ms={}",
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
            info!(
                "display: sleep bitmap rendered render_ms={}",
                render_start.elapsed().as_millis()
            );
        } else {
            info!(
                "display: sleep bitmap render failed render_ms={}",
                render_start.elapsed().as_millis()
            );
        }
        ok.get()
    }

"""
if old not in text:
    if "mode_read_ms" in text:
        pass
    else:
        raise SystemExit("scheduler.rs daily sleep bitmap helper did not match expected direct-loader structure")
else:
    text = text.replace(old, new)

# Add timing around fallback text rendering if still plain.
old_fallback = """        if !sleep_bitmap_rendered {
            self.epd
                .full_refresh_async(self.strip, &mut self.delay, &|s: &mut StripBuffer| {
                    let style = MonoTextStyle::new(&FONT_9X18, BinaryColor::On);
                    let _ = Text::new("(sleep)", Point::new(210, 400), style).draw(s);
                })
                .await;
            info!("display: fallback sleep screen rendered");
        }
"""
new_fallback = """        if !sleep_bitmap_rendered {
            let fallback_start = embassy_time::Instant::now();
            self.epd
                .full_refresh_async(self.strip, &mut self.delay, &|s: &mut StripBuffer| {
                    let style = MonoTextStyle::new(&FONT_9X18, BinaryColor::On);
                    let _ = Text::new("(sleep)", Point::new(210, 400), style).draw(s);
                })
                .await;
            info!(
                "display: fallback sleep screen rendered render_ms={}",
                fallback_start.elapsed().as_millis()
            );
        }
"""
if old_fallback in text:
    text = text.replace(old_fallback, new_fallback)

scheduler.write_text(text)
PY

cargo fmt --all

echo "Applied sleep image timing and mode support. Backup: $backup_dir"
echo "Optional: scripts/write_sleep_image_mode.sh /Volumes/SD_CARD daily"
