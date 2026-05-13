# UI Selection Flash Reduction

The UI selection path reduces visible flashing during dashboard, category, Files/Library, and bookmark-list navigation.

## Current behavior

- Dashboard/category selection uses a lower-flash rail/outline style instead of high-contrast inverted card fills.
- Category item rows use a small left rail.
- Files/Library rows use a left rail.
- Bookmark rows use a left rail.
- Adjacent Files/Library row movement avoids redrawing the top status counter on every button press when possible.
- Adjacent bookmark row movement uses the same rule. Status is still refreshed on wraps, page jumps, and full/list redraws.

## What should stay stable

- SSD1677 refresh driver behavior.
- Kernel refresh scheduler behavior.
- EPUB loading policy.
- Reader page-turn behavior.
- Ghost clearing where configured.
- Full refresh on screen entry/exit where required.

## Validation

```bash
cargo fmt --all
./scripts/check_repo_hygiene.sh
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
```

After flashing, moving between Home/category cards should still show a partial update, but the high-contrast black card fill should no longer flash. Files/Library movement should update the old and new row area without dragging the top status counter into every adjacent-row refresh.
