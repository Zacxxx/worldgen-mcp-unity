# Unity / Rust Boundary Contract

This document defines the runtime contract between Unity and the Rust backend.

It covers:
- world creation
- chunk streaming
- runtime mutations
- validation
- save/load
- versioning

## Purpose

Unity is the playable runtime and presentation layer.

Rust is the authoritative world computation and state layer.

The boundary must allow Unity to render and play the world while Rust owns canonical world state, generation, and persistence.

## Ownership Rules

### Unity owns

- player input
- camera
- physics and collision
- UI and HUD
- rendering
- transient presentation state
- local scene instantiation and destruction

### Rust owns

- world seed and profile
- chunk data
- biome assignment
- landmark placement
- decoration rules
- runtime mutation history
- validation
- snapshot serialization
- world hashes and versioned state transitions

### Shared but Rust-authoritative

- chunk identifiers
- world identifiers
- mutation identifiers
- snapshot manifest
- generation version
- contract version

## Transport Requirements

The boundary may use IPC, FFI, or a loopback server, but it must support:

- versioned request and response types
- batched chunk requests
- streaming updates or polling
- structured errors
- deterministic retries
- clear lifecycle events for play mode start and stop

The transport must not change the domain contract.

## Core Data Types

### `WorldId`

Stable identifier for a generated world.

### `ChunkCoord`

Integer chunk coordinate pair or triplet, depending on the world layout.

### `ChunkKey`

Stable identifier derived from `WorldId` + `ChunkCoord` + generation version.

### `MutationId`

Stable identifier for a runtime mutation event.

### `Mutation`

Canonical world change recorded by Rust.

Examples:
- terrain edit
- object placement
- object removal
- object transform change
- container or interactable state change

### `ChunkPayload`

Renderable and authoritative data for one chunk.

Contains:
- geometry or mesh data reference
- biome data
- feature data
- landmark data
- collision data
- mutation overlay
- version and hash

### `SnapshotManifest`

Versioned summary of saved world state.

Contains:
- world id
- seed
- generation version
- contract version
- profile hash
- chunk count
- mutation log reference
- exported file list

## Request Types

### `CreateWorld`

Creates a new world from seed and profile parameters.

### `ConfigureWorld`

Updates world settings before or during generation.

### `RequestChunkBatch`

Requests one or more chunks around a player or region.

### `RequestRegion`

Requests generation of a rectangular region for loading or preview.

### `SubmitMutationBatch`

Sends runtime changes from Unity back to Rust.

### `ValidateWorld`

Requests validation of a world, region, or snapshot.

### `SaveSnapshot`

Requests Rust to serialize canonical state and a snapshot manifest.

### `LoadSnapshot`

Requests Rust to restore canonical state from a snapshot.

### `GetWorldSummary`

Requests a compact summary for UI or debugging.

## Response Envelope

All boundary responses should use a structured envelope:

- `ok` boolean
- `request_id` string
- `world_id` string or null
- `data` object or null
- `warnings` array
- `errors` array
- `artifacts` array
- `version` string

Errors should be structured and machine-readable.

## Event Types

### From Rust to Unity

#### `ChunkReady`

Sent when a chunk is available for rendering.

Contains:
- chunk key
- chunk payload
- hash
- version

#### `ChunkUnloaded`

Sent when a chunk should be removed or cached.

#### `WorldUpdated`

Sent when canonical world state changes.

#### `ValidationReport`

Sent when a validation run completes.

#### `SnapshotReady`

Sent when a save snapshot is available.

#### `GenerationProgress`

Sent during long generation or streaming operations.

### From Unity to Rust

#### `RuntimeLoaded`

Sent when the playable scene is ready and the initial world can be requested.

#### `PlayerMoved`

Sent when player movement crosses a streaming threshold or affects nearby loading needs.

#### `ApplyMutation`

Sent when Unity has committed a gameplay mutation that must become canonical.

#### `RuntimeUnloaded`

Sent when play mode ends or the scene is torn down.

## Sync Rules

- Rust is always canonical.
- Unity may cache and render derived state, but it must not become the source of truth.
- Every runtime mutation must be reported to Rust.
- Every chunk request must be idempotent for the same world state and coordinates.
- Unity must be able to rebuild the scene from Rust state after a reload or disconnect.
- Rust must be able to reproduce the playable world from a snapshot without Unity-specific hidden state.

## Startup Sequence

1. Unity boots the playable scene.
2. Unity sends `RuntimeLoaded`.
3. Unity requests `CreateWorld` or `LoadSnapshot`.
4. Rust returns the canonical world state.
5. Unity requests the initial chunk batch around the player.
6. Rust returns chunk payloads.
7. Unity renders the initial scene.
8. Unity begins sending `PlayerMoved` and `ApplyMutation` events as needed.

## Runtime Mutation Rules

- Mutations must be serialized in a versioned format.
- Mutations must include stable ids and target references.
- Mutations must be applied in a deterministic order.
- If a mutation cannot be applied, Rust must return a structured rejection.
- Unity must not silently invent canonical changes.

## Save / Load Rules

The save format should include:

- snapshot manifest
- profile data
- chunk index
- mutation log or diff stream
- validation summary

Load flow:

1. Rust restores canonical world state.
2. Unity receives the reconstructed state.
3. Unity re-requests visible chunks.
4. Unity rebuilds presentation state from returned chunk payloads.

## Versioning

- The boundary contract must have a version.
- Breaking changes require a version bump.
- Snapshot and mutation formats must be versioned separately from the transport.

## Minimal Viable Boundary

The first implementation should support:

- create world
- load snapshot
- request chunk batch
- submit mutation batch
- validate world
- save snapshot
- get summary

