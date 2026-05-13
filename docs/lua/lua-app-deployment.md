# Lua App Deployment Contract

This document records the accepted Vaachak OS Lua app deployment model.

## Canonical SD root

All optional Lua apps are deployed under the SD-card root path:

```text
/VAACHAK/APPS
```

Native Vaachak OS features remain authoritative. Lua apps are optional additions for tools, productivity, experiments, and simple games.

## Physical naming rule

The current X4 SD/FAT path is safest with uppercase 8.3 physical names. Use:

```text
NAME.EXT
```

where `NAME` is at most 8 characters and `EXT` is at most 3 characters. Keep folder names uppercase. Keep long logical identity inside `APP.TOM`.

## Current sample app map

```text
/VAACHAK/APPS/CALENDAR -> id = "calendar"      -> Productivity -> Calendar
/VAACHAK/APPS/PANCHANG -> id = "panchang"      -> Productivity -> Panchang
/VAACHAK/APPS/MANTRA   -> id = "daily_mantra"  -> Tools        -> Daily Mantra
/VAACHAK/APPS/DICT     -> id = "dictionary"    -> Tools        -> Dictionary
/VAACHAK/APPS/UNITS    -> id = "unit_converter"-> Tools        -> Unit Converter
/VAACHAK/APPS/SUDOKU   -> id = "sudoku"        -> Games        -> Sudoku
/VAACHAK/APPS/MINES    -> id = "minesweeper"   -> Games        -> Minesweeper
/VAACHAK/APPS/FREECELL -> id = "freecell"      -> Games        -> FreeCell
/VAACHAK/APPS/MEMCARD  -> id = "memory_cards"  -> Games        -> Memory Cards
/VAACHAK/APPS/SOLITAIR -> id = "solitaire"     -> Games        -> Solitaire
/VAACHAK/APPS/LUDO     -> id = "ludo"          -> Games        -> Ludo
/VAACHAK/APPS/SNAKES   -> id = "snakes_ladder" -> Games        -> Snakes and Ladder
```

## Required sample layout pattern

Each app folder should contain an `APP.TOM` manifest and a `MAIN.LUA` entry file. Apps may include bounded data files next to the entry file or under a short uppercase data folder.

Examples:

```text
/VAACHAK/APPS/MANTRA/APP.TOM
/VAACHAK/APPS/MANTRA/MAIN.LUA
/VAACHAK/APPS/MANTRA/MANTRAS.TXT

/VAACHAK/APPS/PANCHANG/APP.TOM
/VAACHAK/APPS/PANCHANG/MAIN.LUA
/VAACHAK/APPS/PANCHANG/DATA/Y2026.TXT

/VAACHAK/APPS/DICT/APP.TOM
/VAACHAK/APPS/DICT/MAIN.LUA
/VAACHAK/APPS/DICT/INDEX.TXT
```

## Manifest rule

`APP.TOM` uses logical app identity and dashboard metadata:

```text
id = "daily_mantra"
name = "Daily Mantra"
category = "Tools"
type = "activity"
version = "0.1.0"
entry = "MAIN.LUA"
capabilities = ["display", "input", "storage", "time"]
```

The physical folder is short and uppercase. The logical app id remains descriptive snake_case.

## Wi-Fi Transfer rule

Wi-Fi Transfer should upload to uppercase 8.3 physical paths. For example:

```text
/VAACHAK/APPS/MANTRA/APP.TOM
/VAACHAK/APPS/MANTRA/MAIN.LUA
/VAACHAK/APPS/MANTRA/MANTRAS.TXT
```

Avoid uploading lower-case long-name folders such as `/VAACHAK/APPS/daily_mantra` while the embedded FAT path layer is short-name oriented.

## Commit hygiene

Old patch zip files, extracted patch folders, and temporary generated apply/patch/validator scripts should not be committed. Keep only source files, docs, examples, tools, and production helper scripts.
