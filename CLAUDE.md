# CLAUDE.md

> **[AGENTS.md](./AGENTS.md) is the canonical source of truth for this repo** —
> vision, architecture, the Hybrid working agreement, build/test commands,
> coding style, the graphify knowledge graph, and the release flow all live
> there. Read it on every session and comply with it.
>
> This file holds **only Claude-Code-specific** notes; everything that applies
> to any AI agent belongs in AGENTS.md, not here.

## Claude Code specifics

- Claude Code auto-loads this file each session — that's why it exists: to
  point at AGENTS.md. Keep it thin; put general guidance in AGENTS.md.
- A graphify **PreToolUse hook** in `.claude/settings.json` nudges toward
  querying the knowledge graph before grepping. It no-ops when
  `graphify-out/graph.json` is absent, so it's safe for contributors without
  graphify. The graph's usage and upkeep are documented in AGENTS.md.
