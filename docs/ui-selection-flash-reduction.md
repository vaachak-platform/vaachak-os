# UI Selection Flash Reduction

This deliverable reduces visible flashing during dashboard, category, Files/Library, and bookmark-list navigation.

## What changes

- Dashboard/category selection moves from high-contrast inverted card fills to a lower-flash rail/outline style.
- Category item rows move from inverted block selection to a small left rail.
- Files/Library rows move from inverted row text/background to a left rail.
- Bookmark rows move from inverted row text/background to a left rail.
- Adjacent Files/Library row movement no longer redraws the top status counter on every button press, because the single-region dirty model unions that top status region with the selected row and causes a much larger partial refresh.
- Adjacent bookmark row movement uses the same rule. Status is still refreshed on wraps, page jumps, and full/list redraws.

## What does not change

- SSD1677 refresh driver behavior is unchanged.
- Kernel refresh scheduler behavior is unchanged.
- EPUB loading policy is unchanged.
- Reader page-turn behavior is unchanged.
- Ghost clearing remains enabled.
- Full refresh on screen entry/exit remains enabled.

## Expected behavior after flashing

- Moving between Home/category cards should still show a partial update, but the high-contrast black card fill should no longer flash.
- Moving up/down in Files/Library should update only the old and new row area, without pulling the top status counter into every adjacent-row refresh.
- Opening a book and page turns should behave as before.

## Validation

Run:

```bash
cargo fmt --all
./scripts/validate_ui_selection_flash_reduction.sh
cargo build --release
```

Expected marker:

```text
ui-selection-flash-reduction-ok
```
