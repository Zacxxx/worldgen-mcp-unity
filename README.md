# WorldGen MCP Unity

Unity runtime for a procedural world game, backed by Rust computation and exposed to AI tools through MCP.

## What This Is

This repo is split into three layers:

- **Unity** for the playable 3D scene, editor workflows, previews, and player runtime
- **Rust** for deterministic world generation, validation, mutation tracking, and snapshots
- **MCP** for AI-facing orchestration and tool access

The first usable version is designed to work in an assetless mode using generated meshes, primitives, colors, lighting, fog, and debug overlays.

## Key Documents

- [`Docs/ARCHITECTURE.md`](Docs/ARCHITECTURE.md)
- [`Docs/BOUNDARY_CONTRACT.md`](Docs/BOUNDARY_CONTRACT.md)
- [`Docs/ENGINEERING_RULES.md`](Docs/ENGINEERING_RULES.md)
- [`Docs/Specs/world_generation_mcp_spec.md`](Docs/Specs/world_generation_mcp_spec.md)
- [`CONTRIBUTING.md`](CONTRIBUTING.md)
- [`backend/README.md`](backend/README.md)

## Quick Start

1. Clone the repo.
2. Install Unity 2021.3 LTS or newer.
3. Ensure Rust and Bun are installed.
4. Open the Unity project from the repo root.
5. Use the Bun commands below for docs and backend checks.

## Bun Installation

Bun is only used here as a task runner for repo commands.

If Bun is already installed:

```bash
bun --version
```

If you want Bun on your PATH for this repo session:

```bash
export PATH="$HOME/.bun/bin:$PATH"
bun --version
```

No `bun install` step is required for this repo, because the `package.json` is only a local command manifest.

If Bun is not installed, follow the official installation guide and then rerun the command above.

## Bun Commands

Run commands from the repo root.

### Docs

- `bun run docs:spec`
- `bun run docs:rules`
- `bun run docs:architecture`
- `bun run docs:contributing`

### Backend

- `bun run backend:check`
- `bun run backend:fmt`
- `bun run backend:clippy`
- `bun run backend:test`
- `bun run backend:run`
- `bun run backend:contract`
- `bun run backend:clean`

### Repo

- `bun run help`
- `bun run repo:tree`
- `bun run workspace:status`

## Recommended Workflow

1. Read [`Docs/ENGINEERING_RULES.md`](Docs/ENGINEERING_RULES.md) before implementation.
2. Update [`Docs/BOUNDARY_CONTRACT.md`](Docs/BOUNDARY_CONTRACT.md) when the Unity/Rust interface changes.
3. Keep Rust authoritative for world state and generation.
4. Keep Unity focused on rendering, input, physics, and presentation.
5. Use assetless generation first, then layer in art later if needed.

## Repository Layout

- `Assets/` - Unity runtime and editor-side code
- `Docs/` - architecture, contract, and engineering rules
- `backend/` - Rust workspace and contract docs
- `Packages/` - Unity package metadata
- `ProjectSettings/` - Unity project settings

## Status

- scaffolded project
- Rust backend workspace initialized
- Unity boundary DTOs and client wrapper added
- architecture and boundary contract documented
- ready for implementation of runtime sync and chunk streaming
