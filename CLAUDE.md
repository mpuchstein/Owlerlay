@AGENTS.md

# CLAUDE.md — Claude Code

- A graphify **PreToolUse hook** in `.claude/settings.json` nudges toward
  querying the knowledge graph before grepping. It no-ops when
  `graphify-out/graph.json` is absent, so it's safe for contributors without
  graphify. The graph's usage and upkeep are documented in AGENTS.md.
