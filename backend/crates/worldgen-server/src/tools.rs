use std::borrow::ToOwned;

use serde_json::Value;

use worldgen_api::{
    BOUNDARY_CONTRACT_VERSION, CreateWorldRequest, ResponseEnvelope,
    ValidateWorldRequest, ValidationResponse, WorldId, WorldSummaryResponse, supported_tools,
};
use worldgen_core::WorldStore;

pub struct ToolRouter {
    store: WorldStore,
}

#[allow(dead_code)]
impl ToolRouter {
    pub fn new(store: WorldStore) -> Self {
        Self { store }
    }

    pub fn create_world(
        &mut self,
        request: CreateWorldRequest,
    ) -> ResponseEnvelope<serde_json::Value> {
        match self.store.create_world(&request) {
            Ok(response) => ResponseEnvelope {
                ok: response.ok,
                tool: response.tool,
                request_id: response.request_id,
                world_id: response.world_id,
                data: Some(serde_json::to_value(response.data).unwrap_or(Value::Null)),
                warnings: response.warnings,
                errors: response.errors,
                artifacts: response.artifacts,
                version: response.version,
            },
            Err(err) => error_response(
                "worldgen.create_world",
                request.request_id,
                None,
                "create_world_failed",
                err.to_string(),
            ),
        }
    }

    pub fn validate_world(&self, request: ValidateWorldRequest) -> ValidationResponse {
        match self.store.validate_world(&request.world_id) {
            Some(mut response) => {
                response.request_id = request.request_id;
                response
            }
            None => error_validation_response(
                "worldgen.validate_world",
                request.request_id,
                request.world_id,
                "unknown_world",
                "world id is not known to the backend".to_string(),
            ),
        }
    }

    pub fn get_world_summary(&self, request_id: String, world_id: WorldId) -> WorldSummaryResponse {
        match self.store.get_world_summary(&world_id) {
            Some(mut response) => {
                response.request_id = request_id;
                response
            }
            None => error_summary_response(
                "worldgen.get_world_summary",
                request_id,
                world_id,
                "unknown_world",
                "world id is not known to the backend".to_string(),
            ),
        }
    }

    pub fn handle_tool(
        &mut self,
        tool: &str,
        request_id: String,
        world_id: Option<WorldId>,
        input: Value,
    ) -> ResponseEnvelope<Value> {
        match tool {
            "worldgen.create_world" => match serde_json::from_value::<CreateWorldRequest>(input) {
                Ok(mut request) => {
                    request.request_id = request_id;
                    self.create_world(request)
                }
                Err(err) => error_response(
                    tool,
                    request_id,
                    world_id,
                    "invalid_request",
                    err.to_string(),
                ),
            },
            "worldgen.validate_world" => {
                match serde_json::from_value::<ValidateWorldRequest>(input) {
                    Ok(mut request) => {
                        request.request_id = request_id;
                        let response = self.validate_world(request);
                        ResponseEnvelope {
                            ok: response.ok,
                            tool: response.tool,
                            request_id: response.request_id,
                            world_id: response.world_id,
                            data: Some(serde_json::to_value(response.data).unwrap_or(Value::Null)),
                            warnings: response.warnings,
                            errors: response.errors,
                            artifacts: response.artifacts,
                            version: response.version,
                        }
                    }
                    Err(err) => error_response(
                        tool,
                        request_id,
                        world_id,
                        "invalid_request",
                        err.to_string(),
                    ),
                }
            }
            "worldgen.get_world_summary" => {
                let parsed_world_id = world_id.or_else(|| {
                    input
                        .get("world_id")
                        .and_then(Value::as_str)
                        .map(ToOwned::to_owned)
                });

                match parsed_world_id {
                    Some(world_id) => {
                        let response = self.get_world_summary(request_id, world_id);
                        ResponseEnvelope {
                            ok: response.ok,
                            tool: response.tool,
                            request_id: response.request_id,
                            world_id: response.world_id,
                            data: Some(serde_json::to_value(response.data).unwrap_or(Value::Null)),
                            warnings: response.warnings,
                            errors: response.errors,
                            artifacts: response.artifacts,
                            version: response.version,
                        }
                    }
                    None => error_response(
                        tool,
                        request_id,
                        None,
                        "invalid_request",
                        "world_id is required".to_string(),
                    ),
                }
            }
            _ => unsupported_tool_response(tool, request_id, world_id),
        }
    }
}

pub fn describe_contract() -> Value {
    serde_json::json!({
        "contract_version": BOUNDARY_CONTRACT_VERSION,
        "transport": "local backend / IPC / FFI",
        "tools": supported_tools(),
    })
}

#[allow(dead_code)]
fn unsupported_tool_response(
    tool: &str,
    request_id: String,
    world_id: Option<WorldId>,
) -> ResponseEnvelope<Value> {
    error_response(
        tool,
        request_id,
        world_id,
        "not_implemented",
        "tool is registered but not implemented yet".to_string(),
    )
}

fn error_response(
    tool: &str,
    request_id: String,
    world_id: Option<WorldId>,
    code: &str,
    message: String,
) -> ResponseEnvelope<Value> {
    ResponseEnvelope {
        ok: false,
        tool: tool.to_string(),
        request_id,
        world_id,
        data: None,
        warnings: Vec::new(),
        errors: vec![worldgen_api::ApiError {
            code: code.to_string(),
            message,
            details: None,
        }],
        artifacts: Vec::new(),
        version: BOUNDARY_CONTRACT_VERSION.to_string(),
    }
}

#[allow(dead_code)]
fn error_validation_response(
    tool: &str,
    request_id: String,
    world_id: WorldId,
    code: &str,
    message: String,
) -> ValidationResponse {
    ResponseEnvelope {
        ok: false,
        tool: tool.to_string(),
        request_id,
        world_id: Some(world_id),
        data: None,
        warnings: Vec::new(),
        errors: vec![worldgen_api::ApiError {
            code: code.to_string(),
            message,
            details: None,
        }],
        artifacts: Vec::new(),
        version: BOUNDARY_CONTRACT_VERSION.to_string(),
    }
}

#[allow(dead_code)]
fn error_summary_response(
    tool: &str,
    request_id: String,
    world_id: WorldId,
    code: &str,
    message: String,
) -> WorldSummaryResponse {
    ResponseEnvelope {
        ok: false,
        tool: tool.to_string(),
        request_id,
        world_id: Some(world_id),
        data: None,
        warnings: Vec::new(),
        errors: vec![worldgen_api::ApiError {
            code: code.to_string(),
            message,
            details: None,
        }],
        artifacts: Vec::new(),
        version: BOUNDARY_CONTRACT_VERSION.to_string(),
    }
}
