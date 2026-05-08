# Input semantic mapping ownership

This document records the Vaachak-owned pure model for input semantics.

## Current compatibility contract

The active X4 runtime still owns physical input behavior:

- ADC ladder scanning remains in the Pulp-derived runtime.
- Button debounce behavior remains in the Pulp-derived runtime.
- Physical GPIO/ADC mapping remains in the Pulp-derived runtime.
- App event loops remain in the Pulp-derived runtime.

This extraction only defines what a button means after the runtime has already produced a physical button event.

## Semantic actions

The Vaachak-owned action set is:

- Up
- Down
- Left
- Right
- Select
- Back
- Menu
- Power
- Unknown

Directional actions are repeatable by default. Select, Back, Menu, and Power are one-shot actions. Unknown is disabled.

## App contexts

The pure mapping currently covers:

- Home/category dashboard
- Files/Library
- Reader
- Settings
- Date & Time
- Wi-Fi Transfer

## Reader mapping

Reader page navigation is modeled without changing runtime behavior:

- Left/Up -> previous page
- Right/Down/Select -> next page
- Back -> exit to Library/Home
- Menu -> open reader menu
- Power -> no page navigation; reserved as a system action

## Non-goals

This extraction does not move ADC scanning, debounce, GPIO mapping, app event loops, reader navigation implementation, Settings implementation, Date & Time runtime behavior, or Wi-Fi Transfer runtime behavior.
