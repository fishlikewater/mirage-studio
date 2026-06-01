# PRD: OpenAI protocol cleanup and XAIS Task removal

## Goal

- Remove remaining built-in API configuration entry points and orphan code while keeping supplier-based configuration as the only user-facing provider configuration.
- Remove the XAIS Task protocol from supplier configuration, runtime model construction, and backend generation paths.
- Review and minimally optimize the custom OpenAPI-compatible and OpenAI Image protocols against official OpenAI API documentation.
- Fix OpenAI Image edit requests so multiple reference photos are sent in the documented multipart shape.

## Assumptions

- "Built-in API configuration" means settings/UI/store/command remnants for global built-in API keys, not every built-in provider implementation that is still reachable through existing non-supplier code paths.
- "Remove XAIS Task" means it should no longer be selectable, persisted as a valid current protocol, or routed in backend generation commands.
- Existing persisted XAIS supplier records may be normalized away or rejected by validation; no migration UI is required unless tests reveal an existing migration contract.
- OpenAI documentation is the source of truth for Images API multipart fields.

## Scope

- Frontend supplier configuration types, validation, editor UI, i18n text, runtime model registry, model parameter controls, and related tests.
- Tauri generation command routing and provider module exports for XAIS Task removal.
- Rust OpenAI Image and OpenAPI-compatible providers plus tests needed to cover official API behavior.
- Search audit for removed protocol strings and built-in API config entry points.

## Out of Scope

- Removing all non-supplier built-in provider implementations unless they become direct orphans from this change.
- Broad redesign of supplier management or model registry.
- Introducing new OpenAI models or pricing/model-selection policy beyond fixing current protocol behavior.

## Verification

- Targeted frontend tests around supplier config, settings editor/page, runtime registry, runtime generation context, model controls, and image node payloads.
- Targeted Rust tests for OpenAI Image/OpenAPI-compatible providers and command routing.
- Search audit for `xais-task`, XAIS UI keys, and built-in API config remnants.
- Final `npm test`, `npm run build`, and `cargo check` in `src-tauri` when feasible in the current environment.
