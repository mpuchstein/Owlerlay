# Owlerlay — Issue Tracker

Scoped issues from user testing (2026-06-29). Open issues live at the
top of this folder; completed (and now-archived) issues live under
[`archive/`](archive/) so the open list stays focused on what remains.

## Bugs

| # | Title | Status |
|---|---|---|
| [archive/001](archive/001-persist-groups.md) | Persist groups across restarts | Done (2026-06-29) |
| [archive/002](archive/002-persist-overlay-config.md) | Persist overlay config + frontend hydration (also fixes the view-switch bug) | Done (2026-06-29) |

## Features

| # | Title | Priority | Status | Tag |
|---|---|---|---|---|
| [003](003-auto-reset.md) | Auto-reset on countdown finish | Medium | Open | backend |
| [004](004-dashboard.md) | Dashboard overview in control hub (tiles w/ inline transport) | Medium | Open | frontend-design |
| [005](005-countdown-icon-label.md) | Icons + labels per countdown (4 presets + custom flexbox) | Medium | Open | frontend-design |
| [006](006-preview-per-group.md) | Per-group preview (static Svelte mock from OverlayConfig) | Low | Open | frontend-design |
| [007](007-preview-per-countdown.md) | Per-countdown preview (static Svelte mock) | Low | Open | frontend-design |

## Tech debt

| # | Title | Priority | Status | Tag |
|---|---|---|---|---|
| [008](008-live-tick-followups.md) | Live-tick fan-out follow-ups (tick batching; CSS↔TICK_INTERVAL coupling; Running-guard rationale) | Low | Open | backend/perf |

## Dependency graph (post-001/002)

```
003 (auto-reset) — independent; small Rust-only change
004 (dashboard) — needs persisted groups (001, done)
005 (icon + label + layout) — needs persisted config (002, done); adds shared overlay cache candidate
006, 007 (previews) — read persisted config (002, done); share a PreviewTile.svelte
```

## Implementation order (each step independently mergeable)

1. **001** + **002** — **done** (2026-06-29, archived).
2. **003** — auto-reset on finish. Self-contained backend feature with
   a small frontend toggle; no dependency on the other items.
3. **004** — Dashboard panel (uses the now-persisted groups and configs
   from 001/002; also a natural home for a future shared overlay cache
   store).
4. **005** — icon-label layout presets. New fields on `OverlayConfig`,
   Jinja2 template changes, frontend preset picker, render tests in
   `tests/overlay_render.rs`.
5. **006** + **007** — preview tiles. New shared `PreviewTile.svelte`
   consumed by both `GroupPanel` and `CountdownDetail`.

## Cross-cutting rules (carried forward from the planning session)

- **Persistence shape:** one `overlays.json` file holding both
  `groups` and `configs` (mirrors the single-store-per-widget rule in
  AGENTS.md and the countdown store precedent).
- **Disk location:** `<app_local_data_dir>/overlays.json`, written
  via `settings::write_atomic`, with the same `.json.corrupt`
  quarantine the countdown store uses.
- **Auto-reset semantics** (003, open): natural finish only,
  transitions Finished → Idle in place (no OBS reload). Verifies the
  existing `finished_events` invariants still hold.
- **Layout presets** (005, open): `OverlayLayout` enum with 4 named
  choices plus a `Custom` arm that unlocks the free-form
  flex-direction / wrap / justify-content / align-items fields. Each
  preset must render under strict-undefined (new render test).
- **Shared overlay cache** (deferred from 002 step 7): when 004/005/006/007
  share a need for already-hydrated configs, that is the moment to
  introduce a shared `overlayStore` (or wherever it naturally lives).
  Documented in `AppShell.svelte` as a block comment.
