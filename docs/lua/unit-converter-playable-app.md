# Lua Unit Converter Playable App

The Unit Converter Lua app uses the canonical SD app path:

```text
/VAACHAK/APPS/UNITS/APP.TOM
/VAACHAK/APPS/UNITS/MAIN.LUA
/VAACHAK/APPS/UNITS/UNITS.TXT
```

Physical folder/file names remain uppercase 8.3-safe. The logical manifest id remains:

```text
unit_converter
```

Initial playable behavior is intentionally simple and offline:

```text
Up/Down       choose conversion type
Left/Right    choose sample amount
OK            choose next sample amount
Back          exit safely to Tools
```

Supported initial conversions:

```text
Length, Weight, Temperature, Volume, Speed, Data
```

No SD save/resume is added in this slice. No vendor/pulp-os files are changed.
