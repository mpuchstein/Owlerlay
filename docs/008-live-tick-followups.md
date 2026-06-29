# 008 — Live-tick fan-out: follow-ups from the v0.4.1 review

**Type:** Tech debt / Performance
**Source:** code review of PR #11 (live-tick the sidebar rows, 250ms cadence)
**Priority:** Low — perf only bites with many *simultaneously running* timers
**Affects:** countdown ticker (backend), countdown store (frontend), overlay + phone SSE consumers

PR #11 made every running countdown's rail row tick live (the store patches
`items[]` on each `countdown_tick`) and dropped the ticker cadence 100ms → 250ms.
The review surfaced three things: two are deferred work, one is a resolved design
decision recorded so it isn't "fixed" away.

## 1. Backend tick batching (deferred — its own PR)

**Problem.** The ticker emits one `countdown_tick` (`app.emit`) **plus** one
`event_bus.send` *per running countdown per cycle*. The frontend store handler
then runs `s.items.map()` over the whole list *per event* → roughly `O(M·N)`
array allocations and `M` store-subscriber notifications every 250ms (M running
of N total). Negligible at a few timers; only matters as M approaches the cap
(64). There is **no cheap frontend-only win** — Svelte needs a fresh array
reference per update regardless, so the saving must come from emitting fewer
events.

**Fix.** Emit a single **batched** tick event carrying `Vec<{ id, remaining_ms }>`
(plus `percent` for the overlay). The store maps `items[]` once per cycle; the
overlay/phone parse one SSE frame. Collapses `M` emits → 1 and `M` maps → 1.

**Why deferred.** Cross-cutting: `commands.rs::spawn_ticker`, the SSE route,
`countdown.store.ts` tick handler, `templates/overlay/countdown/countdown.js.j2`,
and `templates/remote/remote.html` all speak the per-countdown tick today.
That's a feature-sized change, not a v0.4.1 blocker.

## 2. CSS transition ↔ `TICK_INTERVAL` coupling (deferred — cleanup)

The overlay progress-bar `transition: width 250ms linear`
(`countdown.css.j2`) is hand-synced to `TICK_INTERVAL` (`commands.rs`, 250ms)
by **cross-reference comments only**. Change one without the other and the bar
lags (CSS longer than the tick gap) or stutters (CSS shorter). **Fix:** thread
`TICK_INTERVAL.as_millis()` into the overlay template render context (the
`item_css`/`item_js` render path) so the duration renders from the one Rust
constant, deleting the keep-in-sync invariant. Low value; the comments are an
adequate guard until the next cadence change.

## 3. Running-guard on live-tick (RESOLVED — do not remove)

The store tick handler patches `duration` only when the item's
`state === "Running"`. A reviewer flagged this could "drop a tick" if an
out-of-order `countdown_changed` lands first. **This is intended, not a bug.**
A tick that arrives after a reset/pause is *stale* and must not overwrite the
authoritative value `countdown_changed` just set — otherwise the readout
contradicts the Idle/Paused chip. Dropping that stale tick is correct; the next
legitimate transition restores ticking. Recorded here so the guard isn't deleted
as "redundant."

## Verification (when batching lands)

- One `app.emit` + one SSE frame per cycle regardless of running count.
- Rail still ticks every running row; overlay/phone unchanged visually.
- 64 running countdowns: no measurable rail jank.
