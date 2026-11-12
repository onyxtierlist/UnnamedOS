# UNNAMED OS — Roadmap

**Status:** Stage 0. Stages are gates, not calendar promises. Do not claim a stage is complete until its acceptance criteria pass.

## Stage 0 — Project foundation (current)
- [x] Establish project vision and principles.
- [x] Choose Debian Stable as initial base direction.
- [x] Document initial architecture, design, and roadmap.
- [ ] Select license before accepting code contributions.
- [ ] Add initial architecture decision records.
- [ ] Set up contribution and issue templates.
- [ ] Define supported development host and VM workflow.

**Exit gate:** docs reviewed; repository conventions and first technical experiment agreed.

## Stage 1 — Wayland feasibility prototype
- [ ] Create Rust workspace and pinned toolchain.
- [ ] Build a minimal Smithay-based compositor prototype.
- [ ] Run in a disposable VM (QEMU/KVM or documented alternative).
- [ ] Show a background and at least one basic client window.
- [ ] Verify keyboard and pointer input.
- [ ] Capture logs and resource baseline.
- [ ] Document protocol and renderer limitations.

**Exit gate:** reproducible command sequence launches the prototype; known limitations are recorded; no claim of a complete desktop.

## Stage 2 — First desktop vertical slice
- [ ] Establish compositor window/focus behavior.
- [ ] Implement a basic shell surface/panel.
- [ ] Add app launcher and launch a real Linux application.
- [ ] Add workspace switching.
- [ ] Handle shell restart/crash behavior.
- [ ] Add keyboard navigation and accessibility basics.

**Exit gate:** user can boot the dev session, launch/close an app, switch workspace, and recover from a shell restart.

## Stage 3 — Design system and customization
- [ ] Define tokens for color, spacing, typography, radius, elevation, and motion.
- [ ] Add light/dark and accent color.
- [ ] Add wallpaper and icon selection.
- [ ] Add panel position/size settings.
- [ ] Add reduced-motion and effect controls.
- [ ] Implement Lite/Balanced/Ultra rendering profiles.
- [ ] Add profile export/import with validation and rollback.

**Exit gate:** settings survive restart; every tier remains usable and visually coherent; malformed imports cannot break the session.

## Stage 4 — Core desktop services
- [ ] Notifications and quick settings.
- [ ] Audio/network/brightness integration through existing Linux services.
- [ ] Lock screen and session integration.
- [ ] Preferences schema/migrations.
- [ ] Theme manager and recovery defaults.
- [ ] Logging and diagnostics.

**Exit gate:** basic desktop controls work without requiring users to edit config files or run privileged commands.

## Stage 5 — First-party applications
- [ ] Settings / Control Center.
- [ ] File manager or polished integration with a supported existing file manager.
- [ ] System monitor.
- [ ] Software Center and update frontend.
- [ ] Screenshot tool and other high-value utilities.

**Exit gate:** apps install as packages, follow the design system, and have basic tests/documented manual checks.

## Stage 6 — Debian integration and live image
- [ ] Select Debian release and image build tooling.
- [ ] Package owned components.
- [ ] Configure live session and default services.
- [ ] Produce reproducible bootable ISO.
- [ ] Boot in at least two VM configurations.
- [ ] Document checksum and image verification.
- [ ] Test persistence and installer path separately.

**Exit gate:** ISO boots to the real desktop prototype and can be rebuilt from documented source/configuration.

## Stage 7 — Hardware and performance
- [ ] Test representative low-resource and modern machines.
- [ ] Measure RAM, idle CPU, wakeups, frame times, latency, boot/login.
- [ ] Tune renderer and background services.
- [ ] Test Intel/AMD graphics; document NVIDIA status and constraints.
- [ ] Test display scaling, multiple monitors, suspend/resume.
- [ ] Verify Lite mode's visual quality and resource savings.

**Exit gate:** published measurements and a compatibility matrix; known limitations are transparent.

## Stage 8 — Alpha
- [ ] Installer and recovery workflow.
- [ ] Update/repository signing and update rollback plan.
- [ ] Security review of privileged services and extension boundaries.
- [ ] Release notes and known issues.
- [ ] Feedback and bug-report process.
- [ ] Alpha image with clear warnings.

## Stage 9 — Beta and stable release
- [ ] Freeze major interfaces.
- [ ] Stabilize updates and installer.
- [ ] Expand hardware testing.
- [ ] Accessibility and localization review.
- [ ] Release and maintenance policy.
- [ ] Stable release only when quality gates are met.

## Milestone discipline

Each milestone must include:
- code or configuration that actually runs;
- reproducible build/run instructions;
- tests or manual test checklist;
- measured performance where relevant;
- known limitations;
- documentation updates;
- a reviewed commit.
