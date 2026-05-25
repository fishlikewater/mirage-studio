# OpenAI Image Auto Edit

## Goal

Use a single Generate button for openai-image models: automatically call edit when reference images exist, otherwise generate normally. Fix custom provider add button layout so it is not squeezed by nearby text.

## Requirements

- Remove the separate openai-image Edit button from the image generation node.
- Generate with openai-image + reference images submits ction:  edit and existing eferenceImages.
- Generate with openai-image + no reference images omits edit action.
- Preserve non-openai-image generation behavior and submitting feedback.
- Keep Add Provider button visible and non-shrinking in the custom providers settings page header.

## Acceptance Criteria

- [ ] ImageEditNode focused tests cover automatic edit action with references.
- [ ] ImageEditNode focused tests cover normal generation without references.
- [ ] ImageEditNode focused tests confirm no separate Edit button.
- [ ] CustomProvidersPage focused test covers non-shrinking Add Provider button layout.
- [ ] Focused Vitest tests pass.
- [ ] Build passes or any blocker is documented.

## Technical Notes

- Change: .cowork-flow/changes/05-25-openai-image-auto-edit/`n- Plan: .cowork-flow/plans/2026-05-25-openai-image-auto-edit.md`n- Type: L1 frontend behavior/layout change.
