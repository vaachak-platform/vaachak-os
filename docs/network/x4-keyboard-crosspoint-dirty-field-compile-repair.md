# X4 Keyboard CrossPoint Dirty Field Compile Repair

This repair fixes a compile-only mismatch caused by the CrossPoint-style keyboard session repair referencing `wifi_editor_dirty` on a tree where the field was not added to `HomeApp`.

It adds the missing field and initializer idempotently. It does not change Wi-Fi scan, Wi-Fi Transfer, NTP/date-time sync, `SETTINGS.TXT` format, display driver, SD, SPI, EPUB, or reader behavior.

Validation marker:

```text
x4-keyboard-crosspoint-dirty-field-compile-repair-ok
```
