# OpenAI Image Protocol Spec

## Scope

Archive compatibility root spec for the openai-image-protocol change. The detailed behavior spec remains in specs/custom-provider-openai-image/spec.md; this root file mirrors the acceptance contract required by the current cowork-flow archive validator.

## Requirements

- Custom provider protocol options include openai-image.
- openai-image providers use OpenAI Images-compatible Base URL, API Key, and remote model ID configuration.
- Generation requests for openai-image route to /images/generations.
- Edit requests for openai-image route to /images/edits and use existing resolved reference images.
- Image node UI shows an Edit action only for openai-image runtime providers.
- The Edit action is disabled when no reference image is resolved.
- Responses parse data[0].b64_json first and can fall back to data[0].url.
- Existing openapi and xais-task protocol behavior remains unchanged.

## Acceptance Criteria

- Settings can select, save, and reload the OpenAI Image protocol.
- Runtime model metadata carries providerRuntime.protocol = openai-image.
- Image generation payload can carry action = edit when the Edit action is used.
- Backend generation and edit routing use the correct OpenAI Images endpoints.
- Focused TypeScript tests pass for provider config, settings UI, runtime metadata, payload building, and image node edit behavior.
- Rust compile checks pass; Rust unit test execution blocker is documented in the implementation plan.
