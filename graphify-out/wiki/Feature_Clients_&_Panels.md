# Feature Clients & Panels

> 42 nodes · cohesion 0.10

## Key Concepts

- **group.store.ts** (18 connections) — `src/features/overlay/state/group.store.ts`
- **invokeCommand()** (13 connections) — `src/shared/tauri/invoke.ts`
- **group.client.ts** (10 connections) — `src/features/overlay/api/group.client.ts`
- **countdownStore** (10 connections) — `src/features/countdown/state/countdown.store.ts`
- **Stage.svelte** (8 connections) — `src/app/shell/Stage.svelte`
- **listGroups()** (7 connections) — `src/features/overlay/api/group.client.ts`
- **GroupDto** (7 connections) — `src/features/overlay/model/group.types.ts`
- **AppShell.svelte** (7 connections) — `src/app/shell/AppShell.svelte`
- **Roost.svelte** (7 connections) — `src/app/shell/Roost.svelte`
- **countdown.client.ts** (6 connections) — `src/features/countdown/api/countdown.client.ts`
- **CountdownDetail.svelte** (6 connections) — `src/features/countdown/components/CountdownDetail.svelte`
- **GroupPanel.svelte** (6 connections) — `src/features/overlay/components/GroupPanel.svelte`
- **remote.client.ts** (6 connections) — `src/features/remote/remote.client.ts`
- **groupStore** (6 connections) — `src/features/overlay/state/group.store.ts`
- **duration.ts (Duration utils)** (6 connections) — `src/shared/time/duration.ts`
- **group.client.ts** (4 connections) — `src/features/overlay/api/group.client.ts`
- **createGroup()** (4 connections) — `src/features/overlay/api/group.client.ts`
- **Layout** (4 connections) — `src/features/overlay/model/group.types.ts`
- **group.types.ts** (4 connections) — `src/features/overlay/model/group.types.ts`
- **invoke.ts** (4 connections) — `src/shared/tauri/invoke.ts`
- **deleteGroup()** (3 connections) — `src/features/overlay/api/group.client.ts`
- **updateGroup()** (3 connections) — `src/features/overlay/api/group.client.ts`
- **mapSnapshotDtoToSnapshot** (3 connections) — `src/features/countdown/model/countdown.mapper.ts`
- **remote.client.ts** (3 connections) — `src/features/remote/remote.client.ts`
- **RemoteSettings.svelte** (3 connections) — `src/features/remote/RemoteSettings.svelte`
- *... and 17 more nodes in this community*

## Relationships

- [[Tick/CSS Sync Contracts]] (4 shared connections)
- [[Countdown Command Client]] (4 shared connections)
- [[Overlay Persist Follow-ups]] (3 shared connections)
- [[Frontend Module Wiring]] (1 shared connections)

## Source Files

- `src/app/shell/AppShell.svelte`
- `src/app/shell/EyePip.svelte`
- `src/app/shell/OwlMark.svelte`
- `src/app/shell/Roost.svelte`
- `src/app/shell/Stage.svelte`
- `src/app/shell/types.ts`
- `src/features/countdown/api/countdown.client.ts`
- `src/features/countdown/components/CountdownDetail.svelte`
- `src/features/countdown/model/countdown.mapper.ts`
- `src/features/countdown/state/countdown.store.ts`
- `src/features/overlay/api/group.client.ts`
- `src/features/overlay/components/GroupPanel.svelte`
- `src/features/overlay/model/group.types.ts`
- `src/features/overlay/state/group.store.ts`
- `src/features/remote/RemoteSettings.svelte`
- `src/features/remote/remote.client.ts`
- `src/main.ts`
- `src/shared/tauri/invoke.ts`
- `src/shared/time/duration.ts`

## Audit Trail

- EXTRACTED: 177 (93%)
- INFERRED: 13 (7%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [[index]] to navigate.*