# UNNAMED OS Prototype 0.2 — Live desktop test

This prototype is a Debian Bookworm live image with an XFCE desktop session configured for automatic login. It is intended to provide a usable graphical Linux environment while the native UNNAMED compositor and shell are developed.

**Important:** XFCE is a temporary desktop foundation. This image does not yet run the custom UNNAMED desktop shell, and it is not a finished distribution or installer.

## Build

1. Open GitHub Actions → **UNNAMED OS prototype ISO**.
2. Run the workflow on `gpt6/stage1-env-safety` (or `main` after the changes are merged).
3. Download the `unnamedos-prototype-amd64-iso` artifact.

## VM test

- Create a Linux VM with 2 CPU cores, 4 GB RAM, and a 30 GB virtual disk.
- Attach the ISO as optical media and boot it.
- Select the Debian live entry.
- The expected result is a graphical LightDM/XFCE session. The live user is configured for automatic login.
- Verify display, keyboard, mouse, network, terminal, file manager, audio where available, and shutdown.
- Keep VM snapshots; do not install to a host disk.

## Acceptance evidence

Record the GitHub Actions run URL, boot screenshot, VM configuration, and failures in an issue. A successful image build is not proof of successful VM boot. Do not mark the image milestone complete until a real VM boot and basic interaction test are recorded.
