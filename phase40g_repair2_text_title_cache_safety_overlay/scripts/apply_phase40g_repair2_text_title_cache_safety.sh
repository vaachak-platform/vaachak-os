#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OVERLAY="$ROOT/phase40g_repair2_text_title_cache_safety_overlay"
FILES="$ROOT/vendor/pulp-os/src/apps/files.rs"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"
RUNTIME_DIR="$ROOT/target-xteink-x4/src/vaachak_x4/runtime"

if [ ! -f "$FILES" ]; then
  echo "missing $FILES" >&2
  exit 1
fi

python3 - "$FILES" <<'PY'
from pathlib import Path
import re
import sys

path = Path(sys.argv[1])
text = path.read_text()

if "phase40g_repair_extract_text_title" not in text:
    raise SystemExit("phase40g repair scanner not found; apply Phase 40G repair first")

new_extract = '''fn phase40g_repair_extract_text_title(data: &[u8], out: &mut [u8]) -> usize {
    let mut start = 0usize;
    let mut lines_seen = 0usize;

    while start < data.len() && lines_seen < 80 {
        let end = data[start..]
            .iter()
            .position(|&b| b == b'\\n')
            .map(|p| start + p)
            .unwrap_or(data.len());

        let mut line = &data[start..end];
        if line.ends_with(b"\\r") {
            line = &line[..line.len() - 1];
        }

        let mut trimmed_start = 0usize;
        let mut trimmed_end = line.len();
        while trimmed_start < trimmed_end && phase40g_repair_is_ascii_space(line[trimmed_start]) {
            trimmed_start += 1;
        }
        while trimmed_end > trimmed_start && phase40g_repair_is_ascii_space(line[trimmed_end - 1]) {
            trimmed_end -= 1;
        }

        let trimmed = &line[trimmed_start..trimmed_end];
        let title_prefix = b"Title:";
        if trimmed.len() > title_prefix.len()
            && trimmed[..title_prefix.len()].eq_ignore_ascii_case(title_prefix)
        {
            let title_len = phase40g_repair_copy_text_title(trimmed, out);
            if title_len >= 3 {
                return title_len;
            }
        }

        start = end.saturating_add(1);
        lines_seen += 1;
    }

    0
}
'''

pattern = re.compile(
    r"fn phase40g_repair_extract_text_title\(data: &\[u8\], out: &mut \[u8\]\) -> usize \{.*?\n\}\n\nfn scan_one_text_title",
    re.S,
)
text, count = pattern.subn(new_extract + "\n\nfn scan_one_text_title", text)
if count != 1:
    raise SystemExit(f"failed to replace phase40g_repair_extract_text_title; replacements={count}")

if "phase40g-repair2=x4-text-title-cache-safety-ok" not in text:
    if "// phase40g-repair=x4-home-full-width-reader-titles-ok\n" in text:
        text = text.replace(
            "// phase40g-repair=x4-home-full-width-reader-titles-ok\n",
            "// phase40g-repair=x4-home-full-width-reader-titles-ok\n"
            "// phase40g-repair2=x4-text-title-cache-safety-ok\n",
            1,
        )
    else:
        text = "// phase40g-repair2=x4-text-title-cache-safety-ok\n" + text

path.write_text(text)
print("patched strict TXT title extraction in", path)
PY

mkdir -p "$RUNTIME_DIR"
cp -v "$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/state_io_text_title_cache_safety_repair.rs"   "$RUNTIME_DIR/state_io_text_title_cache_safety_repair.rs"
cp -v "$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/state_io_text_title_cache_safety_repair_acceptance.rs"   "$RUNTIME_DIR/state_io_text_title_cache_safety_repair_acceptance.rs"

for export in   "pub mod state_io_text_title_cache_safety_repair;"   "pub mod state_io_text_title_cache_safety_repair_acceptance;"
do
  if ! grep -Fxq "$export" "$RUNTIME_MOD"; then
    printf '\n%s\n' "$export" >> "$RUNTIME_MOD"
  fi
done

"$OVERLAY/scripts/check_phase40g_repair2_text_title_cache_safety.sh"

echo "phase40g-repair2=x4-text-title-cache-safety-ok"
echo "phase40g-repair2-acceptance=x4-text-title-cache-safety-report-ok"
