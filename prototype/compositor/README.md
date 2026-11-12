# UNNAMED OS compositor prototype (Stage 1)

This is a **nested compositor feasibility prototype**, not a complete desktop or OS. It opens a window inside your existing Linux desktop and hosts Wayland client surfaces inside it. Nested mode is intentional: it is safer to develop and test before attempting to take over a machine's display.

## What this slice proves

- Rust + Smithay can initialize a nested Wayland compositor.
- A Wayland socket can accept clients.
- XDG toplevels can be configured and rendered.
- We have a base to build UNNAMED OS shell surfaces and input handling on.

## Prerequisites (Debian/Ubuntu)

Install Rust (stable, Rust 1.88+), Cargo, and native build dependencies:

```sh
sudo apt install build-essential pkg-config libwayland-dev libxkbcommon-dev libegl1-mesa-dev libgles2-mesa-dev libudev-dev libinput-dev libgbm-dev libseat-dev libdisplay-info-dev
```

Some environments may need additional development packages depending on their graphics stack.

## Build and run

From this directory:

```sh
cargo check
cargo run
```

The prototype opens a nested window and creates a Wayland socket named `wayland-5`. It attempts to launch `weston-terminal` if installed; otherwise connect any Wayland client manually by setting `WAYLAND_DISPLAY=wayland-5` in the environment where the compositor is running.

**Do not run this on a bare TTY or replace your system session.** It is a nested development program only.

## Current limitations

- Winit nested backend only; no DRM/KMS direct display backend.
- Minimal keyboard handling; no pointer/touch routing or window management policy.
- No UNNAMED OS panel, launcher, settings, effects, or adaptive tier implementation yet.
- No bootable ISO or installer.

## Upstream reference

The initial compositor loop and Smithay protocol wiring are adapted from Smithay's official `examples/minimal.rs` (MIT): https://github.com/Smithay/smithay/blob/master/examples/minimal.rs . This prototype will be refactored into project-owned modules as functionality grows.
