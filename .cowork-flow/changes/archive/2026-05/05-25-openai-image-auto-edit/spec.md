# OpenAI Image Auto Edit Spec

## Behavior

### Image generation node

- The image generation node exposes a single primary Generate action for openai-image models.
- When generationContext.providerRuntime.protocol is openai-image and resolved incomingImages.length > 0, the Generate action must submit payload field ction:  edit and include the existing eferenceImages list.
- When generationContext.providerRuntime.protocol is openai-image and incomingImages.length === 0, the Generate action must omit ction: edit and submit a normal generation payload.
- The node must not render a separate 
ode.imageEdit.edit button.
- The existing local submitting state continues to disable generation controls immediately during request setup.

### Settings custom providers page

- The Add Provider button in the custom providers page header must remain visible and keep its intrinsic width when the adjacent title or description text is long.
- The text area may wrap or shrink, but it must not push the button out of view or compress the button label.

## Acceptance Criteria

- A focused ImageEditNode test fails before implementation when Generate with openai-image references does not submit ction: edit.
- A focused ImageEditNode test fails before implementation if a separate Edit button is still rendered.
- A focused CustomProvidersPage layout/class test fails before implementation when the add button is not protected from shrinking.
- Existing non-reference openai-image generation remains a generation request.
