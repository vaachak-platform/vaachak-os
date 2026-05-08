# Current System Apps Closure

This gate closes the system apps that already exist in the X4 runtime instead of adding another architecture layer.

## Wi-Fi Transfer

Expected behavior:

- `Original Transfer` remains the SD-card file manager for normal uploads, downloads, rename, delete, and folder creation.
- `Chunked Resume` remains available for large prepared-cache uploads such as `/FCACHE/15D1296A`.
- Browser and device UI never display the saved Wi-Fi password.
- Upload failures keep a visible status and tell the user to retry or use chunked resume.
- Chunked resume skips complete files and resumes partial folders after interruption.

## Date & Time

Expected behavior:

- Back can cancel the network sync path during Wi-Fi start, connect, DHCP, DNS, send, and receive waits.
- The Date & Time screen reports `Live`, `Cached`, or `Unsynced`.
- A failed retry preserves previously cached time and records the failure reason.
- Select can safely retry from the Date & Time screen.

## Settings

Expected behavior:

- Reader settings flow both ways: Settings to Reader and Reader quick actions back to Settings.
- Prepared profile and fallback policy are propagated to the live Reader, not only saved to settings.
- Sleep image mode persists through `SLPMODE.TXT`.
- Settings shows the current battery reading using the same battery percentage helper used by the Home header.

## Validation

Run:

```bash
./scripts/validate_system_apps_closure.sh
```

After flashing, complete the on-device checklist printed by the script.
