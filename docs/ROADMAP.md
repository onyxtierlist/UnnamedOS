# Roadmap & acceptance gates

Stages are ordered by dependency. A stage is complete only when its exit criteria are demonstrated; documentation or mockups alone do not count as implementation.

## Stage 0 — Product foundation
- [x] Vision, architecture, design system, contribution guidance, roadmap.

## Stage 1 — Nested compositor feasibility
- [x] Smithay nested prototype source committed.
- [x] Build-check CI workflow scaffolded.
- [ ] CI green on main; resolve native dependency/compiler failures.
- [ ] Verify compositor launches and accepts a Wayland client in a Linux VM.
- [ ] Document actual test host, commands, and logs.
- [ ] Keyboard/pointer routing, focus, and graceful shutdown behave correctly.

## Stage 2 — Shell vertical slice
- [x] Browser UI concept sandbox for layout and interaction (not native shell).
- [ ] Rust shell process and compositor-owned surfaces.
- [ ] Panel, launcher, app menu, and keyboard navigation.
- [ ] Accessibility baseline: focus rings, reduced motion, contrast, scalable UI.
- [ ] No placeholder UI reported as real functionality.

## Stage 3 — Preferences & customization
- [ ] Settings service and persistent user configuration.
- [ ] Accent, wallpaper, panel position, font scale, profiles.
- [ ] Theme import/export with validation and safe defaults.
- [ ] Lite/Balanced/Ultra effect profile switching.

## Stage 4 — Desktop services
- [ ] Notifications, session actions, workspace management, clipboard.
- [ ] Settings daemon and portal integration.
- [ ] Permissions and extension boundaries.

## Stage 5 — First-party utilities
- [ ] Files, settings, monitor, terminal integration, software/update UI.
- [ ] Use established system services for privileged actions.

## Stage 6 — Debian image
- [ ] Reproducible live image build.
- [ ] Installer, recovery, signed update strategy, release metadata.
- [ ] Boot in virtual machines and selected hardware.

## Stage 7 — Hardware & performance
- [ ] Test Lite on low-resource VM and physical low-end device.
- [ ] Record idle CPU, memory, frame pacing, battery/thermal behavior.
- [ ] Regression budgets and automated smoke tests.

## Stage 8 — Alpha
- [ ] Install/upgrade/recovery test matrix.
- [ ] Security review, accessibility review, crash reporting policy.
- [ ] Known issues and supported hardware published.

## Stage 9 — Beta / stable
- [ ] Feature freeze and release candidate process.
- [ ] Installer and update reliability demonstrated.
- [ ] Documentation, support policy, release artifacts, checksums.

## Current truth

The repository contains a nested compositor prototype, design docs, and a browser-based shell concept. It is **not yet a bootable distribution**, and no native desktop shell is complete.
