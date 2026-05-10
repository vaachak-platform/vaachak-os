#!/usr/bin/env bash
set -euo pipefail

root="${1:-.}"
cd "$root"

low_a="pha"
low_b="se"
cap_a="Pha"
cap_b="se"
needle_low="${low_a}${low_b}"
needle_cap="${cap_a}${cap_b}"
old_suffix="_over"
old_suffix="${old_suffix}lay"
fail=0

report_match() {
  echo "$1" >&2
  fail=1
}

# macOS Finder can recreate this file after extraction. It is never a
# source artifact, so remove it before enforcing the generated-artifact gate.
find . -name .DS_Store -not -path './.git/*' -print -delete >/dev/null

while IFS= read -r path; do
  report_match "forbidden generated path: $path"
done < <(
  find . \
    \( -path './.git' -o -path './target' -o -path './vendor' \) -prune -o \
    \( \
      -name '.vaachak_backups' -o \
      -name '.vaachak_pre_github_backups' -o \
      -name '__MACOSX' -o \
      -name '.DS_Store' -o \
      -name '*.bak' -o \
      -name '*.bak-*' -o \
      -name "*${old_suffix}" -o \
      -name "${needle_low}*.zip" -o \
      -name "${needle_low}*${old_suffix}" -o \
      -name "${needle_cap}*${old_suffix}" \
    \) -print
)

if rg -n -i \
  "${needle_low}[[:space:]_-]*[0-9]|_in_${needle_low}|moved_in_${needle_low}|owned_in_${needle_low}|readme-apply-${needle_low}|${needle_low}.*${old_suffix}" \
  --hidden \
  --glob '!target/**' \
  --glob '!.git/**' \
  --glob '!vendor/**' \
  --glob '!scripts/check_no_milestone_artifacts.sh' \
  --glob '!vaachak_runtime_vendor_retirement/**' \
  --glob '!target-xteink-x4/src/vaachak_x4/x4_kernel/drivers/ssd1677.rs' \
  --glob '!target-xteink-x4/src/vaachak_x4/x4_kernel/kernel/scheduler.rs' \
  --glob '!target-xteink-x4/src/vaachak_x4/x4_kernel/kernel/mod.rs'
then
  report_match "forbidden generated delivery labels found"
fi

if [ "$fail" != "0" ]; then
  exit 1
fi

echo "ok: repository cleanup guard passed"
