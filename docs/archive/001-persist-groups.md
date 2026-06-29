# 001 — Persist groups across restarts

**Type:** Bug  
**Reported by:** Tester  
**Priority:** High  
**Affects:** Groups (overlay module)  
**Status:** Done (2026-06-29) — implemented and archived.

## Problem

Groups are stored in-memory only (`OverlayService::new()` creates empty
`HashMap`s). On app restart, all groups are lost — the user has to recreate
them from scratch.

## Root cause

No persistence layer exists for the overlay module. Countdown persistence
was added in v0.4.0 (`countdown/store.rs`, `countdowns.json`) but groups
(`overlay/service.rs`) were not included.

## Relevant code

| File                                    | What                                                     |
| --------------------------------------- | -------------------------------------------------------- |
| `src-tauri/src/overlay/service.rs`      | In-memory `HashMap<u64, Group>` + CRUD                   |
| `src-tauri/src/overlay/model.rs` L67-74 | `Group` struct (id, name, members, layout, hide_idle)    |
| `src-tauri/src/overlay/dto.rs`          | `GroupDto` — already has `Serialize`/`Deserialize`       |
| `src-tauri/src/overlay/commands.rs`     | Tauri commands: create, list, update, delete             |
| `src-tauri/src/countdown/store.rs`      | Reference impl for persistence (atomic write, load/save) |
| `src-tauri/src/settings.rs`             | `write_atomic`, `local_data_file` helpers                |

## Scope

1. Add `overlay/store.rs` — `load` and `save` functions following the
   countdown store pattern (`local_data_file(handle, "groups.json")`).
2. Wire `save` into every mutating group command (create, update, delete) —
   same fire-and-forget pattern as countdown store.
3. Wire `load` into `AppState::new` in `lib.rs` → pass persisted groups
   into `OverlayService`.
4. Add `OverlayService::from_dtos(groups)` (or equivalent restore constructor)
   that populates the `HashMap` and derives `next_id` as `max(id) + 1`.
5. Handle corrupt/missing file gracefully (fall back to empty, rename `.corrupt`).

## Out of scope

- Persisting `OverlayConfig` (separate issue: 002).
- Migrating existing group data (there is none to migrate).

## Verification

- Create groups, restart app → groups survive.
- Delete a group, restart → deletion persisted.
- Corrupt `groups.json` manually → app starts with empty groups, corrupt file
  renamed.
- `cargo test` — add round-trip tests in `tests/`.

## Implementation (2026-06-29)

Co-located with overlay styling in `overlays.json` (see issue 002) so a
group rename can never desync from its members' configs. The "groups.json"
file proposed in the original spec above was rolled into the single
overlay store as part of the unified fix.

### Files changed

| File                                     | Change                                                                                                                                                  |
| ---------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `src-tauri/src/overlay/store.rs` _(new)_ | `load(handle)` + `save(handle, &groups, &configs)`; corrupt-quarantine mirrors `countdown/store.rs`                                                     |
| `src-tauri/src/overlay/dto.rs`           | Added `GroupsAndConfigsDto` (one DTO holding both groups + configs) and `OverlayConfigMapEntry` transparent newtype                                     |
| `src-tauri/src/overlay/service.rs`       | Added `from_groups_and_configs(...)` (saturating-add `next_id`, duplicate-id dedup) and `snapshot(...)`                                                 |
| `src-tauri/src/overlay/commands.rs`      | Every mutating command (`group_create`, `group_update`, `group_delete`, `set_overlay_config`) calls `overlay::store::save` after the in-memory mutation |
| `src-tauri/src/app_state.rs`             | `AppState::new` accepts `Vec<GroupDto>` + `HashMap<u64, OverlayConfig>` and seeds `OverlayService` from them                                            |
| `src-tauri/src/lib.rs`                   | `setup` hook calls `overlay::store::load` alongside `countdown::store::load`; `get_overlay_config` registered in `invoke_handler!`                      |
| `src-tauri/tests/overlay_service.rs`     | 9 new tests added (see below)                                                                                                                           |

### Tests added

In `src-tauri/tests/overlay_service.rs`:

- `overlay_restore_round_trips_groups_and_configs` — group fields (name,
  members, layout, hide_idle) + per-id configs survive restore.
- `overlay_restore_next_id_avoids_collision` — a newly created group after
  restore gets `max(restored id)+1`.
- `overlay_restore_next_id_does_not_overflow_on_max` — defensive
  `saturating_add`, mirrors the matching countdown test.
- `overlay_restore_dedups_duplicate_group_ids` — first-wins, never drops
  a group in nondeterministic HashMap order.
- `overlay_restore_empty_starts_ids_at_zero` — matches fresh-service
  default.
- `overlay_snapshot_returns_groups_and_configs` — round-trip via
  `Vec<GroupDto>` + `HashMap<u64, OverlayConfig>`.
- `overlay_dto_json_round_trip` — DTO serialises losslessly to the
  on-disk shape.
- `overlay_dto_drops_non_numeric_config_keys` — corrupt/hand-edited store
  can never crash a save.
- `overlay_config_map_entry_serializes_transparently` — no `value`
  envelope, configs land as the config itself.

### Test results

```
running 15 tests
test overlay_restore_dedups_duplicate_group_ids ... ok
test overlay_restore_empty_starts_ids_at_zero ... ok
test overlay_restore_next_id_avoids_collision ... ok
test overlay_restore_next_id_does_not_overflow_on_max ... ok
test overlay_restore_round_trips_groups_and_configs ... ok
test overlay_snapshot_returns_groups_and_configs ... ok
test overlay_dto_drops_non_numeric_config_keys ... ok
test overlay_dto_json_round_trip ... ok
test overlay_config_map_entry_serializes_transparently ... ok
... (15/15 pass)
```
