# Display drawing abstractions ownership

This document records the Vaachak-owned pure display/chrome layout model.

## Current compatibility contract

The active X4 runtime still owns actual drawing, strip rendering, SSD1677 access, refresh policy, and physical display behavior. This extraction only adds pure layout metadata in `vaachak-core`.

Current display assumptions captured by the model:

- X4 logical screen: `800x480`
- Default X4 orientation metadata: portrait with 270-degree rotation in the current runtime convention
- Header/chrome is modeled separately from body content
- Footer/status chrome is modeled separately from body content
- Reader cache diagnostics are body notices, not header text
- Battery/status placement belongs in the header region
- Reader progress/status placement belongs in the footer/status region
- Popup/message placement is inside the body region and avoids header/footer chrome

## Non-goals

This extraction does not move:

- SSD1677 driver behavior
- strip rendering
- EPD refresh logic
- partial/full refresh policy
- reader drawing implementation
- file browser rendering
- settings rendering
- Date & Time rendering
- Wi-Fi Transfer rendering

## App contexts

The pure model names layout contexts for:

- Home/category dashboard
- Files/Library
- Reader
- Settings
- Date & Time
- Wi-Fi Transfer

The current regions are intentionally conservative and shared across contexts until runtime drawing is moved in a later slice.
