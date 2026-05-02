Phase 36A — Active Input Semantic Mapping Takeover

This overlay installs replaceable files for the Vaachak X4 target. It moves active
ButtonMapper construction behind a Vaachak-owned semantic mapper adapter without
moving ADC sampling, debounce, repeat handling, input_task, AppManager internals,
or vendor code.

Apply from the vaachak-os repo root:

  unzip phase36a_active_input_semantic_mapping_overlay.zip
  chmod +x phase36a_active_input_semantic_mapping_overlay/scripts/*.sh
  ./phase36a_active_input_semantic_mapping_overlay/scripts/apply_phase36a_active_input_semantic_mapping.sh

Validate:

  . "$HOME/export-esp.sh"
  cargo fmt --all
  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings
  ./scripts/check_imported_reader_runtime_sync_phase36a.sh
  ./scripts/check_phase36a_active_input_semantic_mapping.sh
  ./scripts/check_phase36a_no_input_hardware_regression.sh

Flash:

  cargo run -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf

Expected boot marker remains:

  vaachak=x4-runtime-ready
