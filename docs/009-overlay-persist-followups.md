# 009 — Overlay-persistence review follow-ups

Deferred findings from the high-effort code review of the overlay-persistence
work (specs 001 + 002, shipped via PR #15). Each was a deliberate "defer" call
during review — real but low-value or not reachable today. Tracked here so they
aren't lost; fix opportunistically.

## 1. Concurrent-save ordering (deferred — matches countdown precedent)

`overlay::commands::save_overlays` snapshots the full overlay state and spawns a
fire-and-forget `spawn_blocking(write_atomic)` on every mutating command, with no
serialization between saves. Two near-simultaneous mutations (e.g. a desktop
`set_overlay_config` and a future LAN-remote `group_update`) each snapshot then
spawn their write; if the earlier-snapshotted (staler) write's rename completes
last, `overlays.json` briefly reflects the older state until the next mutation
rewrites it.

- **Why deferred:** identical pattern to `countdown::store::save` (accepted
  precedent), and the LAN remote does not mutate overlay config today — there is
  no second writer yet. Each write is still atomic (temp + rename); only the
  *ordering* of competing writes is unguarded.
- **Fix when it bites:** serialize saves behind a single writer task (channel +
  one consumer), or coalesce rapid saves with a short debounce. Applies to the
  countdown store too — fix both together.

## 2. Border parse regex is output-shaped only (deferred — hand-edit only)

`AppearancePanel`'s `parseBorderWidth` / `parseBorderColor` only match the literal
`"<n>px solid <color>"` string that `toConfig` emits. `toConfig` writes `"none"`
when width is 0 (parses correctly to 0), but any *other* persisted border string
(hand-edited file, or a future border format) silently yields width 0 — and the
next save then persists `"none"`, erasing the border.

- **Why deferred:** the app only ever writes the exact shape these regexes match;
  this is reachable only via a hand-edited `overlays.json`.
- **Fix when it bites:** parse defensively (split on whitespace, validate units)
  or store border width/style/color as separate `OverlayConfig` fields instead of
  one composite CSS string.

## 3. Double-clone per overlay save (deferred — tiny payload)

`OverlayService::snapshot` clones the whole `configs` map out from behind its lock
(`configs.lock().await.clone()`), then `GroupsAndConfigsDto::from_parts` clones
every `OverlayConfig` a second time into the `BTreeMap`. Each save copies the
configs twice.

- **Why deferred:** configs are small and saves are infrequent (one per user
  edit), so the cost is negligible; the borrow-based `save(&[..], &HashMap)`
  signature mirrors the countdown store.
- **Fix when it bites:** have `snapshot` return owned values straight into a DTO
  (move, don't re-clone), or change `store::save` to take owned `groups`/`configs`.

## 4. One-shot hydration guard (deferred — no out-of-band writer today)

`AppearancePanel`'s hydration `$effect` guards with `if (hydrated[id]) return`, so
it fetches the persisted config exactly once per id and never refreshes it. If the
config for that id ever changed out-of-band (a future SSE/remote-driven config
update) while the panel already cached `hydrated[id]`, the panel would keep showing
the stale first snapshot and could re-save over the out-of-band change.

- **Why deferred:** no live path mutates an overlay config outside this panel
  today, so the cache is never stale in practice.
- **Fix when it bites:** when a remote/SSE config-changed signal lands, invalidate
  `hydrated[id]` (and re-run the effect) for the affected id.
