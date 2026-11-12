# UNNAMED OS — Master Blueprint

**Version:** 1.0  
**Status:** Stage 0 / Planning  
**Project motto:** Light as Linux. Beautiful as Windows 11.

## 1. Purpose

UNNAMED OS is a Debian-based Linux distribution with an original desktop environment. It aims to combine efficient system behavior, premium visual polish, and extensive customization across old and modern PCs.

It is not merely Debian with a theme. Its identity comes from its desktop shell, design system, adaptive rendering, settings experience, integrations, and first-party utilities.

## 2. Product principles

1. **Lightweight by design:** minimize idle CPU work, memory use, wakeups, background services, and unnecessary redraws.
2. **Beautiful by default:** visual quality must not depend on a powerful GPU.
3. **Adaptive beauty:** replace expensive effects with cheaper equivalents instead of removing the design.
4. **Customization as a first-class feature:** expose visual and behavioral settings in a friendly UI.
5. **Linux compatibility:** preserve access to Linux applications and mature hardware support.
6. **User agency:** automatic hardware choices are explainable and overridable.
7. **Reliability:** stable defaults, recovery paths, conservative system integration.
8. **Honesty:** mark planned, experimental, and implemented features accurately.

## 3. Distribution foundation

Initial base: **Debian Stable**.

Use the distribution for the kernel, firmware and driver ecosystem, systemd, networking, audio, graphics stack, package management, security fixes, and foundational utilities. UNNAMED OS owns its experience and integration—not every low-level subsystem.

Planned system pieces:
- Linux kernel and Debian-maintained drivers/firmware policy.
- systemd and standard Linux service interfaces.
- Wayland display stack.
- NetworkManager for network management.
- PipeWire for audio and media.
- Mesa and supported vendor graphics drivers.
- Debian packages and signed repository metadata.
- A reproducible live image and installer path.

Exact Debian release, firmware policy, supported architectures, and image tooling are release decisions and must be recorded.

## 4. Desktop experience

### Shell components
- **Compositor/window manager:** Wayland session, windows, input, output management, focus, stacking, and workspace behavior.
- **Shell:** panel, launcher, overview, notifications, quick settings, lock screen, and desktop surfaces.
- **Settings / Control Center:** discoverable system and desktop configuration.
- **Customization Studio:** theme authoring/import, profiles, and appearance controls.
- **Integration services:** theme application, hardware capability reporting, desktop preferences, and system API adapters.

Keep compositor responsibilities separate from shell UI where possible. A shell crash should not unnecessarily destroy the whole session; restart and recovery behavior should be considered early.

## 5. Adaptive visual system

Three named profiles:

### Lite
- Low-cost solid or translucent surfaces; avoid expensive blur by default.
- Simple shadows and short transitions.
- Static/optimized wallpaper.
- Event-driven UI and minimal background activity.
- No unnecessary animation or continuous polling.
- Same panel, launcher, settings, theme controls, and features.

### Balanced
- Selective blur and richer shadows where measured to be affordable.
- Smooth but restrained transitions.
- Moderate effects and optimized wallpaper behavior.

### Ultra
- Advanced blur, layered transparency, richer motion, and dynamic effects when supported.
- Effects remain interruptible and should not block interaction.

The system should consider GPU/driver capabilities, memory, display configuration, and measured rendering behavior—not a single CPU model or age heuristic. Automatic tier selection must be user-overridable and explainable.

## 6. Customization

Initial settings surface:
- light/dark/system appearance;
- accent and semantic colors;
- wallpapers and slideshow;
- fonts and icon theme;
- panel position, size, visibility, and behavior;
- window decoration and layout preferences;
- motion intensity and reduced-motion mode;
- transparency/blur/shadow levels;
- workspace behavior;
- keyboard shortcuts and pointer settings;
- notification behavior;
- desktop widgets (later, with lifecycle/resource rules);
- profiles, export/import, reset, and recovery.

Themes should use a documented format, validate before application, and fail safely. Extensions need version compatibility, explicit permissions, resource limits where possible, and a clear disable/recovery path.

## 7. First-party applications

Prioritize integration and consistency over rewriting every Linux application.

Planned:
1. Settings / Control Center.
2. File manager.
3. Terminal integration.
4. System monitor.
5. Software Center / package frontend.
6. Update manager.
7. Screenshot tool.
8. Text editor and calculator.

Applications may use GTK4/libadwaita or another proven toolkit. A custom shell renderer is not a requirement for every app.

## 8. Performance and quality

Track at minimum:
- idle RAM (system, compositor, shell, and session separately);
- idle CPU and wakeups over a defined interval;
- compositor frame-time distribution and missed frames;
- input-to-visible-response latency;
- cold/warm application launch times;
- battery/power behavior where measurable;
- crash-free session duration;
- boot and login time.

Every benchmark must state machine specifications, driver, resolution, profile, workload, measurement method, and sample size. Targets will be set after a baseline exists; avoid unsupported “runs on any old PC” claims.

## 9. Security, updates, and recovery

- Use Debian's signed package infrastructure and a planned signed UNNAMED repository.
- Keep privileged operations in narrow, auditable system services.
- Do not run the shell as root.
- Validate theme and extension inputs.
- Prefer sandboxing for untrusted applications/extensions where available.
- Provide logs and diagnostics without collecting telemetry by default.
- Design update failure recovery and a safe boot/recovery path.
- Never promise secure boot compatibility until tested and documented.

## 10. Release shape

1. Developer VM prototype.
2. Internal bootable live image.
3. Developer/technical alpha.
4. Installable alpha with recovery documentation.
5. Beta on selected real hardware.
6. Stable release only after update, installer, compatibility, and security processes are mature.

## 11. Out of scope initially

- Writing a kernel.
- Replacing systemd, NetworkManager, PipeWire, or Mesa.
- Universal support for every PC.
- Windows application compatibility guarantees.
- A custom app ecosystem before Linux app support works.
- Cloud accounts or mandatory telemetry.
- Unbounded extension execution.

## 12. Definition of done

A release is not “done” because a screenshot looks good. Each stage needs:
- reproducible build/run instructions;
- tested behavior and known limitations;
- basic automated tests where feasible;
- performance measurements for relevant changes;
- documentation updates;
- reviewed changes and a commit.
