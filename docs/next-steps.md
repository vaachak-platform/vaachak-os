# Next Steps After HAL Trait Refinement

1. map each refined trait method to the current X4 implementation file and function
2. decide whether battery sampling stays internally shared with input on X4 or becomes a small shared ADC helper
3. add a real X4 bootstrap path in `target-xteink-x4` that preserves the current proven init order:
   - low-speed SD probe
   - display init
   - SPI speed-up
   - mounted storage handoff
4. keep `Kernel` / `AppManager` and reader behavior in `x4-reader-os-rs` until the first HAL extraction compiles cleanly
5. only then start moving the first real X4 slice into `vaachak-os`
