#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OVERLAY="$ROOT/phase40h_host_title_map_txt_display_names_overlay"
DIR="$ROOT/vendor/pulp-os/kernel/src/kernel/dir_cache.rs"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"
RUNTIME_DIR="$ROOT/target-xteink-x4/src/vaachak_x4/runtime"

if [ ! -f "$DIR" ]; then
  echo "missing $DIR" >&2
  exit 1
fi

python3 - "$DIR" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()

if "PHASE40H_TITLE_MAP_FILE" not in text:
    anchor = "const MAX_DIR_ENTRIES: usize = 128;\n"
    if anchor not in text:
        raise SystemExit("dir_cache.rs: missing MAX_DIR_ENTRIES anchor")
    text = text.replace(
        anchor,
        anchor + 'const PHASE40H_TITLE_MAP_FILE: &str = "TITLEMAP.TSV";\n',
        1,
    )

if "phase40h_load_host_title_map" not in text:
    anchor = "    fn load_titles(&mut self, sd: &SdStorage) {\n"
    if anchor not in text:
        raise SystemExit("dir_cache.rs: missing load_titles anchor")
    helper = '''    fn phase40h_load_host_title_map(&mut self, sd: &SdStorage) {
        let mut buf = [0u8; 4096];
        let n = match read_file_start_in_dir(sd, X4_DIR, PHASE40H_TITLE_MAP_FILE, &mut buf) {
            Ok((_, n)) => n,
            Err(_) => return,
        };

        let data = &buf[..n];
        let mut start = 0usize;
        while start < data.len() {
            let end = data[start..]
                .iter()
                .position(|&b| b == b'\\n')
                .map(|p| start + p)
                .unwrap_or(data.len());
            let mut line = &data[start..end];
            if line.ends_with(b"\\r") {
                line = &line[..line.len() - 1];
            }
            if !line.is_empty() {
                self.apply_title_line(line);
            }
            start = end.saturating_add(1);
        }
    }

'''
    text = text.replace(anchor, helper + anchor, 1)

if "self.phase40h_load_host_title_map(sd);" not in text:
    old = "        self.load_titles(sd);\n"
    if old not in text:
        raise SystemExit("dir_cache.rs: missing self.load_titles(sd) call")
    text = text.replace(
        old,
        "        self.phase40h_load_host_title_map(sd);\n" + old,
        1,
    )

if "phase40h=x4-host-title-map-txt-display-names-ok" not in text:
    marker = "// phase40h=x4-host-title-map-txt-display-names-ok\n"
    if "// directory listing cache: sorted entries with title resolution\n" in text:
        text = text.replace(
            "// directory listing cache: sorted entries with title resolution\n",
            "// directory listing cache: sorted entries with title resolution\n" + marker,
            1,
        )
    else:
        text = marker + text

path.write_text(text)
print("patched host title map loader in", path)
PY

mkdir -p "$RUNTIME_DIR"
cp -v "$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/state_io_host_title_map_txt_display_names.rs" \
  "$RUNTIME_DIR/state_io_host_title_map_txt_display_names.rs"
cp -v "$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/state_io_host_title_map_txt_display_names_acceptance.rs" \
  "$RUNTIME_DIR/state_io_host_title_map_txt_display_names_acceptance.rs"

for export in \
  "pub mod state_io_host_title_map_txt_display_names;" \
  "pub mod state_io_host_title_map_txt_display_names_acceptance;"
do
  if ! grep -Fxq "$export" "$RUNTIME_MOD"; then
    printf '\n%s\n' "$export" >> "$RUNTIME_MOD"
  fi
done

"$OVERLAY/scripts/check_phase40h_host_title_map_txt_display_names.sh"

echo "phase40h=x4-host-title-map-txt-display-names-ok"
echo "phase40h-acceptance=x4-host-title-map-txt-display-names-report-ok"
