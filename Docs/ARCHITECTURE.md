# Architecture

This document describes the high-level system shape for the project.

The stack is:
- Unity for presentation, player runtime, editor tooling, and previews
- Rust for deterministic backend generation and validation
- MCP for AI-facing tool access

## Goals

- Keep world generation deterministic and versioned.
- Keep the core logic DRY across Unity, Rust, and MCP.
- Keep the first working version assetless.
- Keep the AI interface small, structured, and stable.

## Top-Level System Split

### Unity

Unity owns:
- playable scene presentation
- player input
- camera control
- physics and collision
- UI and HUD
- scene presentation
- editor UI
- preview rendering
- debug overlays
- mesh display and inspection
- invoking MCP tools or local editor commands

Unity does not own authoritative world-generation logic or world-state computation.

### Rust

Rust owns:
- world state
- deterministic generation
- world streaming decisions
- runtime world state transitions
- biome resolution
- landmark placement logic
- decoration rules
- validation
- serialization
- hashing and diffing

Rust is the source of truth for generation and state transitions.

### MCP

MCP owns:
- request parsing
- input validation
- command routing
- structured tool responses
- access to previews, summaries, reports, and exported artifacts

MCP is a thin orchestration layer, not a duplicate implementation of generation logic.

## Runtime Responsibilities

### Unity runtime

At play time Unity is responsible for:
- loading the initial world view
- spawning the player and runtime scene objects
- rendering terrain, props, lighting, and UI
- streaming chunk visuals in and out
- applying local transforms, physics interactions, and player-driven changes
- reporting runtime mutations back to Rust

### Rust runtime

At play time Rust is responsible for:
- generating initial and streamed world data
- owning canonical chunk and world state
- validating runtime mutations
- producing save snapshots and deltas
- serving deterministic regeneration for modified regions

## Module Ownership

### `Docs/`

- Product specs
- engineering rules
- architecture
- contributor guidance

### `Assets/`

- Unity scripts
- editor windows
- preview and visualization code
- scene-facing generated artifacts
- scriptable config definitions

### Rust backend workspace

When added, the Rust backend should be split into a dedicated workspace or crate set with clear ownership boundaries:

- `core`
  - generation, validation, world-state models
- `io`
  - snapshot format, serialization, versioning
- `api`
  - request and response types exposed to Unity and MCP
- `cli` or `server`
  - local process entry point or service wrapper

### MCP tool layer

- small tool handlers
- request validation
- routing into Rust backend services
- return-value shaping for agents

## Data Flow

### World creation

1. MCP receives a `create_world` request.
2. MCP validates the request shape.
3. MCP forwards the request to Rust.
4. Rust creates or updates world state and returns a versioned result.
5. Unity consumes the result for preview, editor display, or play mode bootstrapping.

### Generation

1. MCP or Unity requests generation for a world, chunk, or region.
2. Rust performs deterministic generation.
3. Rust returns chunk data, hashes, and reports.
4. Unity renders the returned data into the playable scene.

### Runtime sync

1. Unity requests visible chunks around the player.
2. Rust returns authoritative chunk data and mutation state.
3. Unity instantiates or updates renderable scene objects.
4. Unity reports player-driven changes, edits, and destruction back to Rust.
5. Rust records those mutations in the canonical world state.

### Validation

1. A validation request is sent to Rust through MCP or editor tooling.
2. Rust checks reproducibility, biome rules, spacing, and profile constraints.
3. The result is returned as structured diagnostics.

### Export

1. Rust serializes the current world snapshot.
2. MCP exposes file paths and checksums.
3. Unity can preview or import the exported snapshot if needed.

### Save and load

1. Rust saves canonical world state and runtime diffs.
2. Unity stores only presentation-specific state that cannot be derived.
3. On load, Rust reconstructs canonical state and Unity rebuilds the scene from it.

## Boundary Rules

- Do not duplicate generation rules in Unity and Rust.
- Do not put authoritative world logic in MonoBehaviours.
- Do not let MCP become a second implementation of backend logic.
- Do not pass Unity object references across stable API boundaries when ids or paths will do.
- Keep transport schemas versioned.
- Do not let Unity become the canonical source of generated world state.
- Do not let gameplay mutation bypass Rust state tracking.

## Transport Boundary

The backend boundary must be explicit and replaceable.

Preferred options:
- local IPC
- FFI
- local server over loopback

The chosen transport must support:
- versioned request/response types
- streaming or batch chunk requests
- structured errors
- deterministic retries
- clear lifecycle management for play mode and editor mode

## Assetless Rendering Model

The first implementation must be usable without imported art assets.

Required presentation elements:
- terrain meshes
- generated primitives for placeholder props
- solid-color or gradient materials
- debug overlays
- gizmo-based inspection

Optional later additions:
- imported textures
- imported meshes
- polished shaders
- authored VFX

## Versioning Strategy

- Version all world-generation inputs.
- Version snapshot formats.
- Version MCP tool contracts.
- Version the Rust API boundary.

If a change affects reproducibility, it must bump the relevant version and include a migration path or compatibility note.

## Testing Strategy

- Test deterministic generation with fixed seeds.
- Test validation with known-good and known-bad inputs.
- Test snapshot round-trips.
- Test tool contract stability.
- Test Unity preview rendering against returned backend data.

## Recommended Implementation Order

1. Define the Rust domain model.
2. Define the Rust request and response API.
3. Define the Unity runtime sync boundary.
4. Build the MCP tool layer on top of that API.
5. Build Unity play mode, preview, and editor shells that consume the API.
6. Add assetless rendering and debug views.
7. Add export, import, migration, and mutation persistence support.

## Practical Rule

If a piece of logic can be shared, versioned, and tested in Rust, it should live there first.
