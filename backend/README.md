# Rust Backend Workspace

This workspace holds the authoritative world-generation backend for the project.

Layout:
- `crates/worldgen-api` - shared request/response contracts
- `crates/worldgen-core` - deterministic generation and validation logic
- `crates/worldgen-server` - backend entry point and service wrapper
- `contracts/` - human-readable API contract files

Rules:
- Keep backend logic in Rust.
- Keep API contracts versioned and explicit.
- Keep Unity as a consumer of backend output, not the source of truth.

