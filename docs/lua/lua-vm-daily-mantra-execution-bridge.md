
# Lua VM Daily Mantra Execution Bridge

This slice proves VM-backed execution inside the already-working Daily Mantra app without changing the default firmware path.

## Scope

- Physical SD app folder remains `/VAACHAK/APPS/MANTRA`.
- Logical app id remains `daily_mantra`.
- The normal build without `lua-vm` continues to use the existing SD loader and safe declaration-subset parser.
- The `lua-vm` feature first runs an in-memory smoke script: `return 1 + 2`.
- After the smoke script succeeds, the bridge extracts one constrained expression from `MAIN.LUA` and executes it through `support/vaachak-lua-vm`.
- If the VM path fails or no VM expression is present, the existing fallback/subset parser remains active.

## MAIN.LUA VM expression contract

The VM bridge accepts either form:

```lua
vm_expression = "return 108 + 0"
```

or:

```lua
-- vaachak-vm: return 108 + 0
```

The current VM smoke crate intentionally supports only tiny integer return expressions such as:

```lua
return 1
return 1 + 2
return 7 - 4
return 3 * 5
return 8 / 2
```

## Diagnostics

The bridge uses these diagnostics:

```text
VM disabled
VM script loaded
VM execution failed
fallback parser used
```

The success marker is:

```text
vaachak-lua-daily-mantra-vm-bridge-ok
```

## Build behavior

Default build:

```bash
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
```

VM-enabled check:

```bash
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf --features lua-vm
```

No SD app execution is added outside the already-existing Daily Mantra route. No dashboard behavior changes are introduced by this slice.
