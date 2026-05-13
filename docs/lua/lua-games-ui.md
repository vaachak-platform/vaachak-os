

## Lua Ludo Track UI Cleanup

Ludo uses a compact four-lane track UI instead of the earlier 10x10 dotted board. This better matches the current simplified in-memory Ludo rules:

- A/B/C/D are the four player tokens.
- Each token has a start-to-finish track with `S` and `F` markers.
- The selected token row is highlighted.
- Arrow buttons choose the active token.
- OK rolls the deterministic dice and advances the selected token.
- Back exits safely to Games.

The physical app folder remains `/VAACHAK/APPS/LUDO`; no SD save/resume is added in this cleanup.
