# Reader/UI SD font runtime VFN repair

This repair aligns runtime font selection with the 8.3-safe SD font pack.

Scope:
- Accept `.VFN` physical file names while preserving the internal `VFNT` header contract.
- Load the selected Reader SD font file from `/VAACHAK/FONTS/MANIFEST.TXT`.
- Render Reader ASCII body glyphs from the loaded VFN font, falling back to built-in fonts for missing glyphs, styled spans, and non-Latin text.
- Load the first valid UI font from `/VAACHAK/FONTS/UIFONTS.TXT` and use it for non-inverted UI labels.
- Keep built-in UI labels as fallback and for selected/inverted rows.
- Do not bundle any font binaries.

Stable markers:

```text
reader-fonts=x4-reader-sd-font-selection-ok
ui-fonts=x4-ui-sd-font-runtime-ok
```
