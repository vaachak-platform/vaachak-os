# Phase 36A Notes

Phase 36A is intentionally narrow. It takes over active semantic mapper construction without touching ADC/debounce/input polling.

The imported `AppManager` still expects the imported concrete `ButtonMapper` type, so this phase uses a Vaachak-owned factory and preflight adapter to provide a behavior-equivalent mapper.

A deeper extraction can come later by replacing or wrapping more of the imported AppManager/input dispatch path.

Suggested next phases:

```text
Phase 36B — Active Input Semantic Config Sync
Phase 37A — Input ADC/debounce takeover
Phase 38A — Progress state persistence adapter
```
