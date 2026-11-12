# Prototype ISO build

The workflow `.github/workflows/build-prototype-iso.yml` can build a Debian Stable live ISO as a GitHub Actions artifact. Trigger it manually from Actions or push changes to branch `prototype-0.1-bootable`.

**Important:** this first image is a bootable Debian live environment with Weston and common utilities. It does NOT yet run the UNNAMED OS compositor or custom shell automatically. It is an infrastructure/boot milestone, not release 0.1.

Use a VM first. Artifact is temporary (14 days); no ISO is committed to Git.
