use serde::{Deserialize, Serialize};

pub type RequestId = String;
pub type WorldId = String;
pub type ToolName = String;
pub type Version = String;

pub const BOUNDARY_CONTRACT_VERSION: &str = "1.0.0";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseEnvelope<T> {
    pub ok: bool,
    pub tool: ToolName,
    pub request_id: RequestId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub world_id: Option<WorldId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub errors: Vec<ApiError>,
    #[serde(default)]
    pub artifacts: Vec<String>,
    pub version: Version,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorldSeed {
    pub value: String,
    pub world_version: Version,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub salt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ChunkCoord {
    pub x: i32,
    pub y: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub z: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChunkKey {
    pub world_id: WorldId,
    pub coord: ChunkCoord,
    pub generation_version: Version,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RegionBounds {
    pub min: ChunkCoord,
    pub max: ChunkCoord,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorldProfile {
    pub name: String,
    pub world_size: u32,
    pub chunk_size: u32,
    pub sea_level: i32,
    pub max_height: i32,
    pub landmark_density: f32,
    pub feature_density: f32,
    pub streaming_radius: u32,
    pub generation_version: Version,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BiomeDefinition {
    pub id: String,
    pub name: String,
    pub terrain_color: [f32; 4],
    pub fog_color: [f32; 4],
    pub noise_threshold: f32,
    pub height_range: [i32; 2],
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LandmarkDefinition {
    pub id: String,
    pub name: String,
    pub rarity: f32,
    pub min_spacing: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MutationKind {
    TerrainEdit,
    PlaceObject,
    RemoveObject,
    TransformObject,
    SetState,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Mutation {
    pub mutation_id: String,
    pub kind: MutationKind,
    pub target_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chunk: Option<ChunkCoord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChunkPayload {
    pub chunk_key: ChunkKey,
    pub biome_id: String,
    #[serde(default)]
    pub feature_ids: Vec<String>,
    #[serde(default)]
    pub landmark_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh_reference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collision_reference: Option<String>,
    pub hash: String,
    pub version: Version,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SnapshotManifest {
    pub world_id: WorldId,
    pub seed: String,
    pub generation_version: Version,
    pub contract_version: Version,
    pub profile_hash: String,
    pub chunk_count: u32,
    #[serde(default)]
    pub mutation_log: Vec<String>,
    #[serde(default)]
    pub exported_files: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateWorldRequest {
    pub request_id: RequestId,
    pub seed: WorldSeed,
    pub world_name: String,
    pub profile_name: String,
    pub world_size: u32,
    pub chunk_size: u32,
    pub enable_streaming: bool,
    pub assetless_mode: bool,
    pub generation_version: Version,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConfigureWorldRequest {
    pub request_id: RequestId,
    pub world_id: WorldId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub world_size: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chunk_size: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sea_level: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_height: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub streaming_radius: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feature_density: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub landmark_density: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RequestChunkBatchRequest {
    pub request_id: RequestId,
    pub world_id: WorldId,
    #[serde(default)]
    pub chunks: Vec<ChunkCoord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RequestRegionRequest {
    pub request_id: RequestId,
    pub world_id: WorldId,
    pub region: RegionBounds,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubmitMutationBatchRequest {
    pub request_id: RequestId,
    pub world_id: WorldId,
    #[serde(default)]
    pub mutations: Vec<Mutation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValidateWorldRequest {
    pub request_id: RequestId,
    pub world_id: WorldId,
    #[serde(default)]
    pub strict: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<RegionBounds>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SaveSnapshotRequest {
    pub request_id: RequestId,
    pub world_id: WorldId,
    #[serde(default)]
    pub include_previews: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoadSnapshotRequest {
    pub request_id: RequestId,
    pub snapshot_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetWorldSummaryRequest {
    pub request_id: RequestId,
    pub world_id: WorldId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetChunkInfoRequest {
    pub request_id: RequestId,
    pub world_id: WorldId,
    pub chunk: ChunkCoord,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateWorldData {
    pub world_id: WorldId,
    pub profile_hash: String,
}

pub type CreateWorldResponse = ResponseEnvelope<CreateWorldData>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorldSummary {
    pub seed: String,
    pub profile_name: String,
    pub world_size: u32,
    pub chunk_size: u32,
    pub chunk_count: u32,
    pub biome_count: u32,
    pub landmark_count: u32,
}

pub type WorldSummaryResponse = ResponseEnvelope<WorldSummary>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValidationReport {
    pub passed: bool,
    pub metrics: serde_json::Value,
}

pub type ValidationResponse = ResponseEnvelope<ValidationReport>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChunkBatchData {
    pub chunks: Vec<ChunkPayload>,
}

pub type ChunkBatchResponse = ResponseEnvelope<ChunkBatchData>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MutationBatchData {
    pub applied: Vec<String>,
    pub rejected: Vec<String>,
}

pub type MutationBatchResponse = ResponseEnvelope<MutationBatchData>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SnapshotData {
    pub manifest: SnapshotManifest,
}

pub type SnapshotResponse = ResponseEnvelope<SnapshotData>;

pub fn supported_tools() -> &'static [&'static str] {
    &[
        "worldgen.create_world",
        "worldgen.configure_world",
        "worldgen.configure_profile",
        "worldgen.configure_biomes",
        "worldgen.configure_landmarks",
        "worldgen.preview_noise",
        "worldgen.generate_chunk",
        "worldgen.generate_region",
        "worldgen.request_chunk_batch",
        "worldgen.request_region",
        "worldgen.submit_mutation_batch",
        "worldgen.place_landmark",
        "worldgen.decorate_area",
        "worldgen.validate_world",
        "worldgen.save_snapshot",
        "worldgen.load_snapshot",
        "worldgen.export_snapshot",
        "worldgen.regenerate_from_seed",
        "worldgen.get_world_summary",
        "worldgen.get_chunk_info",
    ]
}

