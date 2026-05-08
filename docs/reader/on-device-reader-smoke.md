# On-device Reader Smoke and Diagnostic Cleanup

This checklist validates the reader behavior that cannot be proven by static builds alone.

## Purpose

The reader build/static gate confirms that the firmware compiles, the prepared-cache status text is still present, the Wi-Fi Transfer two-tab UI is still present, and temporary debug reader chrome is absent.

The on-device smoke confirms the same behavior on the Xteink X4 with the SD card, display, buttons, and persisted reader state involved.

## Required SD card content

Prepare the SD card with these files/folders before flashing:

```text
/YEARLY_H.TXT
/FCACHE/<BOOKID for YEARLY_H.TXT>/META.TXT
/FCACHE/<BOOKID for YEARLY_H.TXT>/FONTS.IDX
/FCACHE/<BOOKID for YEARLY_H.TXT>/PAGES.IDX
/FCACHE/<BOOKID for YEARLY_H.TXT>/<font and page files>

/<mixed EPUB smoke book>
/FCACHE/<BOOKID for mixed EPUB>/<prepared cache files>
```

Use Wi-Fi Transfer -> Chunked Resume for large `/FCACHE/<BOOKID>` folders.

## Expected reader chrome

Successful prepared-cache reads must show:

```text
Prep Pg ...
```

Prepared-cache failures must show:

```text
Read cache:<BOOKID> err:<CODE>
```

`err:OPEN`, generic temporary debug-only text, or raw cache-open debug strings should not appear during successful prepared-cache reads.

## Manual smoke checklist

### YEARLY_H.TXT prepared cache

1. Open `YEARLY_H.TXT` from Reader/Library.
2. Confirm the status area shows `Prep Pg ...`.
3. Confirm the body renders with the prepared glyph output.
4. Confirm no `err:OPEN` or temporary debug-only cache text appears.

### YEARLY_H.TXT progress restore

1. Page forward at least two pages.
2. Return using Back.
3. Reopen `YEARLY_H.TXT`.
4. Confirm progress restores near the previous page.
5. Confirm the status area still shows `Prep Pg ...`.

### Mixed EPUB prepared cache

1. Open the mixed EPUB smoke book.
2. Confirm the status area shows `Prep Pg ...`.
3. Page forward and back.
4. Return using Back.
5. Reopen the EPUB.
6. Confirm progress restores and the prepared cache still opens.

### Failure diagnostic

1. Temporarily rename one prepared-cache folder on the SD card.
2. Open the matching book.
3. Confirm the reader does not freeze.
4. Confirm the status area shows `Read cache:<BOOKID> err:<CODE>`.
5. Restore the folder name.
6. Reopen the book.
7. Confirm the status returns to clean `Prep Pg ...`.

### Reader settings

1. Change Reader font/theme/progress visibility in Settings or Reader quick menu.
2. Reopen the prepared TXT and EPUB books.
3. Confirm the settings still apply.
4. Confirm progress/bookmark state remains intact.

### Wi-Fi Transfer large FCACHE upload

1. Open Network -> Wi-Fi Transfer.
2. Confirm both tabs are visible:
   - Original Transfer
   - Chunked Resume
3. Upload or resume a large `/FCACHE/<BOOKID>` folder.
4. Reopen the matching book and confirm `Prep Pg ...`.

## Local validation command

Run before flashing:

```bash
./scripts/validate_on_device_reader_smoke.sh
```

## Reader header diagnostic cleanup

Expected behavior:

- Regular TXT and EPUB files without prepared cache folders use the normal reader path silently.
- Header/status chrome never contains `Read cache:<BOOKID> err:<CODE>`.
- Prepared-cache success keeps the compact `Prep` status.
- True prepared-cache failures may show `Read cache:<BOOKID> err:<CODE>` as a short body-area notice, not in the header.
- Wi-Fi Transfer keeps both `Original Transfer` and `Chunked Resume` tabs for FCACHE uploads.

