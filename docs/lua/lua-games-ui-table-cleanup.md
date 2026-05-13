# Lua Games UI Table Cleanup

This deliverable improves the first playable grid games and the Games category listing.

Changes:

- Games category rows use dynamic compact table rows so seven games do not overlap.
- Sudoku, Minesweeper, and Memory Cards render bordered tables instead of text-only rows.
- Cursor movement still uses the same input contract: arrows move, Select acts, Back exits.
- State remains in memory only.
- No vendor/pulp-os changes.
