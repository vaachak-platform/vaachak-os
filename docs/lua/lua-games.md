

## Lua Card Games Full-Screen Table Cleanup

FreeCell and Solitaire use the same e-paper-safe card table policy as the grid games:

- the Games list remains borderless and compact;
- game boards keep table borders;
- FreeCell uses an 8-column, 7-row full-width card table;
- Solitaire uses a 7-column, 7-row full-width tableau table;
- the header status is short (`SD Lua` or `Picked`) so it does not clip as `Missing APP.TOM`;
- arrow buttons move the cursor and redraw the board;
- OK picks/drops a card;
- Back exits safely to Games;
- state remains in memory only for this slice.

Physical folders remain 8.3-safe:

```text
/VAACHAK/APPS/FREECELL
/VAACHAK/APPS/SOLITAIR
```
