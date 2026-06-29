# Countdown DTOs & Events

> 15 nodes · cohesion 0.22

## Key Concepts

- **CountdownSnapshotDto** (10 connections) — `src-tauri/src/countdown/dto.rs`
- **AppEvent** (9 connections) — `src-tauri/src/countdown/events.rs`
- **events.rs** (7 connections) — `src-tauri/src/countdown/events.rs`
- **state_change_events()** (6 connections) — `src-tauri/src/countdown/events.rs`
- **finished_events()** (5 connections) — `src-tauri/src/countdown/events.rs`
- **CountdownStatePayload** (4 connections) — `src-tauri/src/countdown/events.rs`
- **CountdownTickPayload** (4 connections) — `src-tauri/src/countdown/events.rs`
- **CountdownSnapshotDto** (3 connections) — `src-tauri/src/countdown/events.rs`
- **dto.rs** (2 connections) — `src-tauri/src/countdown/dto.rs`
- **CountdownState** (2 connections) — `src-tauri/src/countdown/dto.rs`
- **CountdownState** (2 connections) — `src-tauri/src/countdown/events.rs`
- **Vec** (2 connections) — `src-tauri/src/countdown/events.rs`
- **Option** (1 connections) — `src-tauri/src/countdown/dto.rs`
- **String** (1 connections) — `src-tauri/src/countdown/dto.rs`
- **String** (1 connections) — `src-tauri/src/countdown/events.rs`

## Relationships

- [[Countdown Tauri Commands]] (4 shared connections)
- [[Countdown Command Client]] (2 shared connections)
- [[OBS Overlay Rendering]] (2 shared connections)
- [[Countdown State Machine]] (2 shared connections)
- [[Tick/CSS Sync Contracts]] (1 shared connections)
- [[Phone Remote & Token Auth]] (1 shared connections)
- [[Specs: Reset/Icons/Preview]] (1 shared connections)

## Source Files

- `src-tauri/src/countdown/dto.rs`
- `src-tauri/src/countdown/events.rs`

## Audit Trail

- EXTRACTED: 54 (92%)
- INFERRED: 5 (8%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [[index]] to navigate.*