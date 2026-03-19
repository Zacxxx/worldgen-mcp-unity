# World Generation MCP Spec

## Goal

Build a Unity project that exposes a small, reliable set of MCP tools for an AI agent to create, inspect, validate, and iterate on procedural worlds.

The target behavior is a Valheim-like workflow:
- deterministic worlds from a seed
- biome-driven terrain and content
- chunk-based streaming
- landmark placement and decoration
- validation and repeatable regeneration

This project must work without imported art assets. The first usable version should be entirely code-generated and visually represented with built-in meshes, colors, materials, lighting, fog, and debug overlays.

Implementation rules for contributors live in:
- [`Docs/ENGINEERING_RULES.md`](../ENGINEERING_RULES.md)
- [`CONTRIBUTING.md`](../../CONTRIBUTING.md)

## Scope

### In scope

- deterministic world generation
- assetless terrain and content generation
- biome configuration
- chunk streaming
- landmark placement
- validation and export
- MCP tool contracts for AI use
- editor previews and debug visualizations

### Out of scope

- hand-authored art pipelines
- custom imported textures or models
- cinematic polish
- complex combat systems
- multiplayer replication

## Governance

- Treat this document as the product and system contract.
- Treat [`Docs/ENGINEERING_RULES.md`](../ENGINEERING_RULES.md) as the project-wide engineering policy.
- Treat [`CONTRIBUTING.md`](../../CONTRIBUTING.md) as the contributor operating guide.
- If a change conflicts with the engineering rules, update the spec explicitly before implementation.

## Runtime Model

This project is not editor-only. The intended result is a playable Unity runtime backed by Rust computation.

- Unity owns the rendered 3D scene, player input, physics, camera, UI, and play mode lifecycle.
- Rust owns world generation, world state, validation, mutation tracking, and snapshot serialization.
- MCP owns AI-facing orchestration and tool access.

The playable game scene must be reconstructible from Rust-owned world data plus Unity presentation state.

## Design Principles

1. Deterministic first
   - Every world result must be reproducible from a seed and generation inputs.
   - The same input set must produce the same world layout, chunk data, and landmark placement.

2. Tool-oriented, not scene-oriented
   - The AI agent should operate through world-gen tools rather than manipulating objects ad hoc.
   - Unity scene edits are an implementation detail of the tool layer.

3. Inspectable at each step
   - Every generation stage must be previewable and report its decisions.
   - The agent should be able to ask for summaries, heatmaps, and validation reports.

4. Safe to iterate
   - Tools must support undoable or regenerable operations where possible.
   - Generated artifacts should be isolated from hand-authored project assets.

5. Batch-friendly
   - The agent must be able to create multiple chunks, biomes, or landmarks in one operation.

6. Assetless by default
   - The first playable version must work using generated meshes, primitives, colors, and built-in Unity components only.
   - External art may be added later, but the generation system must not depend on it.

## Assetless Visual Strategy

The world should be represented without imported art assets using the following layers:

### Geometry

- terrain chunks generated as meshes
- cliffs generated from height discontinuities or stepped surfaces
- rocks generated from perturbed primitives
- trees generated from stacked primitives or lightweight procedural meshes
- ruins and landmarks generated from modular cube/cylinder/arch combinations

### Materials

- built-in Unity materials or minimal project materials
- solid colors and gradient ramps by biome
- vertex colors for terrain and feature tinting
- no texture dependency in the core pipeline

### Debug and presentation

- biome heatmaps
- chunk grid overlays
- contour lines
- feature markers
- generation statistics
- gizmos and labels in editor mode

### Lighting and atmosphere

- directional light
- fog color per biome or region
- skybox optional, not required
- post-processing optional, not required

## Unity Project Structure

Recommended layout:

- `Assets/Scripts/WorldGenMcp/`
  - runtime models, generator pipeline, data containers
- `Assets/Scripts/WorldGenMcp/Editor/`
  - MCP-facing editor commands, validation, previews
- `Assets/WorldGenMcp/Definitions/`
  - biome, landmark, climate, and rule `ScriptableObject` assets
- `Assets/WorldGenMcp/Generated/`
  - generated debug previews, cached maps, exported snapshots
- `Docs/Specs/`
  - project specs and API contracts

Generated output must not overwrite authored definitions. Keep generated previews and snapshots in a separate tree from hand-authored data.

## Domain Model

### Core entities

#### `WorldSeed`
- `seed` string or integer
- `world_version`
- optional `salt`
- optional `profile_hash`

#### `WorldProfile`
- `name`
- `world_size`
- `chunk_size`
- `sea_level`
- `max_height`
- `biome_weights`
- `climate_settings`
- `landmark_density`
- `feature_density`
- `streaming_radius`
- `generation_version`

#### `NoiseLayerDefinition`
- `name`
- `frequency`
- `amplitude`
- `octaves`
- `persistence`
- `lacunarity`
- `offset`
- `warp_strength`
- `mask_mode`

#### `BiomeDefinition`
- `name`
- `id`
- `height_range`
- `temperature_range`
- `moisture_range`
- `noise_threshold`
- `terrain_color`
- `fog_color`
- `allowed_landmarks`
- `allowed_features`
- `spawn_weights`
- `terrain_rules`

#### `LandmarkDefinition`
- `name`
- `id`
- `rarity`
- `biome_affinity`
- `min_spacing`
- `max_per_world`
- `placement_rules`
- `procedural_template`
- `debug_color`

#### `ChunkData`
- `chunk_coords`
- `world_id`
- `seed`
- `heightfield`
- `biome_map`
- `surface_features`
- `landmarks`
- `decorations`
- `validation_flags`
- `hash`

#### `WorldSnapshot`
- `world_id`
- `seed`
- `profile`
- `biomes`
- `landmarks`
- `chunk_index`
- `export_hash`
- `preview_paths`

#### `GenerationReport`
- `world_id`
- `seed`
- `profile_hash`
- `chunks_generated`
- `warnings`
- `errors`
- `coverage_stats`
- `regeneration_suggestions`

## System Architecture

The system should be split into four cooperating layers:

### 1. Unity runtime layer

Responsible for:
- loading and rendering the playable scene
- player input, camera, physics, and UI
- instantiating chunk visuals and runtime objects
- reporting player-driven mutations back to Rust

### 2. Rust runtime generation layer

Responsible for:
- deterministic noise sampling
- terrain mesh generation
- biome classification
- landmark selection
- decoration placement
- chunk state serialization
- runtime world state
- world mutation tracking
- save/load snapshots
- streaming decisions

This layer should be implemented in Rust and treated as the authoritative source of world-generation logic.

### 3. Editor and preview layer

Responsible for:
- generating preview textures
- rendering debug overlays
- validating configuration
- exporting snapshots
- showing reports in the editor

### 4. MCP tool layer

Responsible for:
- receiving AI requests
- validating inputs
- calling generation/editor services
- returning structured results and file paths

The MCP layer must not implement world generation logic directly. It should call into the Rust generation services so the same logic is usable from editor UI, automated tests, and MCP.

## Data Format Rules

### Determinism rules

- All generation must derive from the world seed plus documented parameters.
- Any random call must use a seeded generator.
- Avoid global random state.
- A chunk regenerated from the same seed and parameters must produce the same hash.

### Serialization rules

- Configuration assets use `ScriptableObject`.
- Generated world state uses versioned Rust data structures and serialized interchange formats.
- Unity consumes projections of backend state for visualization and editor workflows.
- Exported snapshots use JSON plus preview files.
- Every snapshot must include a version field.

### Runtime sync rules

- Unity may own transient presentation state, but canonical world state stays in Rust.
- Unity must report player-driven terrain edits, placed objects, destruction, and other mutations back to Rust.
- Rust must be able to reconstruct the playable scene from saved canonical data.
- Chunk streaming requests must be idempotent for the same seed, world state, and chunk coordinates.
- Runtime mutations must be versioned and recorded as diffs or events.

### Stability rules

- Tool inputs must be JSON-friendly.
- Tool outputs must be structured and machine-readable.
- Prefer strings, numbers, booleans, arrays, and simple objects.
- Avoid Unity object references in MCP responses unless they are converted to stable ids or asset paths.

## MCP Tool Surface

The AI agent should have access to a compact tool set. Suggested tool names are below.

### Common response envelope

Every tool should return:

- `ok` boolean
- `tool` string
- `request_id` string
- `world_id` string when relevant
- `data` object or `null`
- `warnings` array
- `errors` array
- `artifacts` array of file paths or ids

If `ok` is `false`, the response must still include `errors` and enough context for the agent to recover.

### 1. `worldgen.create_world`

Creates a new world definition and initializes generation state.

Inputs:
- `seed` string or integer
- `world_name` string
- `profile_name` string
- `world_size` integer
- `chunk_size` integer
- `enable_streaming` boolean
- `assetless_mode` boolean
- `generation_version` string

Outputs:
- `world_id`
- normalized settings
- initial `GenerationReport`
- created profile hash

Validation:
- `world_name` must be non-empty
- `chunk_size` must divide world bounds cleanly or the tool must report the remainder strategy
- `assetless_mode` should default to `true`

### 2. `worldgen.configure_profile`

Creates or updates the world profile.

Inputs:
- `world_id`
- `world_size`
- `chunk_size`
- `sea_level`
- `max_height`
- `streaming_radius`
- `feature_density`
- `landmark_density`
- `noise_layers`

Outputs:
- updated profile
- validation summary
- profile hash

### 3. `worldgen.configure_biomes`

Creates or updates biome definitions used by the world.

Inputs:
- `world_id`
- `biomes` array of biome definitions
- `overwrite_policy` string

Outputs:
- updated biome library
- validation summary
- biome ids

Validation:
- biome ids must be unique
- ranges should not conflict unless overlap is explicitly intended
- at least one biome must cover the default land range

### 4. `worldgen.configure_landmarks`

Creates or updates landmark definitions.

Inputs:
- `world_id`
- `landmarks` array
- `overwrite_policy` string

Outputs:
- updated landmark library
- validation summary
- landmark ids

Validation:
- min spacing must be non-negative
- rarity must be in a documented range
- at least one landmark should be placeable in the configured world

### 5. `worldgen.preview_noise`

Previews one or more noise layers without committing changes.

Inputs:
- `world_id`
- `layer_name`
- `resolution`
- `region`
- `palette_mode`
- `show_contours` boolean

Outputs:
- preview texture or render target path
- sampled statistics
- layer summary

### 6. `worldgen.generate_chunk`

Generates a single chunk or a chunk set.

Inputs:
- `world_id`
- `chunk_coords`
- `regenerate` boolean
- `include_decorations` boolean
- `include_landmarks` boolean
- `include_debug_overlays` boolean

Outputs:
- chunk data
- generated preview
- feature summary
- chunk hash

Validation:
- chunk coordinates must be within world bounds unless the world supports infinite streaming
- chunk hash must be deterministic

### 7. `worldgen.generate_region`

Generates a rectangular region of chunks in one batch.

Inputs:
- `world_id`
- `region_bounds`
- `generation_flags`
- `batch_size`

Outputs:
- chunk list
- aggregate biome coverage
- aggregate feature summary
- region hash

### 8. `worldgen.place_landmark`

Places a landmark using seed-based placement rules.

Inputs:
- `world_id`
- `landmark_name`
- `placement_mode`
- `preferred_biomes`
- `spacing_rules`
- `anchor_region`

Outputs:
- placement result
- collision report
- final coordinates
- landmark instance id

### 9. `worldgen.decorate_area`

Adds vegetation, rocks, clutter, and ambient props.

Inputs:
- `world_id`
- `region_bounds`
- `decorator_profile`
- `density_multiplier`
- `seed_override` optional

Outputs:
- placed prop summary
- excluded areas
- decoration hash

### 10. `worldgen.validate_world`

Checks the generated world against design constraints.

Inputs:
- `world_id`
- `validation_profile`
- `region_bounds` optional
- `strict` boolean

Outputs:
- `GenerationReport`
- list of errors and warnings
- suggested fixes
- pass/fail boolean

Validation should cover:
- seed reproducibility
- biome coverage
- landmark spacing
- chunk continuity
- height limits
- sea level sanity
- feature density sanity

### 11. `worldgen.export_snapshot`

Exports the current world state for review or reuse.

Inputs:
- `world_id`
- `export_format`
- `include_previews` boolean
- `include_reports` boolean

Outputs:
- file paths
- manifest
- checksum

Required export contents:
- world profile JSON
- biome definitions JSON
- landmark definitions JSON
- chunk index JSON
- report JSON
- optional preview images

### 12. `worldgen.regenerate_from_seed`

Rebuilds the world from the original seed and updated profile.

Inputs:
- `world_id`
- `seed`
- `profile_delta`
- `scope`

Outputs:
- regenerated world state
- diff report
- affected chunks

### 13. `worldgen.get_world_summary`

Returns a compact summary for the agent.

Inputs:
- `world_id`

Outputs:
- seed
- profile summary
- chunk counts
- biome counts
- landmark counts
- active issues

### 14. `worldgen.get_chunk_info`

Returns detailed information about a specific chunk.

Inputs:
- `world_id`
- `chunk_coords`

Outputs:
- chunk hash
- biome distribution
- feature list
- landmark list
- validation flags

## Required Agent Workflows

### Workflow A: New world

1. create world
2. configure profile
3. configure biomes
4. configure landmarks
5. preview noise
6. generate initial region
7. validate world
8. export snapshot

### Workflow B: Iteration loop

1. generate chunk or region
2. inspect report
3. adjust biome or profile parameters
4. regenerate affected area
5. validate again

### Workflow C: Landmark pass

1. place landmark constraints
2. generate region
3. run validation
4. resolve overlaps and spacing errors

### Workflow D: Assetless MVP

1. create world in assetless mode
2. generate terrain mesh and debug overlays
3. render biomes using colors and fog only
4. place generated rocks, trees, and ruins from primitives
5. validate reproducibility
6. export snapshot

## Assetless MVP Requirements

The first usable implementation must work without imported art assets.

### Required content types

- terrain mesh per chunk
- primitive-based trees and rocks
- primitive-based ruins or landmarks
- colored biome overlays
- debug UI panels
- gizmo/label visualization
- runtime player controller and camera
- physics-enabled terrain and collision

### Allowed project dependencies

- Unity built-in primitives
- Unity built-in materials
- ScriptableObjects
- generated meshes
- editor gizmos
- simple runtime UI

### Not required for MVP

- imported textures
- imported character models
- imported foliage packs
- shader graph art polish
- custom sound assets

## Implementation Notes

- Keep generated data separate from authored assets.
- Prefer `ScriptableObject` definitions for biomes and landmark rules.
- Prefer chunk-local generation over whole-world generation.
- Use editor tools for previews, validation, and exports.
- Keep public tool inputs JSON-friendly so MCP can call them cleanly.
- Every generation stage should be callable from tests as well as MCP.
- Use a versioned `GenerationProfile` so future changes do not break old saves.

## Suggested Runtime Classes

These are the baseline classes the project should implement.

- `WorldManager`
- `WorldGenerationService`
- `ChunkGenerator`
- `BiomeResolver`
- `LandmarkPlacer`
- `DecorationPlacer`
- `WorldValidator`
- `WorldSnapshotSerializer`
- `NoiseSampler`
- `GenerationCache`
- `McpWorldGenTools`
- `McpWorldGenEditorWindow`

## Suggested Data Contracts

### `CreateWorldRequest`

- `seed`
- `world_name`
- `profile_name`
- `world_size`
- `chunk_size`
- `enable_streaming`
- `assetless_mode`
- `generation_version`

### `GenerateRegionRequest`

- `world_id`
- `region_bounds`
- `generation_flags`
- `batch_size`

### `ValidationResult`

- `ok`
- `warnings`
- `errors`
- `metrics`
- `suggestions`

### `SnapshotManifest`

- `world_id`
- `seed`
- `generation_version`
- `profile_hash`
- `chunk_count`
- `exported_files`
- `checksums`

## Acceptance Criteria

The project is ready for implementation when:

- an AI agent can create a world from a seed without hand-editing scenes
- the same seed produces the same chunk layout and feature placement
- biome rules are editable as data
- chunk generation can run in batches
- validation returns actionable errors
- exported snapshots contain enough data to reproduce the world
- the project can run in a no-asset mode using only generated meshes and built-in visuals
- the MCP tool surface covers create, configure, preview, generate, validate, summarize, and export operations

## Milestones

1. Scaffold runtime data models.
2. Add editor scripts that expose the tool surface.
3. Implement deterministic height and biome sampling.
4. Add chunk generation and validation.
5. Connect the editor commands to MCP.
6. Add assetless debug rendering and primitive-based placeholders.
7. Add snapshot export and regeneration support.

## Definition of Done

The work is complete when all of the following are true:

- a world can be created from an MCP call
- the world can be generated in assetless mode
- the generated world is reproducible from the same seed
- the agent can query summaries and chunk details
- the agent can preview noise and validate results
- export produces a complete reproducible snapshot
- the implementation is usable without any hand-authored art assets
- Unity can load the generated world into a playable scene
- Rust remains authoritative for world state, mutation tracking, and regeneration
