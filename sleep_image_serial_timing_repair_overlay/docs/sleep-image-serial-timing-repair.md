# Sleep Image Serial Timing Repair

This repair changes the active sleep-image timing messages from logger-only output to direct serial prints using `esp_println::println!`.

It instruments the active `render_daily_sleep_bitmap` path with:

- selected sleep image mode
- mode-file read time
- bitmap resolve time
- display render time
- total sleep-image path time

It also prints the final display deep-sleep and MCU deep-sleep handoff messages through direct serial output.
