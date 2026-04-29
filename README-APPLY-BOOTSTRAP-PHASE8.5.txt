VaachakOS Bootstrap Phase 8.5 — X4 input poll-shape fix

This pack fixes Phase 8 input navigation after ADC values were confirmed to match the
working x4-reader-os-rs calibrated ladder values.

Root cause:
- Phase 8 sampled ADC input and then called X4Input::tick() in the same loop.
- tick() fed the current stable state back into the debounce state.
- During a new press, stable was still None while candidate was the newly decoded button.
- The immediate tick reset the candidate before it could survive the 15ms debounce window.

Fix:
- Use a 10ms input loop cadence, matching x4-reader-os-rs INPUT_TICK_FAST_MS.
- Remove the timer-only tick from this smoke phase.
- Keep calibrated ADC values direct, matching the proven x4-reader-os-rs thresholds.
- Keep Press/Release navigation only for this phase.

Apply:
  unzip -o /path/to/vaachak-os-bootstrap-phase8.5-input-poll-shape-fix.zip

Validate host:
  cargo fmt --all
  cargo check --workspace --all-targets
  cargo test --workspace --all-targets
  cargo clippy --workspace --all-targets -- -D warnings

Flash:
  . "$HOME/export-esp.sh"
  cargo run -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf

Expected logs after pressing buttons:
  phase8: input event #1 button=Down kind=Press
  phase8: redraw selected=1 item=Library
  phase8=x4-input-navigation-smoke-ok
