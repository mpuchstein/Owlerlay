# Specs: Reset/Icons/Preview

> 22 nodes · cohesion 0.10

## Key Concepts

- **Per-countdown icon + label + configurable layout** (7 connections) — `docs/005-countdown-icon-label.md`
- **Auto-reset on countdown finish** (6 connections) — `docs/003-auto-reset.md`
- **Per-group overlay preview** (6 connections) — `docs/006-preview-per-group.md`
- **Per-countdown overlay preview** (6 connections) — `docs/007-preview-per-countdown.md`
- **Issue dependency graph (003-009)** (6 connections) — `docs/README.md`
- **Shared PreviewTile.svelte component** (5 connections) — `docs/006-preview-per-group.md`
- **In-place finish, not OBS reload (no flash)** (2 connections) — `docs/003-auto-reset.md`
- **Render test stays source of truth for OBS fidelity** (2 connections) — `docs/006-preview-per-group.md`
- **overlay_render.rs** (2 connections) — `src-tauri/tests/overlay_render.rs`
- **Issue 003 doc — Auto-reset on finish** (1 connections) — `docs/003-auto-reset.md`
- **Trigger only on natural Running->Finished** (1 connections) — `docs/003-auto-reset.md`
- **Reset to Idle, not auto-start** (1 connections) — `docs/003-auto-reset.md`
- **Issue 005 doc — Icons + labels per countdown** (1 connections) — `docs/005-countdown-icon-label.md`
- **Overlay icon_label distinct from Countdown::label** (1 connections) — `docs/005-countdown-icon-label.md`
- **Strict-undefined render test for all presets** (1 connections) — `docs/005-countdown-icon-label.md`
- **Preview is a snapshot, no SSE subscription** (1 connections) — `docs/006-preview-per-group.md`
- **Issue 006 doc — Preview per group** (1 connections) — `docs/006-preview-per-group.md`
- **Static Svelte mock over iframe** (1 connections) — `docs/006-preview-per-group.md`
- **No new single-countdown overlay endpoint needed** (1 connections) — `docs/007-preview-per-countdown.md`
- **Issue 007 doc — Preview per countdown** (1 connections) — `docs/007-preview-per-countdown.md`
- **Owlerlay Issue Tracker** (1 connections) — `docs/README.md`
- **PreviewTile.svelte** (1 connections) — `src/shared/preview/PreviewTile.svelte`

## Relationships

- [[Persistence Specs & Test]] (3 shared connections)
- [[Countdown Tauri Commands]] (1 shared connections)
- [[Countdown DTOs & Events]] (1 shared connections)
- [[Overlay Layout Presets]] (1 shared connections)
- [[Dashboard Spec]] (1 shared connections)

## Source Files

- `docs/003-auto-reset.md`
- `docs/005-countdown-icon-label.md`
- `docs/006-preview-per-group.md`
- `docs/007-preview-per-countdown.md`
- `docs/README.md`
- `src-tauri/tests/overlay_render.rs`
- `src/shared/preview/PreviewTile.svelte`

## Audit Trail

- EXTRACTED: 47 (85%)
- INFERRED: 8 (15%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [[index]] to navigate.*