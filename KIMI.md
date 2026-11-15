# Kimi Collaboration Notes — UnnamedOS

Hi Kimi! 👋

You're collaborating on **UnnamedOS**, a project aiming to be **“Light as Linux. Beautiful as Windows 11.”** The owner may give you tasks directly in Kimi chat and then bring your output back to GPT-6 for review, integration, testing, and GitHub commits.

## Project direction

- Build a real, usable Debian-based OS—not just a mockup or concept.
- Aim for a beautiful, modern desktop while keeping a lightweight mode.
- Planned tiers: **Lite / Balanced / Ultra**.
- Be honest about what is implemented, what is only planned, and what has actually been tested.
- The current ISO milestone is a **Stage 1 compositor feasibility prototype**. A successful ISO build does not automatically mean the custom desktop is complete or that the ISO has booted successfully on hardware/VM.

## How to work on tasks

1. Read `README.md` and inspect the existing code before proposing changes.
2. Identify the current stage and the task's scope. Do not skip stages or silently rewrite the project's direction.
3. Prefer small, coherent changes that fit the existing architecture. Avoid unnecessary dependencies and destructive changes.
4. Make implementations real and runnable where possible; don't substitute fake UI, placeholders, or claims of functionality.
5. Include all relevant files, exact paths, setup/build/run commands, and a concise summary of changes.
6. Test your work when possible. Clearly report tests run, results, limitations, and anything the owner must do manually.
7. If you cannot access GitHub directly, provide complete file contents or a clean unified diff. **Do not claim to have committed or pushed anything unless you actually did.**
8. Never expose, request, or hard-code secrets/API keys. Use environment variables or GitHub Actions secrets.
9. Don't require paid AI APIs for ordinary project development. AI integrations are optional; the owner can manually move work between AI assistants.

## Handoff format

For every task, finish with:

- **What changed**
- **Files changed** (exact paths)
- **How to run/build/test**
- **Tests and results**
- **Known issues / next step**
- **Full patch or file contents** if you did not modify the repository directly

GPT-6 will independently review your contribution before it is merged into the project.

Thanks for helping build UnnamedOS! 🚀
