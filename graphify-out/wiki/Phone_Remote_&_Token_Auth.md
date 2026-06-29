# Phone Remote & Token Auth

> 35 nodes · cohesion 0.11

## Key Concepts

- **remote_sse()** (16 connections) — `src-tauri/src/server/remote.rs`
- **remote.rs** (16 connections) — `src-tauri/src/server/remote.rs`
- **control()** (14 connections) — `src-tauri/src/server/remote.rs`
- **require_token()** (11 connections) — `src-tauri/src/server/remote.rs`
- **list_countdowns()** (10 connections) — `src-tauri/src/server/remote.rs`
- **require_enabled()** (7 connections) — `src-tauri/src/server/remote.rs`
- **AppState** (7 connections) — `src-tauri/src/server/remote.rs`
- **Arc** (7 connections) — `src-tauri/src/server/remote.rs`
- **extract_token()** (5 connections) — `src-tauri/src/server/remote.rs`
- **State** (5 connections) — `src-tauri/src/server/remote.rs`
- **Request** (3 connections) — `src-tauri/src/server/remote.rs`
- **Response** (3 connections) — `src-tauri/src/server/remote.rs`
- **router()** (3 connections) — `src-tauri/src/server/remote.rs`
- **Result** (3 connections) — `src-tauri/src/server/remote.rs`
- **Next** (2 connections) — `src-tauri/src/server/remote.rs`
- **state listener (remote HTML)** (2 connections) — `src-tauri/templates/remote/remote.html`
- **constant_time_eq()** (2 connections) — `src-tauri/src/server/remote.rs`
- **remote_page()** (2 connections) — `src-tauri/src/server/remote.rs`
- **CountdownSnapshotDto** (2 connections) — `src-tauri/src/server/remote.rs`
- **Json** (2 connections) — `src-tauri/src/server/remote.rs`
- **Sse** (2 connections) — `src-tauri/src/server/remote.rs`
- **StatusCode** (2 connections) — `src-tauri/src/server/remote.rs`
- **String** (2 connections) — `src-tauri/src/server/remote.rs`
- **changed listener (remote HTML)** (1 connections) — `src-tauri/templates/remote/remote.html`
- **control() fetch to /api/remote/countdowns (remote HTML)** (1 connections) — `src-tauri/templates/remote/remote.html`
- *... and 10 more nodes in this community*

## Relationships

- [[Countdown Tauri Commands]] (5 shared connections)
- [[Tick/CSS Sync Contracts]] (2 shared connections)
- [[Countdown Command Client]] (1 shared connections)
- [[Countdown State Machine]] (1 shared connections)
- [[OBS Overlay Rendering]] (1 shared connections)
- [[Countdown DTOs & Events]] (1 shared connections)
- [[App State & Event Bus]] (1 shared connections)

## Source Files

- `src-tauri/src/server/remote.rs`
- `src-tauri/templates/remote/remote.html`

## Audit Trail

- EXTRACTED: 132 (94%)
- INFERRED: 8 (6%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [[index]] to navigate.*