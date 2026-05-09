# On-device Reader Smoke

This checklist validates behavior that static builds cannot prove.

## Required SD card content

Use a representative SD card with:

```text
/YEARLY_H.TXT
/FCACHE/<BOOKID for YEARLY_H.TXT>/META.TXT
/FCACHE/<BOOKID for YEARLY_H.TXT>/FONTS.IDX
/FCACHE/<BOOKID for YEARLY_H.TXT>/PAGES.IDX
/<mixed EPUB smoke book>
/FCACHE/<BOOKID for mixed EPUB>/<prepared cache files>
```

Use Wi-Fi Transfer / Chunked Resume for large `/FCACHE/<BOOKID>` folders if needed.

## Prepared TXT smoke

1. Open `YEARLY_H.TXT` from Files/Library.
2. Confirm prepared-page status appears on successful cache open.
3. Confirm body renders with prepared glyph output.
4. Confirm no temporary `err:OPEN` or raw debug-only cache text appears on success.

## Progress restore

1. Page forward at least two pages.
2. Use Back to return.
3. Reopen the book.
4. Confirm progress restores near the previous page.

## Mixed EPUB smoke

1. Open the mixed EPUB smoke book.
2. Confirm prepared-cache status if the cache exists.
3. Page forward and back.
4. Return using Back.
5. Reopen and confirm progress restore.

## Failure diagnostic

1. Temporarily rename a prepared-cache folder.
2. Open the matching book.
3. Confirm the reader does not freeze.
4. Confirm a compact cache failure diagnostic appears.
5. Restore the folder and reopen successfully.

## Settings and transfer

1. Change Reader or device settings.
2. Reopen prepared TXT and EPUB smoke files.
3. Confirm settings still apply.
4. Open Wi-Fi Transfer.
5. Confirm normal and large-folder transfer paths remain understandable.

## Pass criteria

- no reboot/crash in normal reading flow.
- Back always returns to a stable Library/Home state.
- progress restore works.
- cache diagnostics appear only on failure.
- display/input/storage behavior remains stable.
