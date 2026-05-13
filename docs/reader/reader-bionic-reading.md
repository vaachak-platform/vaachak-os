# Reader Bionic Reading

Adds a Reader quick-menu option for Bionic Reading.

Modes:
- Off
- Light
- Medium

Implementation notes:
- Uses a fake-bold overlay by drawing Latin fixation glyphs a second time with a one-pixel horizontal offset.
- Does not alter text bytes, search text, bookmark offsets, or progress records.
- Applies to the dynamic TXT/EPUB reader body path.
- When Bionic Reading is enabled, prepared TXT cache rendering is bypassed so the dynamic renderer can apply fixation glyphs.
- Existing bold, italic, heading, image, and non-Latin glyph handling remain unchanged.

Stable marker:

```text
reader-bionic=x4-reader-bionic-reading-ok
```
