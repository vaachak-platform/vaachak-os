# Lua Games Cursor Redraw Repair

The previous navigation cleanup avoided a full redraw on arrow movement. That reduced flashing, but it also made cursor movement invisible on the X4, so the buttons appeared not to work.

This repair restores a board redraw after cursor movement for the playable Lua grid games:

- Sudoku
- Minesweeper
- Memory Cards

The Games listing keeps compact spacing and no row borders. Game boards keep their bordered table rendering.

Button behavior after this repair:

```text
Up/Down/Left/Right  Move cursor and redraw the board
Select              Act on selected cell/card and redraw
Back                Exit safely to Games
```

This is intentionally a usability repair. A future slice can replace the board redraw with dirty-cell partial redraw if the X4 display path supports it safely.

No SD save/resume is added. No vendor/pulp-os changes are made.
