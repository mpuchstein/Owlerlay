# Repository Guidelines

## Vision

**Owlerlay** is a Tauri-based **OBS overlay control center** for streamers. It
is a desktop app that manages on-screen widgets (countdowns, overlay groups,
more on the roadmap) and serves them to OBS browser-sources over a local
HTTP/SSE server. The countdown is the first widget; **overlay groups** (named
sets of countdowns rendered as one OBS browser source) and a **LAN phone
remote** are also shipped.

**Goals:**
- Control center for OBS browser-source overlays, with live, flicker-free updates.
- Low resource consumption, cross-platform.
- Extensible toward a plugin-style feature set.

---

## Architecture

A Tauri 2 desktop app: Svelte/TypeScript frontend + Rust backend, plus a local
Axum web server that renders and serves overlays to OBS and exposes a token-
guarded LAN control API.

**Frontend — `src/`** (Svelte 5 runes, TypeScript strict, Vite 7, bespoke CSS)
- **No UI framework.** Styling is the "Night-Owl Control Room" theme — design
  tokens live in `src/app/styles/` (no PicoCSS / Tailwind / etc.).
- Self-hosted variable fonts via `@fontsource-variable/*` (Spline Sans Mono,
  Hanken Grotesk, Bricolage Grotesque, Quicksand, Fraunces).
- Organized by feature in `src/features/`:
  - `countdown/` — `api/` (Tauri command wrappers), `model/` (types + DTO
    mappers), `state/` (Svelte stores), `components/`.
  - `overlay/` — overlay-group manager UI.
  - `remote/` — phone-remote settings panel + QR.
- Cross-cutting code in `src/shared/` (generic `invoke` wrapper, duration
  helpers, the `localhost:7420` overlay origin used only by the desktop
  control UI to talk to its own machine).
- App shell in `src/app/shell/` (`AppShell.svelte`, `Roost` = sidebar,
  `Stage` = main pane, plus `OwlMark`/`EyePip` brand bits).

**Backend — `src-tauri/src/`** (Rust Edition 2024, Tauri 2)
- `lib.rs` — Tauri builder, plugin setup, `invoke_handler` registration,
  and the `RunEvent::Ready` hook that spawns the Axum server and ticker.
- `app_state.rs` — shared `AppState` (clock anchor, countdown service,
  **overlay service**, broadcast event bus, app handle, remote enable flag
  and capability token). `ClockAnchor` translates between `tokio::time::Instant`
  and wall-clock epoch ms so running countdowns survive a restart.
- `countdown/` — `model.rs` (Idle→Running→Paused→Finished state machine),
  `service.rs` (in-memory store, 100ms ticker), `commands.rs`, `dto.rs`,
  `events.rs`, `errors.rs`, `store.rs` (persistence).
- `overlay/` — overlay groups + per-countdown `OverlayConfig` (time format,
  font, colors, progress bar, container chrome, layout), `service.rs`,
  `commands.rs`, `dto.rs`, `errors.rs`, `model.rs`.
- `remote.rs` — Tauri commands backing the Phone-Remote settings panel
  (read settings, enable, regenerate token).
- `settings.rs` — persisted config + capability token. **Two files, split
  deliberately** so the config can be tracked (e.g. chezmoi) without
  leaking the secret:
  - `<app_config_dir>/config.json` — `remote_enabled` only.
  - `<app_local_data_dir>/remote_token` — the 256-bit capability secret.

**Overlay server — `src-tauri/src/server/`** (single Axum app on `:7420`)
- `mod.rs` — router wiring; opt-in LAN bind (`0.0.0.0`) only when
  `remote_enabled`, otherwise loopback-only. CORS is GET-only, `Any` origin
  (read paths only; the remote page is same-origin so its POSTs are not
  subject to CORS).
- `routes.rs` — `/api/icons`, `/overlay?group=<id>` (renders Jinja2),
  `/sse/group/<id>` (SSE), `/static/...` (served from `public/`).
- `remote.rs` — `/remote` page (mobile control) + POST control routes,
  all guarded by the capability token.
- Templates: `src-tauri/templates/overlay/` (Jinja2) and
  `src-tauri/templates/remote/` (static mobile control page).
- Rendering uses `minijinja` with `strict_undefined=True`; templates ship
  minimal JS — just an `EventSource` listener.
- Overlay pages set `Cache-Control: no-store` so config changes re-render
  instantly.

**Data flow:**
UI `invoke()` → Tauri command → `CountdownService` / `OverlayService` mutates
state → broadcast event bus → (Tauri events back to the UI) **and** (SSE out
to OBS overlays). The LAN remote POSTs hit `server::remote::router`, which
calls the same `AppState` services so the desktop panel and OBS overlay
update live.

**Debug-only MCP bridge** (`tauri-plugin-mcp-bridge`, bound to `127.0.0.1`).
Lets an AI agent drive the native webview (keyboard, screenshots, IPC) for
UI testing. Gated by `#[cfg(debug_assertions)]` in `lib.rs`; the crate stays
in `Cargo.toml` for now — move behind an optional `mcp` feature if release
bloat ever matters.

---

## Roadmap

**Shipped:** widget storage / persistence — countdowns persist across
restarts in `countdowns.json` (v0.4.0); overlay groups + per-countdown
`OverlayConfig` persist in `overlays.json` (v0.4.1), with running `Instant`s
restored via `ClockAnchor`. See `docs/archive/001`, `docs/archive/002`.

Ordered, near-term first:

1. **Alarms / scheduled events** — time-of-day triggers, not just countdowns.
2. **Twitch integration** — react to Twitch events (subs/follows/points) to
   drive overlays and timers.
3. **Alerts / notifications** — on-screen alert overlays for OBS (e.g. new
   follower).
4. **Companion (avatar)** — scope TBD, next-up not now. Basis is the proof at
   `~/Dev/3D_Models/AndreIsohedronCephalon/`: a Three.js audio-reactive 3D
   avatar with in-browser Kokoro TTS. Becomes a responsive companion for
   voice/alert animations.

### Persistence rules

Two rules keep the per-widget JSON store (the countdown + overlay stores)
tiny — sub-millisecond
saves — through every later widget, so it never needs a binary format or a
database:

1. **Persist config + coarse run-state, never the high-frequency stream.**
   Transient runtime data — the Twitch event firehose, the avatar's per-
   frame audio reactivity, the 100ms countdown tick — must not route through
   the save path. Only user-driven config/state changes persist.
2. **Store media/3D/audio by path reference, never embedded.** A base64 blob
   in the JSON store bloats it catastrophically; a path string stays ~50
   bytes.

---

## Working agreement (Hybrid)

This was a learning-only repo; it is now built collaboratively in a
**Hybrid** model.

- The assistant **may** write and modify any code (frontend, Rust, tests).
- For non-trivial work, the assistant proposes a brief plan first; the owner
  decides who implements ("you do it" / "I'll do it" — the owner may claim
  parts they want to learn). When unspecified, default to: assistant plans,
  owner picks.
- The assistant must read the current, on-disk source before answering
  technical questions or reviewing — never rely on memory from earlier
  turns.
- The assistant may edit documentation any time it's helpful.
- Active user-testing issues and the implementation order live in `docs/`
  (each numbered spec + a `docs/README.md` dependency graph). Read
  `docs/README.md` before claiming an item is "next" or "done".

---

## Project Structure & Module Organization
- `src/`: Svelte/TS frontend (feature folders + `src/shared/`, `src/app/`).
- `src-tauri/src/`: Rust entry points, Tauri commands, state, overlay server.
- `src-tauri/templates/overlay/`: Jinja2 overlay templates.
- `src-tauri/templates/remote/`: static mobile phone-remote page.
- `src-tauri/icons/`: app icons for desktop bundles.
- `src-tauri/tauri.conf.json`: Tauri app/build configuration.
- `src-tauri/capabilities/`: Tauri capability ACLs.
- `src-tauri/gen/`: Tauri-generated code (do not edit).
- `public/`: assets served at `/static/` (fonts live here).
- `docs/`: active issue tracker + per-issue specs.
- `dist/` and `src-tauri/target/`: build outputs (generated; do not edit).

## Build, Test, and Development Commands
- `pnpm install`: install JavaScript dependencies.
- `pnpm dev`: Vite web dev server at `http://localhost:1420`.
- `pnpm tauri dev`: run the full app in development mode.
- `pnpm build`: type-check (`tsc`) and build frontend into `dist/`.
- `pnpm check`: `svelte-check` (against `tsconfig.app.json`) +
  `tsc -p tsconfig.node.json`.
- `pnpm format` / `pnpm format:check`: Prettier write / verify. The
  `prettier-plugin-jinja-template` is wired up via `.prettierrc.json` to
  format `*.j2`; only `*.ts`, `*.svelte`, `*.css`, and the non-ignored
  `*.j2` files in `src-tauri/templates/` are in scope. `.prettierignore`
  excludes the browser-source HTML/CSS/JS templates from formatting.
- `pnpm tauri build`: create desktop bundles.
- `pnpm preview`: preview the frontend bundle.
- `cargo test  --manifest-path src-tauri/Cargo.toml`: run Rust integration
  tests in `src-tauri/tests/`.
- `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings`
- `cargo fmt   --manifest-path src-tauri/Cargo.toml`

## Coding Style & Naming Conventions
- TypeScript/Svelte: formatted by Prettier (`prettier-plugin-svelte` +
  `prettier-plugin-jinja-template`, 2-space) — run `pnpm format` before
  committing. `strict` mode; prefer explicit types at API boundaries.
  `camelCase` vars/functions, `PascalCase` types.
- Rust: Edition 2024, `rustfmt` defaults (4-space), `snake_case`
  functions/modules. Run `cargo fmt` + `cargo clippy -D warnings` before
  committing.
- Keep Tauri commands small and side-effect focused; register them in
  `lib.rs` (`tauri::generate_handler![...]`) **and** in the `commands.rs`
  use line at the top of `lib.rs`.
- Name files by feature; the four-folder split
  (`api/` / `model/` / `state/` / `components/`) inside a frontend feature
  is the established convention.
- Tauri services live in `service.rs`; commands in `commands.rs`; DTO
  mappers in `dto.rs`; types and the state machine in `model.rs`. Mirror
  that layout when adding a new widget.

## Testing Guidelines
No JS test framework is configured yet. For new features:
- add Rust tests in `src-tauri/tests/` (integration tests against each
  module's public API — keep them out of `src-tauri/src/`, no
  `#[cfg(test)]` blocks inside source files);
- add frontend tests only for non-trivial UI/state logic (Vitest preferred
  when introduced);
- include manual verification steps in PRs (OS, command run, expected
  behavior).
- Existing Rust test files (in `src-tauri/tests/`): `countdown_model`,
  `countdown_finish`, `countdown_persist`, `overlay_service`,
  `overlay_render`, `remote_settings`, `time_format`. Add new ones beside
  them following the same integration-test style (use the `owlerlay_lib`
  crate via `use owlerlay_lib::…`).

## Commit & Pull Request Guidelines
Use Conventional Commits:
- `feat: add tray menu action`
- `fix: handle empty greet input`

PRs should include a short problem/solution summary, linked issues when
relevant, screenshots/recordings for UI changes, and the exact verification
commands run.

## Releases

Two remotes: **origin** = Forgejo (`somegit.dev`, driven by `tea`); **mirror**
= GitHub (driven by `gh`). The mirror's `main` is only synced at release time.

1. Bump the version in `package.json`, `src-tauri/Cargo.toml`,
   `src-tauri/tauri.conf.json`, and `src-tauri/Cargo.lock` (run `cargo build`
   to sync the lock). Land it via a `chore(release): vX.Y.Z` PR — don't push
   `main` directly.
2. Push `main` to **both** remotes, then create and push an annotated tag
   `vX.Y.Z` to **both**. The tag triggers `.github/workflows/release.yml` and
   `.gitea/workflows/release.yml` (both fire on `tags: ['v*']`).
3. Create the release object with notes (credit contributors) on each:
   `gh release create vX.Y.Z -R <repo> --notes-file …` and
   `tea release create --tag vX.Y.Z --note …`. Do this right after the tag
   push, before the ~8-minute build finishes.
4. Each build runs `tauri-action` with `tagName`, so it **attaches the
   individual installers** (`.deb`/`.rpm`/`.AppImage`, `.msi`/`.exe`) to the
   release for that tag automatically — no manual artifact download/upload.

If a build can't attach assets (e.g. the Forgejo path needs verifying on a new
release), fall back to uploading them by hand from the workflow run:
`gh run download <id>` then `gh release upload` / `tea releases assets create`.

## Known cleanup backlog
Tracked, not yet done (fix opportunistically):
- About page is a one-line stub (`src/app/shell/AppShell.svelte`).
- No sanitization of overlay config strings into Jinja2 (low risk,
  local-only).
- `svelte.config.js` sets `emitCss: false` to suppress the cold-start
  "failed to load virtual css module" / postcss "Unknown word" errors, but
  they still appear in the `pnpm tauri dev` log (the UI renders fine on the
  served request). Verify the option is actually taking effect, or drop the
  file if it isn't.
- `src-tauri/templates/overlay/countdown/countdown.html.j2` fails
  `pnpm format:check` (pre-existing). Either format it or add it to
  `.prettierignore` alongside the other browser-source templates.
