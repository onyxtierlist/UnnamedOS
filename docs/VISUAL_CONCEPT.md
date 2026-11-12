# UNNAMED OS — Visual concept (not a screenshot)

This is an original, editable SVG concept illustration for the future desktop. It is **not running UI** and is not a promise that these components already exist.

![UNNAMED OS desktop concept](mockup-desktop.svg)

## Visual direction

- Deep navy foundation, blue/cyan/violet accent range, restrained glow.
- Rounded but not excessively inflated surfaces; readable contrast and clear hierarchy.
- Centered launcher with search, pinned apps, and recent activity.
- Optional desktop widgets for clock and system health; widgets must be removable.
- Bottom panel with launcher, active apps, and status area.
- Lite keeps the same layout and palette using opaque surfaces and simpler shadows; Balanced can use selective blur; Ultra can add richer blur and motion.
- The question-mark mark is the temporary original brand symbol. Do not use Windows, Microsoft, or third-party app logos as UNNAMED OS branding.

## Assets

- [Question-mark brand mark](../../assets/branding/unnamed-mark.svg)
- [Desktop concept (SVG)](mockup-desktop.svg)

The SVG is deliberately simple and source-editable. The generated cinematic image is a separate inspiration reference; this SVG is the repo's editable design artifact.

## Next design implementation steps

1. Define reusable design tokens (color, spacing, radii, typography, elevation).
2. Build a standalone shell mock/demo to validate layout and accessibility.
3. Implement launcher/panel as real Wayland surfaces after compositor input/focus work.
4. Replace placeholder glyphs with original, consistent icon set.
5. Validate Lite mode on a low-resource VM; measure memory and idle CPU.
