# Phase 42A — App Shell Routing + Settings

Phase 42A keeps the active Biscuit Home dashboard and wires the Settings card to
the active `SettingsApp`.

Implemented surface:

- Home card order remains Reader / Library / Bookmarks / Settings / Sync / Upload.
- Settings opens a real scrollable Settings screen.
- Settings shows Reader, Display, Storage, Device, and About sections.
- Phase 42A Settings choices are local/in-memory UI state.
- Existing `SystemSettings` load/save hooks remain available for runtime code.
- Sync is a safe placeholder; Upload uses the existing Upload route.

Frozen surfaces:

- Files/Reader title-cache behavior unchanged.
- Reader pagination and restore unchanged.
- Display geometry, input thresholds, SD/FAT/write lane unchanged.

Marker:

```text
phase42a=x4-app-shell-routing-settings-implementation-ok
```
