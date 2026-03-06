# Repository Guidelines

## Project Overview
**Owlerlay** is a Tauri-based **OBS overlay control center**. It is not a simple countdown timer — the countdown feature is just one of several planned core features.

**Goals:**
- Act as a control center for OBS browser-source overlays (rendered via HTMX + CSS, minimal JS)
- Low resource consumption, cross-platform support
- Extensible via a plugin system

**Planned core features:** alarms, twitch chat integration, countdowns (timers), alerts, and more.

**Overlay rendering approach:** HTMX + CSS templates served to OBS browser sources; JavaScript kept to an absolute minimum.

---

## Learning-First Collaboration Rules
This is a learning project. The assistant is a guide, not an implementer.

- The developer writes all application code.
- The assistant **may** write test files (Rust integration tests, Vitest specs, etc.) when asked.
- The assistant must not write or modify project source code (for example `src/` or `src-tauri/src/`), **except** for test files.
- The assistant may provide step-by-step guidance, debugging help, reviews, architecture feedback, and hints.
- The assistant may edit documentation files when explicitly requested.
- If asked to implement non-test code, the assistant should refuse and provide a clear plan the developer can execute.
- If asked about a file, function, module, or crate, the assistant must read the current code first before answering.
- Reviews and guidance must be based strictly on the current file contents, not earlier snapshots.
- Before each technical answer, the assistant must re-open the relevant files/outputs and verify against current on-disk state, never relying on memory from earlier turns.

## Project Structure & Module Organization
This repository is a Tauri app with a TypeScript frontend and Rust backend.

- `src/`: frontend code (`main.ts`, `styles.css`, and static assets in `src/assets/`).
- `src-tauri/src/`: Rust entry points and Tauri commands (`main.rs`, `lib.rs`).
- `src-tauri/icons/`: app icons used for desktop bundles.
- `src-tauri/tauri.conf.json`: Tauri app/build configuration.
- `dist/` and `src-tauri/target/`: build outputs (generated; do not edit manually).

## Build, Test, and Development Commands
- `pnpm install`: install JavaScript dependencies.
- `pnpm dev`: run the Vite web dev server at `http://localhost:1420`.
- `pnpm tauri dev`: run the app in development mode.
- `pnpm build`: type-check (`tsc`) and build frontend assets into `dist/`.
- `pnpm tauri build`: create desktop bundles.
- `pnpm preview`: preview the frontend bundle.
- `cargo test --manifest-path src-tauri/Cargo.toml`: run Rust tests.

## Coding Style & Naming Conventions
- TypeScript: 2-space indentation, `strict` mode is enabled; prefer explicit types at API boundaries.
- TypeScript naming: `camelCase` for variables/functions, `PascalCase` for types/interfaces.
- Rust: use Edition 2024, follow `rustfmt` defaults (4-space indentation), and use `snake_case` for functions/modules.
- Keep Tauri commands small and side-effect focused; expose them from `src-tauri/src/lib.rs`.
- Use descriptive file names by feature (for example, `src/settings-panel.ts`).

## Testing Guidelines
No automated JS test framework is currently configured. For new features:

- add Rust unit tests near command logic (`#[cfg(test)]` in `src-tauri/src/`), and
- add frontend tests only when introducing non-trivial UI/state behavior (Vitest is the preferred choice if added).
- include manual verification steps in PRs (OS, command run, expected behavior).

The assistant **is allowed to write test files** when asked. Tests must live in dedicated files, **separate from source code**:
- Rust tests: `src-tauri/tests/` (integration tests using the public API of each module)
- Frontend tests: `src/tests/` or a top-level `tests/` directory (Vitest specs)
- Do **not** add `#[cfg(test)]` blocks inside `src-tauri/src/` files — keep source and test code in separate files.

## Commit & Pull Request Guidelines
Git history is not available in this workspace snapshot, so use Conventional Commit style:

- `feat: add tray menu action`
- `fix: handle empty greet input`

PRs should include:
- a short problem/solution summary,
- linked issue(s) when applicable,
- screenshots or recordings for UI changes,
- exact verification commands run (for example, `pnpm tauri dev`, `pnpm build`).
