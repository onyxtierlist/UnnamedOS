# UNNAMED OS — Shell UI concept

> **This is a UI prototype, not the real operating system.** It is a browser-based interaction/design demo and does not run Linux, manage windows, launch native apps, or change system settings.

## Run it

Open `index.html` in a modern browser. No build step or network dependency is required.

## Prototype interactions

- Open/close the app launcher using the `?` button or `Ctrl/⌘ + Space`.
- Search/filter the pinned app tiles.
- Toggle demo quick-control switches.
- Click app tiles to show placeholder feedback.
- Clock/date update from your browser's local time.

## What's next

This is a visual/interaction sandbox used to test layout and responsive behavior. Real desktop surfaces will be implemented in Rust/Smithay after compositor focus, input, and surface lifecycle are reliable. Never treat these browser controls as actual system controls.
