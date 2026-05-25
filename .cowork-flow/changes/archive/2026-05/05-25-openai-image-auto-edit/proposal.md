# OpenAI Image Auto Edit

## Goal

Adjust image generation UX so OpenAI Image providers use one Generate action: when reference images are resolved, Generate sends an edit request; when no reference image is resolved, Generate sends a normal generation request. Also keep the Settings custom provider add button visible when nearby text is long.

## Requirements

- Remove the separate image-node Edit button for openai-image providers.
- For openai-image runtime providers, clicking Generate submits action edit only when incoming reference images are non-empty.
- For openai-image runtime providers without reference images, clicking Generate does not submit edit action.
- Non-openai-image providers keep existing Generate behavior.
- The generation submitting guard and immediate disabled feedback remain intact.
- The custom providers page Add Provider button must not shrink or be pushed out by left-side heading/description text.

## Non-Goals

- Do not change backend endpoint behavior.
- Do not add a new image picker.
- Do not redesign settings page visuals.

## Success Criteria

- ImageEditNode tests show openai-image Generate submits edit action with reference images.
- ImageEditNode tests show openai-image Generate omits edit action without reference images.
- ImageEditNode no longer renders an Edit button.
- CustomProvidersPage test or layout assertion covers a non-shrinking add button/header layout.
- Focused Vitest tests pass.
