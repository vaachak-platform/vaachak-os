#!/usr/bin/env bash
set -euo pipefail

repo="${1:-.}"
cd "$repo"

if [ ! -f Cargo.toml ]; then
  echo "error: run from the vaachak-os repository root or pass the repository path" >&2
  exit 1
fi

stamp="$(date +%Y%m%d-%H%M%S)"
backup_dir=".vaachak_pre_github_backups/daily_mantra_direct_sleep_loader_${stamp}"
mkdir -p "$backup_dir"

backup_if_exists() {
  local path="$1"
  if [ -e "$path" ]; then
    mkdir -p "$backup_dir/$(dirname "$path")"
    cp -R "$path" "$backup_dir/$path"
  fi
}

backup_if_exists vendor/pulp-os/kernel/src/kernel/mod.rs
backup_if_exists vendor/pulp-os/kernel/src/kernel/scheduler.rs
backup_if_exists vendor/pulp-os/kernel/src/kernel/sleep_bitmap.rs
backup_if_exists vendor/pulp-os/kernel/src/drivers/storage.rs
backup_if_exists scripts/write_daily_mantra_today_file.sh
backup_if_exists scripts/verify_daily_mantra_direct_sleep_files.sh

cp -f daily_mantra_direct_sleep_loader_overlay/files/vendor/pulp-os/kernel/src/kernel/sleep_bitmap.rs \
  vendor/pulp-os/kernel/src/kernel/sleep_bitmap.rs
cp -f daily_mantra_direct_sleep_loader_overlay/files/scripts/write_daily_mantra_today_file.sh \
  scripts/write_daily_mantra_today_file.sh
cp -f daily_mantra_direct_sleep_loader_overlay/files/scripts/verify_daily_mantra_direct_sleep_files.sh \
  scripts/verify_daily_mantra_direct_sleep_files.sh
chmod +x scripts/write_daily_mantra_today_file.sh scripts/verify_daily_mantra_direct_sleep_files.sh

python3 - <<'PY'
from pathlib import Path

mod = Path("vendor/pulp-os/kernel/src/kernel/mod.rs")
text = mod.read_text()
if "pub mod sleep_bitmap;" not in text:
    text = text.replace("pub mod scheduler;\n", "pub mod scheduler;\npub mod sleep_bitmap;\n")
mod.write_text(text)

storage = Path("vendor/pulp-os/kernel/src/drivers/storage.rs")
text = storage.read_text()
if "pub fn read_file_chunk_in_subdir" not in text:
    anchor = """pub fn read_file_start_in_dir(
    sd: &SdStorage,
    dir: &str,
    name: &str,
    buf: &mut [u8],
) -> crate::error::Result<(u32, usize)> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        in_dir!(inner, dir, |dir_h| op_read_start!(inner, dir_h, name, buf))
    })
}
"""
    insert = anchor + """
pub fn read_file_chunk_in_subdir(
    sd: &SdStorage,
    dir: &str,
    subdir: &str,
    name: &str,
    offset: u32,
    buf: &mut [u8],
) -> crate::error::Result<usize> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        in_subdir!(inner, dir, subdir, |dir_h| op_read_chunk!(
            inner, dir_h, name, offset, buf
        ))
    })
}

pub fn read_file_start_in_subdir(
    sd: &SdStorage,
    dir: &str,
    subdir: &str,
    name: &str,
    buf: &mut [u8],
) -> crate::error::Result<(u32, usize)> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        in_subdir!(inner, dir, subdir, |dir_h| op_read_start!(
            inner, dir_h, name, buf
        ))
    })
}
"""
    if anchor not in text:
        raise SystemExit("storage.rs anchor not found for directory read helpers")
    text = text.replace(anchor, insert)
storage.write_text(text)

scheduler = Path("vendor/pulp-os/kernel/src/kernel/scheduler.rs")
text = scheduler.read_text()
old = """        self.sd_card_sleep();

        self.epd
            .full_refresh_async(self.strip, &mut self.delay, &|s: &mut StripBuffer| {
                let style = MonoTextStyle::new(&FONT_9X18, BinaryColor::On);
                let _ = Text::new(\"(sleep)\", Point::new(210, 400), style).draw(s);
            })
            .await;
        info!(\"display: sleep screen rendered\");

        self.epd.enter_deep_sleep();
"""
new = """        let sleep_bitmap_rendered = self.render_daily_sleep_bitmap().await;
        if !sleep_bitmap_rendered {
            self.epd
                .full_refresh_async(self.strip, &mut self.delay, &|s: &mut StripBuffer| {
                    let style = MonoTextStyle::new(&FONT_9X18, BinaryColor::On);
                    let _ = Text::new(\"(sleep)\", Point::new(210, 400), style).draw(s);
                })
                .await;
            info!(\"display: fallback sleep screen rendered\");
        }

        self.sd_card_sleep();

        self.epd.enter_deep_sleep();
"""
if old not in text:
    raise SystemExit("scheduler.rs sleep render block not found")
text = text.replace(old, new)

helper_anchor = """    // send cmd0 to put sd card into idle/sleep state;
"""
helper = """    async fn render_daily_sleep_bitmap(&mut self) -> bool {
        use core::cell::Cell;
        use super::sleep_bitmap;

        let Some(info) = sleep_bitmap::resolve_sleep_bitmap(&self.sd) else {
            info!(\"sleep image: no valid bitmap found; using fallback text\");
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
            info!(\"display: sleep bitmap rendered\");
        } else {
            info!(\"display: sleep bitmap render failed\");
        }
        ok.get()
    }

"""
if "async fn render_daily_sleep_bitmap" not in text:
    if helper_anchor not in text:
        raise SystemExit("scheduler.rs helper anchor not found")
    text = text.replace(helper_anchor, helper + helper_anchor)
scheduler.write_text(text)
PY

cargo fmt --all

echo "Applied Daily Mantra direct sleep bitmap loader. Backup: $backup_dir"
