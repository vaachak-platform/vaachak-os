# Phase 30 Implementation Notes

Use these implementation tactics:

```text
1. First create target-xteink-x4/src/vaachak_x4/.
2. Move or copy existing Vaachak-owned runtime boundary modules into vaachak_x4/contracts/.
3. Move the accepted imported runtime into vaachak_x4/imported/pulp_reader_runtime.rs.
4. Update main.rs so it only loads vaachak_x4.
5. Keep the real async runtime entrypoint exactly where it compiles.
6. If the async entrypoint must remain in the imported runtime wrapper, that is acceptable.
7. Emit only vaachak=x4-runtime-ready during normal boot.
8. Keep old phase marker constants/helpers but do not call them in the active boot path.
9. Add scripts and docs.
10. Run validation.
```

Avoid clever refactors. This phase is about ownership and layout, not behavior.
