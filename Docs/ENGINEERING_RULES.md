# Engineering Rules

These are the top-level rules for the project. They are intentionally high level and apply across the Unity frontend, Rust backend, and MCP tool surface.

## 1. API First

- Define the contract before implementation.
- Treat every feature as a request/response API, event stream, or data model.
- Keep Unity UI, editor code, and backend code as consumers of the same contract.

## 2. Deterministic by Default

- World generation must be reproducible from seed plus versioned inputs.
- No untracked randomness, timestamps, or global mutable state in generation paths.
- Any nondeterminism must be explicit and isolated.

## 3. Data Over Behavior

- Prefer declarative configuration over hardcoded logic.
- Use data models, rule sets, and versioned profiles to describe generation behavior.
- Keep content definitions separate from engine code.

## 4. Dry and Composable

- Do not duplicate rules across Unity, Rust, and MCP layers.
- Shared behavior belongs in one place and is called from the others.
- Small functions and small modules beat large monoliths.

## 5. Backend in Rust

- Put heavy generation, validation, indexing, serialization, and world-state logic in Rust.
- Keep Rust as the source of truth for world-generation rules and authoritative state transitions.
- Expose Rust through a stable FFI or IPC boundary rather than reimplementing core logic in Unity.

## 6. Unity as Presentation and Editor Shell

- Unity owns visualization, editor workflows, previews, and scene presentation.
- Unity should not become the canonical home of generation logic.
- If code can live in Rust, it probably should.

## 7. Strict Boundaries

- Separate runtime, editor, backend, and tooling concerns.
- Do not leak backend implementation details into the UI layer.
- Keep transport format and domain model versioned.

## 8. Observable and Testable

- Every important operation should produce a structured result.
- Every important rule should have a test.
- Prefer validation errors over silent corrections.

## 9. Fast Feedback Loops

- Optimize for iteration speed and local reproducibility.
- Make preview, validation, and regeneration cheap to run.
- Provide summaries, diffs, and hashes for every generated unit.

## 10. Minimal Surface Area

- Expose the smallest tool and API set that solves the job.
- Remove duplicate entry points.
- Keep public interfaces narrow and stable.

## 11. Version Everything

- Version generation profiles, snapshots, and tool contracts.
- Make backward compatibility explicit.
- Never assume old world data will match new code without a migration path.

## 12. Assetless First

- The first working version must not depend on imported art assets.
- Use generated geometry, colors, primitives, and debug overlays as the baseline.
- Art can be layered in later without changing the core rules.

## 13. Validate Before Expand

- New features must pass the current contract before scope is widened.
- Prefer a working narrow feature over an incomplete broad one.
- Keep acceptance criteria concrete.

## 14. Prefer Simplicity Under Load

- Choose the simplest design that meets performance and correctness targets.
- Reserve complexity for places where profiling or product needs justify it.
- Complexity must earn its place.
