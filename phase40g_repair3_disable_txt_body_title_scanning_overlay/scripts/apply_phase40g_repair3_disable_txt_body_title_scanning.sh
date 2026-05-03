#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OVERLAY="$ROOT/phase40g_repair3_disable_txt_body_title_scanning_overlay"
DIR="$ROOT/vendor/pulp-os/kernel/src/kernel/dir_cache.rs"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"
RUNTIME_DIR="$ROOT/target-xteink-x4/src/vaachak_x4/runtime"

if [ ! -f "$DIR" ]; then
  echo "missing $DIR" >&2
  exit 1
fi

python3 - "$DIR" <<'PY'
from pathlib import Path
import re
import sys

path = Path(sys.argv[1])
text = path.read_text()

if "next_untitled_reader_title" not in text:
    raise SystemExit("next_untitled_reader_title not found; apply Phase 40G repair first")

pattern = re.compile(
    r'''\n\s*if phase40g_repair_is_text_title_name\(name\) \{\n\s*return Some\(\(i, e\.name, e\.name_len, PHASE40G_REPAIR_TITLE_KIND_TEXT\)\);\n\s*\}\n''',
    re.M,
)

replacement = '''
            // Phase 40G Repair 3:
            // TXT/MD body-title scanning is disabled. It was unsafe because
            // license/body lines can be cached as display titles. A future
            // FAT LFN/title-map lane should provide proper TXT display names.
            if phase40g_repair_is_text_title_name(name) {
                continue;
            }
'''

text, count = pattern.subn("\n" + replacement, text)
if count != 1:
    raise SystemExit(f"failed to disable TXT title candidate branch; replacements={count}")

if "phase40g-repair3=x4-disable-txt-body-title-scanning-ok" not in text:
    marker = "// phase40g-repair3=x4-disable-txt-body-title-scanning-ok\n"
    if "// phase40g-repair=x4-home-full-width-reader-titles-ok\n" in text:
        text = text.replace(
            "// phase40g-repair=x4-home-full-width-reader-titles-ok\n",
            "// phase40g-repair=x4-home-full-width-reader-titles-ok\n" + marker,
            1,
        )
    else:
        text = marker + text

path.write_text(text)
print("disabled TXT/MD body-title candidate branch in", path)
PY

mkdir -p "$RUNTIME_DIR"
cp -v "$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/state_io_disable_txt_body_title_scanning_repair.rs"   "$RUNTIME_DIR/state_io_disable_txt_body_title_scanning_repair.rs"
cp -v "$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/state_io_disable_txt_body_title_scanning_repair_acceptance.rs"   "$RUNTIME_DIR/state_io_disable_txt_body_title_scanning_repair_acceptance.rs"

for export in   "pub mod state_io_disable_txt_body_title_scanning_repair;"   "pub mod state_io_disable_txt_body_title_scanning_repair_acceptance;"
do
  if ! grep -Fxq "$export" "$RUNTIME_MOD"; then
    printf '\n%s\n' "$export" >> "$RUNTIME_MOD"
  fi
done

"$OVERLAY/scripts/check_phase40g_repair3_disable_txt_body_title_scanning.sh"

echo "phase40g-repair3=x4-disable-txt-body-title-scanning-ok"
echo "phase40g-repair3-acceptance=x4-disable-txt-body-title-scanning-report-ok"
