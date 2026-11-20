# Prototype 0.2 live image

The ISO workflow builds a Debian Bookworm live image with an XFCE desktop, LightDM automatic login for the live user, NetworkManager, audio components, a file manager, terminal, and browser. XFCE is a temporary functional desktop foundation while the native UNNAMED compositor and shell are developed.

This is **not** the final UNNAMED desktop, an installer, or a stable distribution. A successful GitHub Actions build only proves that the image was produced; it does not prove that it boots or that all hardware works.

## Build

Run GitHub Actions → **UNNAMED OS prototype ISO** on `main` or `gpt6/stage1-env-safety`. Download the `unnamedos-prototype-amd64-iso` artifact.

## Validate

Follow [the VM boot checklist](../BOOT_TEST.md). Record the actual VM configuration, boot result, screenshot, and any issues before calling the image usable.
