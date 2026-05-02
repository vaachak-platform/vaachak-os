# Phase 35C-2 Acceptance

## Required Commands

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings

./scripts/check_phase35c2_button_runtime_guard.sh
./scripts/check_phase35c2_direct_app_manager_runtime.sh
```

## Acceptance Criteria

```text
- Active runtime uses direct Pulp AppManager, not a Vaachak AppLayer wrapper.
- Active runtime still creates InputDriver and spawns tasks::input_task.
- Active runtime still creates ButtonMapper through AppManager::new.
- Vaachak reader state facade remains present but inactive.
- vendor/pulp-os and vendor/smol-epub have no tracked edits.
- Normal boot marker remains vaachak=x4-runtime-ready.
```

## Hardware Expectation

After flashing, buttons should behave as they did before the failed wrapper:

```text
navigation works
select/open works
back works
reader page navigation works
menu/bookmark behavior remains unchanged
```
