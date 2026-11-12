# Native shell proof-of-concept

GTK4 launcher window used to validate native typography, keyboard focus, and app-grid layout. It is **not integrated into Smithay** and app buttons are placeholders.

## Build (Debian/Ubuntu)

```sh
sudo apt install build-essential pkg-config libgtk-4-dev
cd prototype/shell
cargo run
```

This starts a regular GTK window on the host desktop. Integration as a Wayland client/shell surface is a later milestone.
