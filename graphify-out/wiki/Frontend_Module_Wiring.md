# Frontend Module Wiring

> 36 nodes · cohesion 0.08

## Key Concepts

- **AppShell.svelte** (16 connections) — `src/app/shell/AppShell.svelte`
- **./Roost.svelte** (9 connections) — `src/app/shell/Roost.svelte`
- **./Stage.svelte** (8 connections) — `src/app/shell/Stage.svelte`
- **../../../app/shell/EyePip.svelte** (6 connections) — `src/app/shell/EyePip.svelte`
- **AppearancePanel.svelte** (6 connections) — `src/features/countdown/components/AppearancePanel.svelte`
- **../../countdown/state/countdown.store** (5 connections) — `src/features/countdown/state/countdown.store`
- **CountdownDetail.svelte** (5 connections) — `src/features/countdown/components/CountdownDetail.svelte`
- **GroupPanel.svelte** (5 connections) — `src/features/overlay/components/GroupPanel.svelte`
- **RemoteSettings.svelte** (4 connections) — `src/features/remote/RemoteSettings.svelte`
- **svelte** (4 connections) — `svelte`
- **./types** (3 connections) — `src/app/shell/types`
- **../state/group.store** (3 connections) — `src/features/overlay/state/group.store`
- **../../../shared/time/duration** (3 connections) — `src/shared/time/duration`
- **./OwlMark.svelte** (3 connections) — `src/app/shell/OwlMark.svelte`
- **main.ts** (3 connections) — `src/main.ts`
- **svelte/store** (3 connections) — `svelte/store`
- **../model/countdown.types** (2 connections) — `src/features/countdown/model/countdown.types`
- **../../../shared/overlay/origin** (2 connections) — `src/shared/overlay/origin`
- **destroy()** (1 connections) — `src/features/countdown/components/AppearancePanel.svelte`
- **../api/countdown.client** (1 connections) — `src/features/countdown/api/countdown.client`
- **../model/group.types** (1 connections) — `src/features/overlay/model/group.types`
- **./remote.client** (1 connections) — `src/features/remote/remote.client`
- **qrcode** (1 connections) — `qrcode`
- **active** (1 connections) — `src/app/shell/AppShell.svelte`
- **finishCreate()** (1 connections) — `src/app/shell/AppShell.svelte`
- *... and 11 more nodes in this community*

## Relationships

- [[Countdown Command Client]] (1 shared connections)
- [[Feature Clients & Panels]] (1 shared connections)

## Source Files

- `qrcode`
- `src/app/shell/AppShell.svelte`
- `src/app/shell/EyePip.svelte`
- `src/app/shell/OwlMark.svelte`
- `src/app/shell/Roost.svelte`
- `src/app/shell/Stage.svelte`
- `src/app/shell/types`
- `src/features/countdown/api/countdown.client`
- `src/features/countdown/components/AppearancePanel.svelte`
- `src/features/countdown/components/CountdownDetail.svelte`
- `src/features/countdown/model/countdown.types`
- `src/features/countdown/state/countdown.store`
- `src/features/overlay/components/GroupPanel.svelte`
- `src/features/overlay/model/group.types`
- `src/features/overlay/state/group.store`
- `src/features/remote/RemoteSettings.svelte`
- `src/features/remote/remote.client`
- `src/main.ts`
- `src/shared/overlay/origin`
- `src/shared/time/duration`

## Audit Trail

- EXTRACTED: 108 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [[index]] to navigate.*