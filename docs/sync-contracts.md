# Sync Contracts

> **Reference doc, not an issue.** The numbered `NNN-*.md` files are the issue
> tracker; this is a standing checklist like [AGENTS.md](../AGENTS.md).

Owlerlay drives **three frontends** off one Rust backend, and they share data
shapes that are **mirrored by hand** — there is no codegen. Change one side of a
contract without updating its mirrors and a frontend silently drifts (parse
errors, missing fields, stale UI).

The three frontends:

1. **Control UI** — Svelte/TS desktop window (`src/`), talks via Tauri IPC.
2. **OBS overlay** — server-rendered Jinja + SSE (`src-tauri/templates/overlay/`).
3. **Phone remote** — token-gated web page (`src-tauri/templates/remote/remote.html`).

**How to use this doc:** before editing any "source of truth" file below, open
its mirrors in the same change. This is a pointer map on purpose — it does *not*
re-list every field (that would just become a fourth copy to drift). The source
files are the spec; this table tells you who else has to follow.

| # | Contract | Source of truth | Mirrors that must follow |
|---|----------|-----------------|--------------------------|
| 1 | `CountdownState` (`Idle`/`Running`/`Paused`/`Finished`) | `src-tauri/src/countdown/model.rs` | `src/features/countdown/model/countdown.types.ts` (`CountdownState`); every SSE consumer that switches on state |
| 2 | `OverlayConfig` fields (serde `camelCase`) | `src-tauri/src/overlay/model.rs` | `countdown.types.ts` (`OverlayConfig`); overlay templates that read each field — `src-tauri/templates/overlay/countdown/*.j2` |
| 3 | `TimeFormat` (serde `lowercase`: `auto`/`dhms`/`hms`/`ms`/`s`) | `src-tauri/src/overlay/model.rs` | `countdown.types.ts` (`TimeFormat`); the format logic lives Rust-side (`TimeFormat::format`) so templates just render its output |
| 4 | `Group` / `Layout` + `hide_idle` | `src-tauri/src/overlay/model.rs` | `src/features/overlay/model/group.types.ts` (`GroupDto`, `Layout`) |
| 5 | `CountdownSnapshotDto` (snake_case JSON) | `src-tauri/src/countdown/dto.rs` | `countdown.types.ts` (`CountdownSnapshotDto`); phone-remote list rendering in `remote.html` |
| 6 | **Overlay SSE** event names + payload keys: `countdown-tick`, `countdown-state`, `reload` | `src-tauri/src/server/routes.rs` (`sse_group`) | `src-tauri/templates/overlay/countdown/countdown.js.j2` (the `addEventListener` names + the keys it reads) |
| 7 | **Remote SSE** event names: `tick`, `state`, `changed` (note: **different names** from overlay) | `src-tauri/src/server/remote.rs` (`remote_sse`) | `src-tauri/templates/remote/remote.html` (`addEventListener` names + keys) |
| 8 | HTTP route paths (`/overlay`, `/sse/group/{id}`, `/remote`, `/api/remote/*`) | `src-tauri/src/server/mod.rs` + `routes.rs` + `remote.rs` | hard-coded URLs in `countdown.js.j2` and `remote.html` |
| 9 | Remote token scheme (`?t=<hex>` query **or** `Authorization: Bearer <hex>`) | `src-tauri/src/server/remote.rs` (`require_token`) | token extraction in `remote.html` |
| 10 | **Tick interval ↔ CSS sweep**: `TICK_INTERVAL = 250ms` | `src-tauri/src/countdown/commands.rs` (`TICK_INTERVAL`) | progress-bar `transition` must match the tick: `countdown.css.j2:~35` **and** `browsersource.html.j2:~53` (two places, both `0.25s`/`250ms` linear) |

## Why these are easy to get wrong

- **#6 vs #7** — the overlay and the remote use *different SSE event names* for
  the same underlying backend events. Renaming on one side does not break the
  other, so a half-rename ships green and breaks one frontend.
- **#10** — the tick interval and the CSS transition are coupled by *value*, not
  by code. Change the ticker to 100ms and the bar still sweeps over 250ms,
  lagging a quarter second behind. Two CSS sites must both change. (See
  `docs/008-live-tick-followups.md` for the standing note on this coupling.)
- **#2** — `OverlayConfig` uses `#[serde(default)]`, so a missing field
  deserializes silently instead of erroring. A field added in Rust but not in
  the TS type won't crash the backend; it just goes un-set from the UI.

## When this list itself drifts

This doc is hand-maintained. After adding a new cross-frontend contract (a new
shared enum, SSE event, route, or persisted shape), add a row here in the same
change. The graphify wiki (`graphify-out/wiki/`) is the navigation layer that
*finds* these flows; this doc is the checklist that *guards* them.
