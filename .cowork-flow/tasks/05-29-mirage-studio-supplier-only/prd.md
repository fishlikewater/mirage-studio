# PRD: mirage-studio rename and supplier-only configuration

## Goal

Update the project identity and app-facing brand:

- Project/package name: `mirage-studio`.
- Packaged app and visible UI name: `天工`.
- Settings and model configuration should no longer expose built-in API key configuration. Supplier configuration is the only user-facing configuration entry for generation providers.

## Assumptions

- `mirage-studio` is the technical project/package name used in package manifests, Rust crate/package metadata, log paths, upload paths, and current development docs.
- `天工` is the product name shown in Tauri bundle metadata, browser title, title bar, About panel, and localized UI strings.
- Removing built-in API configuration means removing the Settings API Config category and preventing built-in API-key-only providers from appearing as runtime model choices. Custom suppliers remain supported through OpenAPI, XAIS Task, and OpenAI Image protocols.
- Historical archived superpowers/cowork documents do not need a broad rewrite unless they are part of current product-facing documentation.

## Scope

- Update package/Tauri/Rust metadata and current UI strings.
- Update current docs and update/release URLs only where they describe the current project identity.
- Remove the built-in provider API settings UI and redirect missing-configuration prompts to supplier settings.
- Adjust runtime model registry tests so visible model/provider choices are supplier-only.

## Out of Scope

- Rebranding icons or screenshots.
- Removing backend built-in provider implementations unless they become unused and safely removable after frontend/runtime changes.
- Changing release ownership without an existing authoritative repository owner.

## Verification

- Targeted unit tests for settings dialog, runtime registry, runtime generation context, and model parameter controls.
- `npm run build`.
- `cargo check` for Tauri/Rust metadata and crate rename.
- Search audit for old project/product names and built-in API config UI references.
