# Lua Card Games Playable Pack

This slice completes the target-side wiring for the first two card games:

- `/VAACHAK/APPS/FREECELL` -> `freecell`
- `/VAACHAK/APPS/SOLITAIR` -> `solitaire`

Both games use a small in-memory card table. Arrow buttons move the cursor. Select picks or drops a card. Back exits safely to Games. Legal move enforcement, win-state, and SD save/resume remain deferred.

No vendor/pulp-os changes are made.
