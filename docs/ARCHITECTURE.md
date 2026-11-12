# UNNAMED OS — Architecture

**Status:** Proposed; validate with prototypes before locking interfaces.

## 1. Layered model

```text
Applications (UNNAMED apps + ordinary Linux apps)
                 |
Desktop shell (panel, launcher, overview, notifications, quick settings)
                 |
Desktop services (preferences, themes, capability profile, integrations)
                 |
Wayland compositor / window manager (Rust + Smithay prototype)
                 |
Wayland protocols + Linux graphics/input/audio APIs
                 |
Debian Stable + Linux kernel + drivers + systemd
```

The shell is a client of the compositor. The compositor owns display/input/window policy; shell surfaces should not be privileged merely because they are visually important.

## 2. Proposed components

### Compositor
Responsibilities:
- Wayland server and protocol lifecycle.
- Window placement, focus, stacking, workspaces.
- Keyboard/pointer/touch input.
- Output configuration and scaling.
- Rendering/composition and frame scheduling.
- Session boundaries and crash handling.

Prototype candidate: Smithay. Verify supported protocols, renderer behavior, multi-monitor needs, and maintenance status against the chosen version.

### Shell
Responsibilities:
- Panel and application launcher.
- Overview and workspace UI.
- Notification presentation.
- Quick settings surfaces.
- Lock-screen UI (with secure session integration).
- Consistent interaction and accessibility.

The shell should communicate through standard Wayland protocols and narrow documented APIs rather than reaching into compositor internals.

### Desktop services
- **Preferences service:** typed, versioned settings with defaults and migration.
- **Theme manager:** validate, apply, rollback, and report theme errors.
- **Capability service:** expose hardware/renderer capability facts and selected profile.
- **Integration adapters:** desktop portals, NetworkManager, PipeWire, power/session services as appropriate.

Prefer existing D-Bus interfaces and portals. Avoid a broad privileged daemon.

### Applications
- GTK4/libadwaita for many conventional applications.
- Shell-specific rendering only where it materially improves interaction or performance.
- Standard file chooser, notification, screenshot, and other portals where available.

## 3. Adaptive rendering policy

Inputs can include:
- renderer/driver capabilities;
- available memory and memory pressure;
- output resolution and refresh;
- GPU support and observed frame timing;
- power/battery state when available;
- user-selected profile and accessibility preferences.

Policy produces a **cost profile** (blur level, shadow complexity, animation duration, wallpaper behavior, redraw strategy). It must not hide essential features.

Requirements:
- deterministic fallback if capability detection fails;
- profile changes do not require reboot;
- manual overrides take precedence;
- explain why an effect is unavailable or reduced;
- avoid polling; react to events and monitor metrics only when needed;
- use hysteresis to prevent profile flapping.

## 4. Preferences and themes

- Use typed settings with schema/version.
- Keep user preferences in the user's home directory.
- Validate imports before changing active configuration.
- Apply themes transactionally: stage → validate → apply → verify → commit; rollback on failure.
- Keep a known-good default profile.
- Theme files must not execute arbitrary code.
- Extensions are separate from themes and require a permission/compatibility model.

## 5. IPC and boundaries

- Wayland protocols for compositor/client interaction.
- D-Bus for system services and desktop integration.
- Portals for application access to privileged resources.
- Document any custom IPC and keep it narrow, versioned, and authenticated by OS user/session boundaries.

## 6. Build and packaging

- Rust workspace for owned Rust components.
- Debian packages for system integration and release artifacts.
- Reproducible development builds; pinned toolchain and dependency policy.
- CI should run formatting, linting, unit tests, and build checks.
- Image build should be scripted and documented; no manual-only release steps.

## 7. Initial workspace proposal

```text
unnamed-os/
├── README.md
├── CONTRIBUTING.md
├── docs/
│   ├── BLUEPRINT.md
│   ├── ARCHITECTURE.md
│   ├── ROADMAP.md
│   └── DESIGN_SYSTEM.md
├── crates/                 # Rust workspace (created when Stage 1 starts)
│   ├── unnamed-compositor/
│   ├── unnamed-shell/
│   ├── unnamed-settings/
│   └── unnamed-common/
├── apps/                   # first-party apps, added incrementally
├── packaging/               # Debian packaging and image configuration
├── tests/
└── tools/
```

Do not create empty code crates until their first buildable vertical slice is ready.

## 8. Architecture decision records

Use `docs/adr/` for decisions that affect interfaces, dependencies, or long-term maintenance. Each ADR should include context, options, decision, consequences, and revisit conditions.

First ADRs to write:
- Debian release and image builder.
- Smithay feasibility and compositor scope.
- Shell toolkit/rendering strategy.
- Theme/preferences format.
- Extension model and security boundary.
