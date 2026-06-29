# App State & Event Bus

> 20 nodes · cohesion 0.17

## Key Concepts

- **AppState** (16 connections) — `src-tauri/src/app_state.rs`
- **app_state.rs** (10 connections) — `src-tauri/src/app_state.rs`
- **.new()** (9 connections) — `src-tauri/src/app_state.rs`
- **ClockAnchor** (6 connections) — `src-tauri/src/app_state.rs`
- **AppEvent** (5 connections) — `src-tauri/src/app_state.rs`
- **AppHandle** (3 connections) — `src-tauri/src/app_state.rs`
- **CountdownService** (2 connections) — `src-tauri/src/app_state.rs`
- **OverlayService** (2 connections) — `src-tauri/src/app_state.rs`
- **.instant_to_epoch_ms()** (2 connections) — `src-tauri/src/app_state.rs`
- **.new()** (2 connections) — `src-tauri/src/app_state.rs`
- **CountdownSnapshotDto** (2 connections) — `src-tauri/src/app_state.rs`
- **GroupDto** (2 connections) — `src-tauri/src/app_state.rs`
- **HashMap** (2 connections) — `src-tauri/src/app_state.rs`
- **Instant** (2 connections) — `src-tauri/src/app_state.rs`
- **OverlayConfig** (2 connections) — `src-tauri/src/app_state.rs`
- **Self** (2 connections) — `src-tauri/src/app_state.rs`
- **String** (2 connections) — `src-tauri/src/app_state.rs`
- **RwLock** (1 connections) — `src-tauri/src/app_state.rs`
- **Sender** (1 connections) — `src-tauri/src/app_state.rs`
- **Vec** (1 connections) — `src-tauri/src/app_state.rs`

## Relationships

- [[Overlay Commands & Bootstrap]] (3 shared connections)
- [[Remote Settings Commands]] (2 shared connections)
- [[Countdown Service]] (2 shared connections)
- [[Phone Remote & Token Auth]] (1 shared connections)
- [[OBS Overlay Rendering]] (1 shared connections)
- [[Overlay Service]] (1 shared connections)

## Source Files

- `src-tauri/src/app_state.rs`

## Audit Trail

- EXTRACTED: 73 (99%)
- INFERRED: 1 (1%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [[index]] to navigate.*