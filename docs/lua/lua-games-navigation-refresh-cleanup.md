# Lua Games Navigation Refresh Cleanup

This deliverable refines the Lua Games UI after the first playable grid games.

Changes:

- Games category list keeps compact spacing but removes row borders.
- Playable game boards keep bordered tables.
- Arrow movement updates the in-memory cursor only and does not request a full-screen redraw.
- Select/OK performs the action and requests redraw.
- Back exits safely to Games.
- State remains in memory only.
- No SD save/resume is added.
- No vendor/pulp-os changes.

Current behavior intentionally avoids a redraw on cursor movement to reduce e-paper flashing. The cursor movement becomes visible after the next action/redraw. A future slice can add dirty-cell partial redraw if that feels better on-device.
