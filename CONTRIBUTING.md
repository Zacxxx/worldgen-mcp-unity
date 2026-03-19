# Contributing

This project is a Unity frontend with a Rust backend and an MCP tool surface. Treat the repo as a contract-first system, not a pile of scripts.

## Working Rules

- Define the API before writing implementation.
- Keep world-generation logic deterministic and versioned.
- Put heavy generation, validation, indexing, and serialization in Rust.
- Keep Unity focused on visualization, editor workflows, and presentation.
- Keep MCP as a thin tool layer that validates inputs and forwards requests.
- Prefer data-driven rules over hardcoded behavior.
- Keep the first working version assetless.

## Repository Shape

- `Docs/ENGINEERING_RULES.md` contains the top-level engineering rules.
- `Docs/Specs/` contains product and interface specs.
- `Assets/` holds Unity-facing content and editor code.
- Rust backend code should live in a dedicated backend workspace or crate set when added.

## Development Expectations

- Work from a spec or contract before implementation.
- Keep changes narrow and easy to review.
- Avoid duplicating generation rules across Unity and Rust.
- Preserve reproducibility when changing seeds, profiles, or generation logic.
- Keep generated output separate from authored source files.
- Add validation before expanding behavior.

## Unity Side

- Use Unity for scene presentation, debug overlays, previews, and editor commands.
- Avoid embedding authoritative generation logic in MonoBehaviours.
- Prefer scriptable data definitions for configuration.
- Treat scene objects as generated runtime artifacts, not source of truth.

## Rust Side

- Use Rust for core world state, deterministic generation, and validation.
- Make public Rust APIs stable and explicit.
- Version request and response formats.
- Keep error messages structured and actionable.

## MCP Side

- Keep tools small, descriptive, and composable.
- Return structured data, not free-form strings.
- Include IDs, hashes, and file paths in responses where useful.
- Make previews and validation outputs easy for an agent to consume.

## Testing Expectations

- Test deterministic behavior with fixed seeds.
- Test validation rules with known-good and known-bad inputs.
- Test versioned snapshot loading and regeneration paths.
- Prefer tests that prove behavior rather than implementation details.

## Review Checklist

- The contract is clear.
- The change stays within the Unity/Rust/MCP boundary rules.
- The implementation is deterministic.
- Assetless mode still works.
- Validation covers the new behavior.
- The output is reproducible and inspectable.

## When In Doubt

- Keep the smallest useful interface.
- Move logic to Rust if Unity would duplicate it.
- Add a rule or test instead of relying on convention.
