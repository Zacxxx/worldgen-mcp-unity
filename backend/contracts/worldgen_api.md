# WorldGen API Contract

## Purpose

This contract defines the Rust-owned backend boundary for world generation.

The backend is responsible for:
- deterministic world creation
- profile and biome configuration
- chunk and region generation
- validation
- snapshot/export state
- stable summaries and hashes

## Response Envelope

All backend responses should follow the same shape:

- `ok`: boolean
- `tool`: string
- `request_id`: string
- `world_id`: string or null
- `data`: object or null
- `warnings`: string array
- `errors`: structured error array
- `artifacts`: string array

## Core Request Types

- `CreateWorldRequest`
- `ConfigureProfileRequest`
- `ConfigureBiomesRequest`
- `ConfigureLandmarksRequest`
- `PreviewNoiseRequest`
- `GenerateChunkRequest`
- `GenerateRegionRequest`
- `PlaceLandmarkRequest`
- `DecorateAreaRequest`
- `ValidateWorldRequest`
- `ExportSnapshotRequest`
- `RegenerateFromSeedRequest`
- `GetWorldSummaryRequest`
- `GetChunkInfoRequest`

## Core Response Types

- `CreateWorldResponse`
- `ConfigureProfileResponse`
- `ConfigureBiomesResponse`
- `ConfigureLandmarksResponse`
- `PreviewNoiseResponse`
- `GenerateChunkResponse`
- `GenerateRegionResponse`
- `PlaceLandmarkResponse`
- `DecorateAreaResponse`
- `ValidateWorldResponse`
- `ExportSnapshotResponse`
- `RegenerateFromSeedResponse`
- `WorldSummaryResponse`
- `ChunkInfoResponse`

## Versioning

- Contract versions must be explicit.
- Snapshot formats must carry a version.
- Breaking changes require a version bump.

