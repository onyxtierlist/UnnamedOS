# ADR 0001: Nested compositor before direct hardware

- **Status:** Accepted for prototype
- **Date:** 2026-11-12

## Context

Taking over a host display during early compositor development risks locking out the developer and makes iteration difficult. We need a safe path to test Wayland protocol handling, rendering, and shell surfaces.

## Decision

Keep the first compositor on Smithay's nested Winit backend. Run it inside an existing desktop or VM. Direct DRM/KMS backend work is deferred until surface lifecycle, input, focus, and recovery behavior are established.

## Consequences

- Easy to test without replacing the host session.
- Nested input/rendering differs from a direct session; the difference must be documented.
- No claim of a bootable OS or native display session during this phase.
