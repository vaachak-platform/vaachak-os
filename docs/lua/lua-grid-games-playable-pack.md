# Lua Grid Games Playable Pack

This deliverable makes the first three grid-friendly Lua Games apps playable on-device while keeping state in memory only:

- Sudoku under `/VAACHAK/APPS/SUDOKU`
- Minesweeper under `/VAACHAK/APPS/MINES`
- Memory Cards under `/VAACHAK/APPS/MEMCARD`

Button contract:

```text
Up/Down/Left/Right  Move cursor
Select              Act on selected cell/card
Back                Exit safely to Games
```

The slice intentionally does not add SD save/resume, recursive scanning, or vendor changes.
