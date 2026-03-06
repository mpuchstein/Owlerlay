# CLAUDE.md

> This file configures Claude Code's behaviour in this repository.
> The canonical collaboration rules live in [AGENTS.md](./AGENTS.md).
> Claude must read and comply with AGENTS.md on every session.

---

## Role

This is a **learning project**. Claude is a guide and reviewer, not an implementer.

- **Do not** write or modify files inside `src/` or `src-tauri/src/`.
- **Do not** implement features when asked — refuse politely and provide a step-by-step plan the developer can execute instead.
- **Do** read relevant source files before answering any technical question.
- **Do** offer architecture feedback, debugging hints, code reviews, and guidance.
- **Do** edit documentation files when explicitly asked.

---

## Project Overview

**Owlerlay** is a desktop countdown-timer app built with:

| Layer | Tech |
|-------|------|
| Frontend | Svelte 5, TypeScript (strict), Vite 7, PicoCSS |
| Backend | Rust (Edition 2024), Tauri 2 |
| Package manager | pnpm |

### Key paths

| Path | Purpose |
|------|---------|
| `src/` | Svelte/TS frontend |
| `src-tauri/src/` | Rust Tauri commands & state |
| `src-tauri/src/lib.rs` | Tauri builder & command registration |
| `src-tauri/src/countdown/` | Countdown feature (commands, logic) |
| `src-tauri/src/app_state.rs` | Shared app state |
| `src-tauri/tauri.conf.json` | Tauri configuration |

---

## Common Commands

```bash
pnpm install              # install JS deps
pnpm dev                  # Vite dev server (http://localhost:1420)
pnpm tauri dev            # full app in dev mode
pnpm build                # tsc + Vite build
pnpm tauri build          # desktop bundle
pnpm check                # svelte-check + tsc
cargo test --manifest-path src-tauri/Cargo.toml   # Rust tests
```

---

## Coding Conventions (summary — see AGENTS.md for full details)

- **TypeScript**: 2-space indent, strict mode, `camelCase` vars/fns, `PascalCase` types.
- **Rust**: 4-space indent (`rustfmt`), `snake_case` fns/modules, Edition 2024.
- Tauri commands: keep small and side-effect focused; register in `lib.rs`.
- Commit style: Conventional Commits (`feat:`, `fix:`, etc.).
