# App Category Dashboard Home Wiring

The current implementation makes the category dashboard the main Vaachak OS Home screen.

## Home

```text
Network
Productivity
Games
Reader
System
Tools
```

Home displays `6 categories` directly. There is no separate Apps tile to enter before showing categories.

## Category routes

```text
Network
  Wi-Fi Connect        placeholder
  Wi-Fi Transfer       placeholder
  Network Status       placeholder

Productivity
  Daily Mantra         existing Daily Mantra status screen

Games
  Coming soon          placeholder

Reader
  Continue Reading     existing reader continue/library behavior
  Library              existing Files/Library flow
  Bookmarks            existing bookmark list

System
  Settings             existing Settings app
  Sleep Image          placeholder for now
  Device Info          placeholder for now

Tools
  File Browser         existing Files/Library flow
  QR Generator         placeholder
```

## Navigation

```text
Home category dashboard
  Select -> open selected category

Category item list
  Select -> open selected item
  Back   -> return to Home category dashboard

Daily Mantra / placeholder / Bookmarks
  Back   -> return to the category item list that opened it
```

## Active files changed

```text
vendor/pulp-os/src/apps/home.rs
```

The model/reference file remains included:

```text
src/apps/app_category_dashboard.rs
```

## Validation

After applying, run:

```bash
cargo fmt --all
cargo build --release
```

Then flash and verify:

```text
Home shows Network, Productivity, Games, Reader, System, Tools
Productivity -> Daily Mantra opens the existing status screen
Reader -> Continue Reading preserves continue behavior
Reader -> Library opens existing file/library flow
Reader -> Bookmarks opens existing bookmark list
System -> Settings opens existing Settings
Tools -> File Browser opens existing file/library flow
```
