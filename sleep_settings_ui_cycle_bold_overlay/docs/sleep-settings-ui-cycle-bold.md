# Sleep Settings UI Cycle + Bold Sections

This repair makes the Settings `Sleep image` row editable and renders section names using the bold bitmap font.

The previous Sleep Image Mode settings overlay added the row and value formatter, but the row could remain non-editable if the cycle handler was not patched after `DeviceSleepImageMode` already existed.

Expected Settings behavior:

- Device > Sleep image can cycle through:
  - Daily
  - Fast Daily
  - Static
  - Cached
  - Text
  - No Redraw
- Section labels such as Reader, Display, Storage, Device, About render with bold font.
- Mode persistence remains `/SLPMODE.TXT`.
