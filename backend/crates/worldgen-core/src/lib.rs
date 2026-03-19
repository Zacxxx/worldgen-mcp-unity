use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use worldgen_api::{
    ApiError, BOUNDARY_CONTRACT_VERSION, CreateWorldData, CreateWorldRequest,
    CreateWorldResponse, ResponseEnvelope, ValidationReport, ValidationResponse, WorldId,
    WorldSummary, WorldSummaryResponse,
};

#[derive(Debug, Error)]
pub enum WorldgenError {
    #[error("world name cannot be empty")]
    EmptyWorldName,
    #[error("chunk size must be greater than zero")]
    InvalidChunkSize,
    #[error("world size must be greater than zero")]
    InvalidWorldSize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldState {
    pub world_id: WorldId,
    pub seed: String,
    pub world_name: String,
    pub profile_name: String,
    pub world_size: u32,
    pub chunk_size: u32,
    pub enable_streaming: bool,
    pub assetless_mode: bool,
    pub generation_version: String,
    pub profile_hash: String,
}

#[derive(Debug, Default)]
pub struct WorldStore {
    worlds: HashMap<WorldId, WorldState>,
}

impl WorldStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_world(
        &mut self,
        request: &CreateWorldRequest,
    ) -> Result<CreateWorldResponse, WorldgenError> {
        validate_create_world_request(request)?;

        let profile_hash = hash_world_profile(request);
        let world_id = format!("world-{}", short_hash(&profile_hash));
        let state = WorldState {
            world_id: world_id.clone(),
            seed: request.seed.value.clone(),
            world_name: request.world_name.clone(),
            profile_name: request.profile_name.clone(),
            world_size: request.world_size,
            chunk_size: request.chunk_size,
            enable_streaming: request.enable_streaming,
            assetless_mode: request.assetless_mode,
            generation_version: request.generation_version.clone(),
            profile_hash: profile_hash.clone(),
        };

        self.worlds.insert(world_id.clone(), state);

        Ok(ResponseEnvelope {
            ok: true,
            tool: "worldgen.create_world".to_string(),
            request_id: request.request_id.clone(),
            world_id: Some(world_id.clone()),
            data: Some(CreateWorldData {
                world_id,
                profile_hash,
            }),
            warnings: Vec::new(),
            errors: Vec::new(),
            artifacts: Vec::new(),
            version: BOUNDARY_CONTRACT_VERSION.to_string(),
        })
    }

    pub fn get_world_summary(&self, world_id: &str) -> Option<WorldSummaryResponse> {
        let state = self.worlds.get(world_id)?;
        let chunk_count = estimate_chunk_count(state.world_size, state.chunk_size);

        Some(ResponseEnvelope {
            ok: true,
            tool: "worldgen.get_world_summary".to_string(),
            request_id: String::new(),
            world_id: Some(state.world_id.clone()),
            data: Some(WorldSummary {
                seed: state.seed.clone(),
                profile_name: state.profile_name.clone(),
                world_size: state.world_size,
                chunk_size: state.chunk_size,
                chunk_count,
                biome_count: 0,
                landmark_count: 0,
            }),
            warnings: Vec::new(),
            errors: Vec::new(),
            artifacts: Vec::new(),
            version: BOUNDARY_CONTRACT_VERSION.to_string(),
        })
    }

    pub fn validate_world(&self, world_id: &str) -> Option<ValidationResponse> {
        let state = self.worlds.get(world_id)?;
        let passed = state.world_size > 0 && state.chunk_size > 0;

        Some(ResponseEnvelope {
            ok: passed,
            tool: "worldgen.validate_world".to_string(),
            request_id: String::new(),
            world_id: Some(state.world_id.clone()),
            data: Some(ValidationReport {
                passed,
                metrics: serde_json::json!({
                    "world_size": state.world_size,
                    "chunk_size": state.chunk_size,
                    "assetless_mode": state.assetless_mode,
                }),
            }),
            warnings: Vec::new(),
            errors: if passed {
                Vec::new()
            } else {
                vec![ApiError {
                    code: "validation_failed".to_string(),
                    message: "world state failed validation".to_string(),
                    details: None,
                }]
            },
            artifacts: Vec::new(),
            version: BOUNDARY_CONTRACT_VERSION.to_string(),
        })
    }
}

fn validate_create_world_request(request: &CreateWorldRequest) -> Result<(), WorldgenError> {
    if request.world_name.trim().is_empty() {
        return Err(WorldgenError::EmptyWorldName);
    }
    if request.chunk_size == 0 {
        return Err(WorldgenError::InvalidChunkSize);
    }
    if request.world_size == 0 {
        return Err(WorldgenError::InvalidWorldSize);
    }
    Ok(())
}

fn hash_world_profile(request: &CreateWorldRequest) -> String {
    let payload = serde_json::json!({
        "seed": request.seed.value,
        "world_name": request.world_name,
        "profile_name": request.profile_name,
        "world_size": request.world_size,
        "chunk_size": request.chunk_size,
        "enable_streaming": request.enable_streaming,
        "assetless_mode": request.assetless_mode,
        "generation_version": request.generation_version,
    });
    blake3::hash(payload.to_string().as_bytes())
        .to_hex()
        .to_string()
}

fn short_hash(value: &str) -> String {
    value.chars().take(12).collect()
}

fn estimate_chunk_count(world_size: u32, chunk_size: u32) -> u32 {
    if chunk_size == 0 {
        return 0;
    }
    let chunks_per_side = (world_size + chunk_size - 1) / chunk_size;
    chunks_per_side * chunks_per_side
}
