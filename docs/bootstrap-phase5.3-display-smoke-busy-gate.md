# Bootstrap Phase 5.3 — Display Smoke BUSY Gate

The Phase 5.2 log showed `display init complete busy=true` immediately followed by a full-frame RAM write attempt. On SSD1677, RAM writes should not begin while BUSY is active.

This phase adds a conservative pre-write BUSY wait in the target smoke binary. It does not change the display command sequence.

If this shows `pre-write busy=true waited_ms=5000`, the next extraction step is to port the proven `x4-reader-os-rs` SPI device/display path rather than tune commands blindly.
