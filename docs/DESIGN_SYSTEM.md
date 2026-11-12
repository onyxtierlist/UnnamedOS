# UNNAMED OS — Design System

**Goal:** preserve a premium, coherent visual identity on low-end and high-end hardware.

## 1. Design principles

- Clarity before decoration.
- Beautiful defaults; powerful controls when requested.
- Consistent hierarchy, spacing, and typography.
- Accessible contrast, focus, and keyboard operation.
- Motion communicates state; it is never required to understand the UI.
- Lite mode changes rendering cost, not product identity.
- Users can disable or reduce effects without breaking layouts.

## 2. Visual language

Initial direction (not final art):
- Calm, modern, understated surfaces.
- Rounded geometry with consistent radius tokens.
- Layered panels and restrained translucency.
- Clear typography and generous spacing.
- Distinct accent color, with semantic colors for status.
- Carefully designed iconography.
- Wallpaper complements rather than competes with content.

Do not copy Windows, macOS, or FlowOS assets, branding, or exact layouts. References are inspiration only; UNNAMED must develop original patterns and identity.

## 3. Token categories

Create centrally managed tokens for:
- color: base, surface, elevated surface, text, muted text, border, accent, semantic status;
- spacing: a consistent scale;
- radius: control, card, panel, window;
- typography: family, size, weight, line height;
- elevation: shadow levels and fallback border treatment;
- motion: duration, easing, reduced-motion alternatives;
- opacity and blur budgets;
- icon sizes and stroke conventions.

Tokens should be themeable and versioned. Avoid hardcoded colors throughout applications.

## 4. Effect tiers

### Lite
- Prefer opaque surfaces or low-cost translucency.
- Blur disabled or replaced by a static tint/contrast layer.
- Shadows simplified to a border or one inexpensive shadow.
- Short, optional transitions; no continuous animation.
- Static, compressed wallpaper.
- No parallax or animated background.
- Preserve depth through spacing, contrast, and shape.

### Balanced
- Selective blur for a small number of important surfaces.
- Soft shadows where performance permits.
- Short transitions and subtle state motion.
- Static wallpaper by default.

### Ultra
- Advanced blur and layered transparency.
- Richer transitions and dynamic backgrounds (opt-in).
- More complex shadowing, with frame-time safeguards.

All profiles:
- same layout and controls;
- honor reduced-motion and accessibility;
- avoid blocking input;
- degrade gracefully if a renderer lacks an effect;
- expose a manual override.

## 5. Motion

- Motion should be brief, purposeful, and interruptible.
- Do not animate large areas continuously without a clear reason.
- Provide reduced-motion mode and a no-animation option.
- Respect system accessibility settings.
- Never rely on motion alone to convey state.

## 6. Customization Studio

Organize settings into discoverable groups:
1. Appearance (light/dark/system, accent).
2. Wallpaper.
3. Panels and desktop layout.
4. Windows and workspaces.
5. Icons and typography.
6. Effects and animation.
7. Keyboard and pointer.
8. Profiles and import/export.

Theme import must validate before applying. Failed application must roll back to the last known-good theme.

## 7. Performance budgets

Set budgets only after Stage 1 baseline measurements. For every effect:
- measure frame time and GPU/CPU impact;
- test at target resolutions and refresh rates;
- avoid expensive full-screen effects in Lite;
- cache static assets and avoid needless redraws;
- prefer event-driven updates over polling.

## 8. Accessibility checklist

- Keyboard navigation and visible focus.
- Screen-reader labels where supported.
- Contrast checks.
- Scalable text and display scaling.
- Reduced motion.
- Non-color status indicators.
- Large/comfortable pointer and touch targets.
- Clear error and recovery messages.
