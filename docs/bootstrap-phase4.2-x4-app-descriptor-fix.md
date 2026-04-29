# VaachakOS Bootstrap Phase 4.2 — X4 App Descriptor Fix

Phase 4.1 reached `espflash`, but `espflash` refused the ELF because the ESP-IDF app descriptor was not detected.

This fix keeps Phase 4 intentionally small while making the descriptor path more robust:

- the embedded path is now gated by `target_arch = "riscv32"` instead of the stricter `target_arch + target_os` check
- `esp_bootloader_esp_idf::esp_app_desc!()` remains in the target binary
- ESP crate versions are pinned to the known-working `x4-reader-os-rs` ESP crate line
- `linkall.x` remains configured for the ESP32-C3 target

Still out of scope: display driver migration, SD/FAT migration, Home/Files/Reader runtime migration, real input ADC sampling, and real battery ADC sampling.
