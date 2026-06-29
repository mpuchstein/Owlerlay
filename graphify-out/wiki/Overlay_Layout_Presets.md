# Overlay Layout Presets

> 4 nodes · cohesion 0.50

## Key Concepts

- **OverlayLayout enum (4 presets + Custom)** (3 connections) — `docs/005-countdown-icon-label.md`
- **OverlayConfig** (3 connections) — `src-tauri/src/overlay/model.rs`
- **OverlayService::from_groups_and_configs restore ctor** (1 connections) — `docs/archive/001-persist-groups.md`
- **Custom flex escape hatch over per-preset variants** (1 connections) — `docs/005-countdown-icon-label.md`

## Relationships

- [[Specs: Reset/Icons/Preview]] (1 shared connections)
- [[Persistence Specs & Test]] (1 shared connections)

## Source Files

- `docs/005-countdown-icon-label.md`
- `docs/archive/001-persist-groups.md`
- `src-tauri/src/overlay/model.rs`

## Audit Trail

- EXTRACTED: 3 (38%)
- INFERRED: 5 (62%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [[index]] to navigate.*