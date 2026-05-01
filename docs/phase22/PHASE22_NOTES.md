# Phase 22 Notes

Phase 22 intentionally does not modify `vendor/pulp-os` or `vendor/smol-epub`.

The input boundary is a typed ownership contract. It exists so future phases can move one behavior at a time from imported Pulp runtime into Vaachak-owned runtime modules while keeping compile, clippy, and device acceptance stable.

Recommended next extraction order after Phase 22:

```text
1. Input event naming/role normalization only
2. Button footer/action mapping tests
3. ADC sample wrapper extraction
4. Debounce/repeat behavior extraction
5. Reader/menu event routing extraction
```

Do not move ADC reads and event routing in the same phase.
