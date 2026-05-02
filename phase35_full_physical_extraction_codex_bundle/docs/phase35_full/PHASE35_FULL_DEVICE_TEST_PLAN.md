# Phase 35 Full Device Test Plan

After Codex passes software validation, user should flash and validate:

```text
1. Boot marker is vaachak=x4-physical-runtime-owned.
2. Home/library appears.
3. Input navigation works on all buttons.
4. TXT/MD opens.
5. EPUB/EPU opens with real text.
6. Next/previous page work.
7. Back returns to library.
8. Continue restores progress.
9. Bookmark add/view/remove behavior works.
10. Theme/menu/footer behavior works.
11. Display refresh/strip rendering has no clipping/inversion regression.
12. Power/reset cycle preserves state.
```
