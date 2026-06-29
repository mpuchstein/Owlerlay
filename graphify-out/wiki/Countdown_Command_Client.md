# Countdown Command Client

> 49 nodes · cohesion 0.09

## Key Concepts

- **countdown.store.ts** (33 connections) — `src/features/countdown/state/countdown.store.ts`
- **countdown.client.ts** (25 connections) — `src/features/countdown/api/countdown.client.ts`
- **countdown.types.ts** (18 connections) — `src/features/countdown/model/countdown.types.ts`
- **countdown.mapper.ts** (9 connections) — `src/features/countdown/model/countdown.mapper.ts`
- **duration.ts** (9 connections) — `src/shared/time/duration.ts`
- **invokeCountdownCommand()** (6 connections) — `src/features/countdown/api/countdown.client.ts`
- **mapSnapshotDtoToSnapshot()** (6 connections) — `src/features/countdown/model/countdown.mapper.ts`
- **CountdownSnapshotDto** (6 connections) — `src/features/countdown/model/countdown.types.ts`
- **actionOnSelected()** (6 connections) — `src/features/countdown/state/countdown.store.ts`
- **createCountdown()** (5 connections) — `src/features/countdown/api/countdown.client.ts`
- **listCountdowns()** (5 connections) — `src/features/countdown/api/countdown.client.ts`
- **CountdownState** (5 connections) — `src/features/countdown/model/countdown.types.ts`
- **OverlayConfig** (5 connections) — `src/features/countdown/model/countdown.types.ts`
- **Duration** (5 connections) — `src/shared/time/duration.ts`
- **deleteCountdown()** (4 connections) — `src/features/countdown/api/countdown.client.ts`
- **CountdownSnapshot** (4 connections) — `src/features/countdown/model/countdown.types.ts`
- **millisToDuration()** (4 connections) — `src/shared/time/duration.ts`
- **pauseCountdown()** (3 connections) — `src/features/countdown/api/countdown.client.ts`
- **resetCountdown()** (3 connections) — `src/features/countdown/api/countdown.client.ts`
- **resumeCountdown()** (3 connections) — `src/features/countdown/api/countdown.client.ts`
- **startCountdown()** (3 connections) — `src/features/countdown/api/countdown.client.ts`
- **CountdownTickPayload** (3 connections) — `src/features/countdown/model/countdown.types.ts`
- **TimeFormat** (3 connections) — `src/features/countdown/model/countdown.types.ts`
- **create()** (3 connections) — `src/features/countdown/state/countdown.store.ts`
- **deleteSelected()** (3 connections) — `src/features/countdown/state/countdown.store.ts`
- *... and 24 more nodes in this community*

## Relationships

- [[Feature Clients & Panels]] (4 shared connections)
- [[Tick/CSS Sync Contracts]] (4 shared connections)
- [[Countdown DTOs & Events]] (2 shared connections)
- [[Countdown State Machine]] (1 shared connections)
- [[Phone Remote & Token Auth]] (1 shared connections)
- [[Frontend Module Wiring]] (1 shared connections)

## Source Files

- `src/features/countdown/api/countdown.client.ts`
- `src/features/countdown/model/countdown.mapper.ts`
- `src/features/countdown/model/countdown.types.ts`
- `src/features/countdown/state/countdown.store.ts`
- `src/shared/payloads/empty.ts`
- `src/shared/time/duration.ts`

## Audit Trail

- EXTRACTED: 211 (97%)
- INFERRED: 6 (3%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [[index]] to navigate.*