# 002 — Persist overlay config (countdown design)

**Type:** Bug  
**Reported by:** Tester  
**Priority:** High  
**Affects:** Per-countdown appearance / overlay config  
**Status:** Done (2026-06-29) — implemented and archived. All three
reported symptoms (restart loss, view-switch loss, design-lost-on-restart)
are fixed in one change set.

## Problem

Three related symptoms that all trace to "the frontend has no way to read
back what it wrote":

1. **Design resets on view-switch** — switching from a countdown to a group
   and back loses the countdown's appearance settings.
2. **Design lost on restart** — overlay config is in-memory only on both
   backend and frontend.
3. (Implicit) Any future widget that wants to _display_ the styling (e.g.
   the previews in 006/007) currently has no backend source of truth to
   read from.

## Root cause

### Restart loss

`OverlayConfig` lives in `OverlayService.configs: Mutex<HashMap<u64,
OverlayConfig>>`, which starts empty on boot. No save/load path exists.

### View-switch loss

`AppearancePanel.svelte` stores settings in a **component-local** `$state`
object (`overlaySettings: Record<number, OverlaySettings>`). This survives
within a session as long as the component stays mounted, but:

- The panel is only rendered when `subject === "countdown"`.
- Switching to a group unmounts the panel → component state is destroyed.
- Switching back remounts with fresh defaults.

The frontend never loads config back from the backend — `set_overlay_config`
is write-only (UI → backend), there is no `get_overlay_config` call on mount.

**Fix is unified:** adding the persistence + the `get_overlay_config` IPC +
having `AppearancePanel` re-hydrate from the backend on mount fixes BOTH
restart loss and view-switch loss in one change. After the fix, the
component-local dict is just the in-flight editing buffer; the source of
truth is the backend (which is itself backed by `overlays.json` on disk).

## Relevant code

| File                                                                 | What                                                                        |
| -------------------------------------------------------------------- | --------------------------------------------------------------------------- |
| `src-tauri/src/overlay/model.rs` L81-128                             | `OverlayConfig` struct (16 fields, derives `Default`)                       |
| `src-tauri/src/overlay/service.rs` L96-107                           | In-memory `set_config` / `get_config` (must become persistent)              |
| `src-tauri/src/overlay/commands.rs` L57-66                           | `set_overlay_config` Tauri command (must save after mutation)               |
| `src-tauri/src/app_state.rs` L70                                     | `OverlayService::new()` (must accept persisted groups + configs)            |
| `src-tauri/src/lib.rs` L43-56                                        | `setup` hook (must `overlay::store::load` alongside the countdown store)    |
| `src-tauri/src/countdown/store.rs`                                   | Reference impl — atomic write, `.json.corrupt` quarantine, `spawn_blocking` |
| `src-tauri/src/settings.rs`                                          | `write_atomic`, `local_data_file` helpers                                   |
| `src/features/countdown/components/AppearancePanel.svelte` L139-185  | Component-local `$state` (must hydrate from backend)                        |
| `src/features/countdown/api/countdown.client.ts`                     | Where the new `getOverlayConfig` wrapper lives                              |
| `src/features/countdown/state/countdown.store.ts` / `group.store.ts` | Where the warm-up fetch on `AppShell.onMount` was originally planned        |

## Scope

### Backend

1. Add `src-tauri/src/overlay/store.rs` mirroring `countdown/store.rs`:
   - `load(handle) -> (Vec<GroupDto>, HashMap<u64, OverlayConfig>)` from
     `<app_local_data_dir>/overlays.json` via `settings::local_data_file`;
     same `countdowns.json.corrupt` quarantine on parse failure.
   - `save(handle, &groups, &configs)` — fire-and-forget
     `spawn_blocking` + atomic write.
   - On-disk shape: a single `OverlaysDto { groups, configs }` so
     `next_id` derivation stays a one-stop change.
2. New `OverlayService::from_groups_and_configs(dto, next_id) -> Self`
   that mirrors `CountdownService::from_dtos` (derives `next_id` as
   `max(group.id)+1`).
3. Wire `overlay::store::save` into every mutating overlay command
   (`group_create`, `group_update`, `group_delete`, `set_overlay_config`)
   — same fire-and-forget pattern as the countdown commands.
4. Wire `overlay::store::load` into the `setup` hook in `lib.rs` and
   pass the loaded groups + configs through a new `AppState::new`
   parameter.
5. Add `get_overlay_config(id) -> OverlayConfig` Tauri command (exposes
   the existing `OverlayService::get_config`); register it in
   `lib.rs::generate_handler!`.

### Frontend

6. Add `getOverlayConfig(id: number): Promise<OverlayConfig>` to the
   countdown client, mirroring the existing `invokeCommand` pattern.
7. (Deferred — see "Frontend deviation" below.) Originally: in
   `AppShell.svelte`'s `onMount`, after `loadGroups()` resolves, walk
   every group member and call `getOverlayConfig` for each so the cache
   is warm before any panel mounts. Cache lives alongside the countdown
   store (or a thin new `overlayStore` — design choice while
   implementing, no separate spec needed).
8. In `AppearancePanel.svelte`, replace the empty-local-state fallback
   with: `$effect` on `id` → call `getOverlayConfig(id)` → seed local
   state from the response → only fall back to `DEFAULT_SETTINGS` when
   the backend has nothing stored (still safe because `OverlayConfig:
Default` fills in any missing field).

## Design decision

Should overlay configs live in the same file as groups (`groups.json` →
`overlays.json` containing both) or their own file? **Same file** —
they're both overlay-module data and always small. Mirrors the
"one store per widget" rule in AGENTS.md.

## Out of scope

- Wiring the remaining `OverlayConfig` fields not yet exposed in the UI
  (progress bars, dividers, borders, etc.) — that's roadmap item 1.
- Lifting the local dict into a Svelte store as an alternative to
  backend hydration — the backend round-trip is required anyway for
  restart, so doing both is wasted work.

## Verification

- Configure a countdown's appearance, switch to a group, switch back →
  settings preserved.
- Configure appearance, restart app → settings restored.
- Two countdowns with different configs → each restores independently.

## Implementation (2026-06-29)

All three reported symptoms trace to the same root cause (the frontend
had no way to read back what it wrote) and are fixed in a single,
unified change set.

### Backend

- **Persistence**: `src-tauri/src/overlay/store.rs` (new) — `load` /
  `save` against `<app_local_data_dir>/overlays.json`. Single file
  holds both groups and per-countdown configs so a group rename can
  never persist without its members' configs (or vice versa).
  Same `.json.corrupt` quarantine as `countdown/store.rs`. Fire-
  and-forget `spawn_blocking` writes via `settings::write_atomic`.
- **DTO**: `src-tauri/src/overlay/dto.rs` gains `GroupsAndConfigsDto`
  - `#[serde(transparent)] OverlayConfigMapEntry` so configs land at
    `configs["<id>"]` and not `configs["<id>"] = { "value": ... }`.
- **Service**: `OverlayService::from_groups_and_configs(...)` (with
  `saturating_add` `next_id`, duplicate-id dedup) +
  `OverlayService::snapshot()` for the save path.
- **Commands**: every mutating overlay command (`group_create`,
  `group_update`, `group_delete`, `set_overlay_config`) now calls
  `overlay::store::save` after the in-memory mutation, via the shared
  `save_overlays(app, state)` helper.
- **New command**: `get_overlay_config(id) -> OverlayConfig`. The
  existing `OverlayService::get_config` already returns `Default` when
  nothing is stored — that fall-through behaviour is what lets the
  frontend treat it as "never fails, always returns a config".
- **AppState**: `AppState::new` now takes the persisted groups + configs
  and seeds `OverlayService` via the new constructor.
- **lib.rs setup**: `overlay::store::load` runs alongside the
  countdown store in the `setup` hook.

### Frontend

- **Client**: `src/features/countdown/api/countdown.client.ts` gains
  `getOverlayConfig(id)` (mirrors the existing `invokeCommand`
  pattern).
- **Types**: `"get_overlay_config"` added to the `CountdownCommand`
  union so the contract stays accurate.
- **`AppearancePanel.svelte`**: new `hydrated: Record<id, OverlaySettings>`
  populated by a `$effect` that watches `id` and re-fetches on every
  (re-)mount. `getSettings(n)` now falls back
  `overlaySettings[n] ?? hydrated[n] ?? DEFAULT_SETTINGS` — local
  edits take precedence while the panel is mounted, the hydrated
  snapshot takes over after unmount/remount, and the very first
  ever visit still shows `DEFAULT_SETTINGS` until either the user
  edits or the backend returns a stored config.
- **Inverse parsing helpers**: `parseRem`, `parseBorderWidth`,
  `parseBorderColor` on the panel — exact inverses of `toConfig`
  so the round-trip is lossless even with the existing
  `DEFAULT_SETTINGS`-spread composition.

### Frontend deviation from spec (deliberate)

The warm-up fetch across all group members in `AppShell.onMount`
(spec step 7) was **not** implemented in this slice. The per-panel
`$effect` already guarantees correctness for both restart and
view-switch loss, and a shared overlay cache store would add a
new file + duplicated parse logic + a separate freshness story
for the dashboard/previews coming in 004/006/007. A block comment
in `AppShell.svelte` records the reasoning and points at the
right moment to introduce the cache (when 004 lands).

### Test results

- `cargo test --manifest-path src-tauri/Cargo.toml` → all green
  (15/15 in `overlay_service`, 10/10 in `countdown_persist`,
  4/4 in `countdown_finish`, 4/4 in `countdown_model`, 2/2 in
  `time_format`, 1/1 in `overlay_render`).
- `cargo build --manifest-path src-tauri/Cargo.toml --release` → clean.
- `pnpm check` → 0 errors, 0 warnings.
- `pnpm format:check` → only the pre-existing
  `countdown.html.j2` warning remains (unrelated to this slice).

### Files changed

| File                                                       | Change                                                                             |
| ---------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| `src-tauri/src/overlay/store.rs` _(new)_                   | Persistence layer                                                                  |
| `src-tauri/src/overlay/dto.rs`                             | New DTO + transparent newtype                                                      |
| `src-tauri/src/overlay/service.rs`                         | Restore ctor + snapshot helper                                                     |
| `src-tauri/src/overlay/mod.rs`                             | Export `store`                                                                     |
| `src-tauri/src/overlay/commands.rs`                        | `save_overlays` helper wired into every mutating cmd; new `get_overlay_config` cmd |
| `src-tauri/src/app_state.rs`                               | Accept persisted groups + configs in `new`                                         |
| `src-tauri/src/lib.rs`                                     | `setup` hook loads `overlays.json`; register new command                           |
| `src-tauri/tests/overlay_service.rs`                       | 9 new tests (see 001 for the list)                                                 |
| `src/features/countdown/api/countdown.client.ts`           | `getOverlayConfig(id)`                                                             |
| `src/features/countdown/model/countdown.types.ts`          | `"get_overlay_config"` in `CountdownCommand` union                                 |
| `src/features/countdown/components/AppearancePanel.svelte` | Hydrate from backend on mount; new parse helpers                                   |
| `src/app/shell/AppShell.svelte`                            | `await loadGroups()`; deferred-warm-up note comment                                |
