# WorldGen MCP Unity

Unity project scaffold for an MCP-driven procedural world generation toolset.

## Overview

This repo is split into three layers:
- Unity for preview, editor workflows, and scene presentation
- Rust for deterministic world generation and validation
- MCP for AI-facing tool access

Purpose:
- let an AI agent author and iterate on world generation inside Unity
- keep generation deterministic and seed-based
- expose editor-safe operations through MCP rather than direct scene hacking
- support an assetless MVP using generated meshes, primitives, and debug overlays

Key documents:
- [`Docs/Specs/world_generation_mcp_spec.md`](Docs/Specs/world_generation_mcp_spec.md)
- [`Docs/ARCHITECTURE.md`](Docs/ARCHITECTURE.md)
- [`Docs/BOUNDARY_CONTRACT.md`](Docs/BOUNDARY_CONTRACT.md)
- [`Docs/ENGINEERING_RULES.md`](Docs/ENGINEERING_RULES.md)
- [`CONTRIBUTING.md`](CONTRIBUTING.md)
- [`backend/README.md`](backend/README.md)

## Bun Commands

Run these from the repo root with Bun. On this machine the binary is:
`/home/moebius/.bun/bin/bun`

Examples:

- `/home/moebius/.bun/bin/bun run help`
- `/home/moebius/.bun/bin/bun run backend:check`
- `/home/moebius/.bun/bin/bun run backend:fmt`
- `/home/moebius/.bun/bin/bun run backend:clippy`
- `/home/moebius/.bun/bin/bun run backend:test`
- `/home/moebius/.bun/bin/bun run backend:run`
- `/home/moebius/.bun/bin/bun run backend:contract`

Useful docs commands:
- `/home/moebius/.bun/bin/bun run docs:spec`
- `/home/moebius/.bun/bin/bun run docs:rules`
- `/home/moebius/.bun/bin/bun run docs:architecture`
- `/home/moebius/.bun/bin/bun run workspace:status`
- `/home/moebius/.bun/bin/bun run repo:tree`

Project status:
- scaffold only
- no gameplay code yet
- ready for Unity import and MCP tool implementation
- spec now includes the tool contract, data model, and no-asset requirements
- engineering rules define the Unity/Rust split and API-first approach
