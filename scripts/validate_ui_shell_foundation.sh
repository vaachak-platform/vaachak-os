#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

SHELL_RS="target-xteink-x4/src/vaachak_x4/ui/page_shell.rs"
UI_MOD="target-xteink-x4/src/vaachak_x4/ui.rs"
DOC="docs/ui/ui-shell-foundation.md"

for path in "$SHELL_RS" "$UI_MOD" "$DOC"; do
  if [[ ! -f "$path" ]]; then
    echo "missing required UI shell file: $path" >&2
    exit 1
  fi
done

rg -n 'pub mod page_shell;' "$UI_MOD" >/dev/null
rg -n 'UI_SHELL_FOUNDATION_MARKER|ui-shell-foundation-vaachak-ok' "$SHELL_RS" >/dev/null
rg -n 'UiShellLayout|UiShellTokens|UiShellPageContract|UiShellRowLayout|UiShellTabMetrics|UiShellFooterMetrics' "$SHELL_RS" >/dev/null
rg -n 'DEFAULT_SETTINGS_TABS|DEFAULT_READER_TABS|DEFAULT_NETWORK_TABS|DEFAULT_FOOTER_LABELS' "$SHELL_RS" >/dev/null
rg -n 'CHANGES_HOME_DASHBOARD_RENDERING: bool = false' "$SHELL_RS" >/dev/null
rg -n 'CHANGES_READER_PAGINATION: bool = false' "$SHELL_RS" >/dev/null
rg -n 'CHANGES_STORAGE_BEHAVIOR: bool = false' "$SHELL_RS" >/dev/null
rg -n 'CHANGES_WIFI_BEHAVIOR: bool = false' "$SHELL_RS" >/dev/null
rg -n 'TOUCHES_VENDOR_PULP_OS: bool = false' "$SHELL_RS" >/dev/null
rg -n 'TOUCHES_DISPLAY_REFRESH_SCHEDULER: bool = false' "$SHELL_RS" >/dev/null
rg -n 'UI Shell Foundation|ui-shell-foundation-vaachak-ok' "$DOC" >/dev/null

if git diff --name-only -- vendor/pulp-os 2>/dev/null | rg . >/dev/null; then
  echo "unexpected vendor/pulp-os modification detected" >&2
  exit 1
fi

if rg -n 'ui shell|page shell|CrossInk' vendor/pulp-os >/dev/null 2>&1; then
  echo "unexpected UI shell strings inside vendor/pulp-os" >&2
  exit 1
fi

echo "marker=ui-shell-foundation-vaachak-ok"
