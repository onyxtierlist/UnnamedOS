# UNNAMED OS

> **Light as Linux. Beautiful as Windows 11.**

UNNAMED OS is an ambitious, Debian-based Linux distribution with an original desktop environment, a carefully engineered adaptive visual system, and deep user customization. The goal is not to reskin an existing desktop: it is to build a distinct, cohesive operating-system experience while reusing mature Linux infrastructure for hardware, networking, audio, security, and package management.

**Status:** Planning / Stage 0 — no functional OS or desktop exists yet. This repository is the source of truth for the project plan.

## Vision

- **Lightweight by design:** minimize idle work, unnecessary services, memory pressure, and rendering cost.
- **Beautiful by default:** polished typography, spacing, surfaces, icons, motion, and coherent interaction.
- **Customizable without limits:** user-controlled themes, layouts, effects, shortcuts, profiles, and extensions.
- **Adaptive, not stripped-down:** Lite mode keeps the same visual identity and essential features, using cheaper implementations of effects.
- **Real Linux:** ultimately bootable from USB or disk, installable, updateable, and usable as a daily system.
- **Own identity:** original shell, settings experience, system utilities, visual language, branding, and applications where practical.

## Design promise: beauty at every performance tier

| Tier | Experience |
|---|---|
| **Lite** | Elegant opaque/translucent surfaces, restrained shadows, short transitions, static wallpapers, low background activity. |
| **Balanced** | Selective blur, richer shadows, smooth motion, moderate effects. |
| **Ultra** | Advanced blur, layered transparency, richer animations, dynamic backgrounds where supported. |

These tiers are implementation profiles, not feature gates. The same desktop, controls, and customization options remain available in every tier. Users may override automatic choices.

## Planned highlights

- Original Wayland desktop shell and compositor.
- Panel, launcher, workspaces, notifications, lock screen, and quick settings.
- **Customization Studio:** themes, accent colors, icons, fonts, wallpaper, panel, effects, layouts, and profiles.
- Hardware-aware adaptive rendering with clear manual controls.
- First-party settings, file manager, terminal integration, system monitor, software center, and update experience.
- Community themes and extensions with explicit permissions and safe installation.
- Debian-based live image, installer, recovery path, and reproducible release process.

## Technology direction

- **Base:** Debian Stable (release and support window selected deliberately for each milestone).
- **Kernel / system:** Linux, systemd, NetworkManager, PipeWire, Mesa and distro-supported drivers.
- **Display:** Wayland; evaluate Smithay for the custom compositor.
- **Core code:** Rust.
- **App UI:** GTK4/libadwaita where it fits; custom shell surfaces may use a different toolkit/rendering path if benchmarks justify it.
- **Packaging:** Debian packages; prototype image built with Debian live-build or another documented reproducible image pipeline.

These are initial choices. Architecture decisions must be tested in small prototypes before they become irreversible.

## Documentation

- [Master blueprint](docs/BLUEPRINT.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Roadmap and acceptance criteria](docs/ROADMAP.md)
- [Design system](docs/DESIGN_SYSTEM.md)
- [Contributing](CONTRIBUTING.md)

## Development principles

1. **No fake completion:** clearly label mockups, prototypes, and implemented features.
2. **Measure, don't guess:** record hardware, workload, and methodology for performance claims.
3. **Preserve usability in Lite:** reduce cost, not identity or core capability.
4. **Prefer mature infrastructure:** do not rewrite kernel, drivers, package management, or security primitives without a compelling reason.
5. **Small vertical slices:** each milestone should build, run, and have tests or documented manual checks.
6. **Document decisions:** use architecture decision records when a choice affects long-term direction.
7. **Safe by default:** updates, permissions, extensions, and recovery are product requirements—not afterthoughts.

## Current status

Stage 0: planning and repository foundation. The next implementation milestone is a **Wayland compositor feasibility prototype** in a VM, followed by a minimal shell surface and measurable resource baseline.

## License

The license will be selected before code contributions begin. Until then, repository documentation is not an invitation to assume a particular code license.
