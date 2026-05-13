# UI Shell Foundation

The UI shell foundation defines a Vaachak-owned internal page layout contract for larger, tabbed X4 screens. It is intended for Settings, Reader Library, Files, Fonts, Network, Tools, Games lists, and System pages.

The accepted Home/dashboard direction remains Biscuit-inspired category navigation. The shell foundation is for internal pages after a user enters a category or app.

## Goals

- Keep the Biscuit-style Home dashboard unchanged.
- Provide a shared tabbed page shell for internal screens.
- Support CrossInk-style readability: larger rows, tabs, section headers, right-aligned values, selected value pills, scrollbars, and bottom soft-key hints.
- Keep all behavior-only logic unchanged in this foundation slice.
- Keep reader pagination, display refresh scheduling, storage behavior, Wi-Fi behavior, settings persistence, and button mapping unchanged.
- Keep `vendor/pulp-os` unchanged.

## Source files

```text
 target-xteink-x4/src/vaachak_x4/ui/page_shell.rs
 target-xteink-x4/src/vaachak_x4/ui.rs
 scripts/validate_ui_shell_foundation.sh
 docs/ui/ui-shell-foundation.md
```

## Shell structure

```text
┌────────────────────────────────────┐
│ Page Title                    42%  │
├────────────────────────────────────┤
│ Display | Reader | Controls | Sys  │
├────────────────────────────────────┤
│ SECTION HEADER                     │
│ Selected row              Value    │
│ Normal row                Value    │
│ Normal row                Value    │
│                         scrollbar  │
├────────────────────────────────────┤
│ Back | Select | Up | Down           │
└────────────────────────────────────┘
```

The descriptor layer includes:

- `UiShellLayout`
- `UiShellTokens`
- `UiShellPageContract`
- `UiShellRowLayout`
- `UiShellTabMetrics`
- `UiShellFooterMetrics`
- default tab labels for Settings, Reader, and Network pages

## Current marker

```text
ui-shell-foundation-vaachak-ok
```

## Migration plan

Use this order for screen migration:

1. Settings shell adoption.
2. Files / Library list adoption.
3. Font browser adoption.
4. Network / Wi-Fi / Transfer / Time pages.
5. Tools and Games catalog pages.

Do not migrate Reader page rendering as part of this shell foundation. Reader page turns, pagination, EPUB/TXT layout, Bionic Reading, Guide Dots, and sunlight-fading mitigation remain on the existing reader path.

## Validation

Run:

```bash
./scripts/validate_ui_shell_foundation.sh
cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
```

Expected validation marker:

```text
marker=ui-shell-foundation-vaachak-ok
```
