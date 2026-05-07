Prepared Cache TOO_LARGE Limits Fix

What this fixes:
- Reader header shows err:TOO_LARGE for YEARLY_H.TXT / 15D1296A.
- PAGES.IDX for real books can exceed the old 512-byte smoke-test limit.
- Real prepared pages can exceed the old 4KB page limit.
- Real books can exceed the old 8-page/160-glyph smoke-test caps.

Apply:
  unzip -o prepared_cache_too_large_limits_overlay.zip
  ./prepared_cache_too_large_limits_overlay/scripts/apply_prepared_cache_too_large_limits.sh .

Validate:
  cargo fmt --all --check
  cargo check --workspace --target riscv32imc-unknown-none-elf
  cargo clippy --workspace --target riscv32imc-unknown-none-elf -- -D warnings
  cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
  git diff --check

Expected after flash:
- YEARLY_H.TXT should change from err:TOO_LARGE to Prep Pg ... if cache files are complete.
- If it changes to a different err:<CODE>, use that code for the next targeted fix.
