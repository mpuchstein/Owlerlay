# Countdown State Machine

> 28 nodes · cohesion 0.17

## Key Concepts

- **Countdown** (22 connections) — `src-tauri/src/countdown/model.rs`
- **CountdownState** (10 connections) — `src-tauri/src/countdown/model.rs`
- **.restore()** (9 connections) — `src-tauri/src/countdown/model.rs`
- **Instant** (9 connections) — `src-tauri/src/countdown/model.rs`
- **.finished()** (6 connections) — `src-tauri/src/countdown/model.rs`
- **.pause()** (6 connections) — `src-tauri/src/countdown/model.rs`
- **.start()** (6 connections) — `src-tauri/src/countdown/model.rs`
- **Duration** (6 connections) — `src-tauri/src/countdown/model.rs`
- **.new()** (5 connections) — `src-tauri/src/countdown/model.rs`
- **.remaining_at()** (5 connections) — `src-tauri/src/countdown/model.rs`
- **.resume()** (5 connections) — `src-tauri/src/countdown/model.rs`
- **.mark_finished()** (4 connections) — `src-tauri/src/countdown/model.rs`
- **.sync_finished_at()** (4 connections) — `src-tauri/src/countdown/model.rs`
- **CountdownError** (4 connections) — `src-tauri/src/countdown/model.rs`
- **Option** (4 connections) — `src-tauri/src/countdown/model.rs`
- **String** (4 connections) — `src-tauri/src/countdown/model.rs`
- **.start_timestamp()** (3 connections) — `src-tauri/src/countdown/model.rs`
- **.target_timestamp()** (3 connections) — `src-tauri/src/countdown/model.rs`
- **Into** (3 connections) — `src-tauri/src/countdown/model.rs`
- **model.rs** (3 connections) — `src-tauri/src/countdown/model.rs`
- **Result** (3 connections) — `src-tauri/src/countdown/model.rs`
- **Self** (3 connections) — `src-tauri/src/countdown/model.rs`
- **countdown-state listener (overlay JS)** (2 connections) — `src-tauri/templates/overlay/countdown/countdown.js.j2`
- **.initial_duration()** (2 connections) — `src-tauri/src/countdown/model.rs`
- **.state()** (2 connections) — `src-tauri/src/countdown/model.rs`
- *... and 3 more nodes in this community*

## Relationships

- [[Countdown DTOs & Events]] (2 shared connections)
- [[OBS Overlay Rendering]] (1 shared connections)
- [[Countdown Command Client]] (1 shared connections)
- [[Phone Remote & Token Auth]] (1 shared connections)
- [[Tick/CSS Sync Contracts]] (1 shared connections)

## Source Files

- `src-tauri/src/countdown/model.rs`
- `src-tauri/templates/overlay/countdown/countdown.js.j2`

## Audit Trail

- EXTRACTED: 132 (97%)
- INFERRED: 4 (3%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [[index]] to navigate.*