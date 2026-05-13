# Lua Games Catalog + Stub Pack

Vaachak OS Lua games are deployed under the canonical SD root:

```text
/VAACHAK/APPS
```

This deliverable adds SD-loaded placeholder screens for the first requested Games apps. The physical folders are uppercase 8.3-safe, while logical app ids remain descriptive snake_case inside `APP.TOM`.

| Physical folder | Logical app id | Display label |
| --- | --- | --- |
| `SUDOKU` | `sudoku` | Sudoku |
| `MINES` | `minesweeper` | Minesweeper |
| `FREECELL` | `freecell` | FreeCell |
| `MEMCARD` | `memory_cards` | Memory Cards |
| `SOLITAIR` | `solitaire` | Solitaire |
| `LUDO` | `ludo` | Ludo |
| `SNAKES` | `snakes_ladder` | Snakes and Ladder |

Each folder contains `APP.TOM`, `MAIN.LUA`, and a minimal data file. The first runtime slice loads `APP.TOM` and `MAIN.LUA`, renders the title/instructions on-device, and exits safely back to Games with Back.

No full game logic, SD save/resume, raw SD/FAT/SPI change, or `vendor/pulp-os` change is included in this pack.
