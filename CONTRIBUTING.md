# Contributing to UNNAMED OS

Thanks for helping build UNNAMED OS. The project is currently in Stage 0 (planning); architecture and implementation will evolve through small, testable milestones.

## Before contributing
- Read [README](README.md), [Blueprint](docs/BLUEPRINT.md), [Architecture](docs/ARCHITECTURE.md), and [Roadmap](docs/ROADMAP.md).
- Check existing issues and discussions before starting large work.
- For major architectural changes, propose an ADR or issue first.
- Do not assume a planned feature is already implemented.

## Development expectations
- Prefer small, focused changes and buildable vertical slices.
- Explain why a dependency is needed and its maintenance/security implications.
- Use Rust formatting (`cargo fmt`) and linting (`cargo clippy`) once Rust crates exist.
- Add tests for logic and regression fixes.
- Include manual test steps for UI, compositor, hardware, and image behavior that cannot be automated.
- Never commit credentials, private keys, personal data, or machine-specific secrets.
- Keep privileged code minimal; do not run the shell or applications as root.
- Do not introduce telemetry without explicit, documented user consent.

## Pull requests
A good PR includes:
- purpose and scope;
- implementation summary;
- tests and exact commands/results;
- screenshots or recordings for UI changes when useful;
- performance measurements for rendering/resource changes;
- known limitations and follow-up work;
- documentation updates.

Avoid claiming a feature is complete if it is a mockup, stub, or only works in one untested environment.

## Commit messages
Use concise, descriptive messages, for example:
- `docs: establish project blueprint`
- `feat(compositor): add initial output handling`
- `fix(shell): restore panel after restart`
- `test(theme): reject invalid profile imports`

## Reporting issues
Include OS/host details, reproduction steps, expected and actual behavior, relevant logs, and whether the issue is reproducible. Remove secrets and sensitive information from logs before posting.
