<!-- BEGIN LUA_APP_DISCOVERY_DEPLOYMENT_NAMES -->
## Lua app discovery deployment names

Discovery treats `/VAACHAK/APPS` as the app root. Physical folder names are 8.3-safe aliases; logical ids come from `APP.TOM`.

```text
physical folder: MANTRA
manifest path:   /VAACHAK/APPS/MANTRA/APP.TOM
entry path:      /VAACHAK/APPS/MANTRA/MAIN.LUA
logical id:      daily_mantra
```

Future SD discovery should enumerate physical folders under `/VAACHAK/APPS`, read `APP.TOM`, validate the logical app id, and convert valid apps into dashboard catalog entries.
<!-- END LUA_APP_DISCOVERY_DEPLOYMENT_NAMES -->
