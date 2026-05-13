# Reader Bionic Reading compile repair

This repair is intended to be applied after `reader_bionic_reading.zip`.

It fixes compile-only issues introduced by the first Bionic Reading overlay:

- Replaces `_ctx.request_full_redraw()` with `ctx.request_full_redraw()` in the quick-menu cycle path.
- Uses `PreparedTxtState::clear()` instead of a non-existent `reset()` method.
- Adds the new `bionic_mode` field to the `ReaderPreferences` initializer in Settings.

Behavior preserved:

- Bionic Reading quick-menu option remains Off / Light / Medium.
- Prepared TXT cache is bypassed/cleared when Bionic mode is enabled.
- Reader full redraw is requested when Bionic mode changes.
- Stable marker remains `reader-bionic=x4-reader-bionic-reading-ok`.
