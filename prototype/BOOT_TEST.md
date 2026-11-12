# UNNAMED OS Prototype 0.1

This is the first ISO pipeline. It boots Debian Stable live with Weston and utilities, but does not yet start the UNNAMED OS custom compositor/shell. Do not call it the finished desktop.

## Build
GitHub Actions → **UNNAMED OS prototype ISO** → Run workflow. Download artifact `unnamedos-prototype-amd64-iso`.

## VM test
- Create a Linux VM (2 CPU cores, 4 GB RAM, 30 GB virtual disk; UEFI or BIOS).
- Attach ISO as optical media and boot.
- Select live entry. Verify display, keyboard, network, terminal and shutdown.
- Keep VM snapshots; do not install to host disk.

## Acceptance evidence
Record GitHub Actions run URL, boot screenshot, VM config, and any failures in an issue. Only after this base works should we integrate our compositor and shell, then rebuild image.
