# ADR 0002: Performance tiers preserve feature parity

- **Status:** Accepted
- **Date:** 2026-11-12

## Context

Low-end devices need reduced rendering cost, but a Lite edition that removes useful features or becomes visually unrelated would violate the project's design goal.

## Decision

Lite, Balanced, and Ultra are rendering/performance profiles—not product editions. All profiles retain the same core features and customization controls. Profiles may vary blur, shadow quality, animation complexity, background refresh, and dynamic effects. Users can override automatic choices, subject to hardware limitations.

## Consequences

- Shared shell and behavior across hardware tiers.
- Each effect needs a cheaper fallback.
- Performance claims require measurements on documented hardware and workloads.
