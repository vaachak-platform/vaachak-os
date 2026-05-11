# Lua Daily Mantra Text Rendering Cleanup

This slice cleans the on-device Daily Mantra layout after the first VM proof.

## Screen contract

The app screen shows:

```text
Daily Mantra
Day: <weekday or Today>
Mantra: <wrapped mantra text>
```

The physical SD layout remains:

```text
/VAACHAK/APPS/MANTRA/APP.TOM
/VAACHAK/APPS/MANTRA/MAIN.LUA
/VAACHAK/APPS/MANTRA/MANTRAS.TXT
```

`MANTRAS.TXT` uses `|` as a record separator. The separator is never rendered.

## Record forms

```text
Monday|Om Namah Shivaya|A steady mind turns every page into practice.
Om Shanti Shanti Shanti|Peace in thought, word, and action.
```

The first form displays `Day: Monday`. The second displays `Day: Today`.

## Runtime behavior

Back exits safely to Tools. The optional `lua-vm` path still runs `vm_expression` from `MAIN.LUA`, but it no longer overwrites the Day/Mantra content lines.
