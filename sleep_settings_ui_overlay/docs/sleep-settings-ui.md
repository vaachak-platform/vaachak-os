# Sleep Settings UI

Adds a Settings row named **Sleep image** under the Device section.

Supported values:

- Daily
- Fast Daily
- Static
- Cached
- Text
- No Redraw

The selected value is persisted to the SD card root file:

```text
/SLPMODE.TXT
```

The sleep renderer already reads this file before sleep. This UI makes the same file editable on-device while keeping the existing Mac-side helper scripts.

The root value written is one of:

```text
daily
fast-daily
static
cached
text
no-redraw
```
