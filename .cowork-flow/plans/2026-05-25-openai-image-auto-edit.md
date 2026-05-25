# OpenAI Image Auto Edit Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (- [ ]) syntax for tracking.

**Goal:** Use one Generate action for openai-image models and keep the custom provider add button from being squeezed by header text.

**Architecture:** Keep backend protocol behavior unchanged. Move openai-image edit selection into the ImageEditNode Generate path by deriving action from provider protocol plus resolved reference image count; fix settings page layout with local flex/min-width classes.

**Tech Stack:** React, TypeScript, Vitest, Tailwind classes.

---

## Current Execution Status

- Requirements approved by user on 2026-05-25.
- Not suitable for agent-team parallelism: both changes are small, frontend-only, and need tight RED/GREEN feedback in existing tests.
- RED verified: focused tests failed because the old Edit button still rendered, Generate omitted edit action with references, and settings header layout protection was missing.
- GREEN verified: `rtk npm exec vitest run src/features/canvas/nodes/ImageEditNode.test.tsx src/components/settings/CustomProvidersPage.test.tsx` passed 13 tests; `rtk npm run build` passed with existing Vite chunk warnings.

## Files

- Modify src/features/canvas/nodes/ImageEditNode.test.tsx: update openai-image tests from explicit Edit button to automatic Generate action.
- Modify src/features/canvas/nodes/ImageEditNode.tsx: remove separate Edit button and derive ction for Generate.
- Modify src/components/settings/CustomProvidersPage.test.tsx: add layout/class assertion for add button/header protection.
- Modify src/components/settings/CustomProvidersPage.tsx: add min-w-0 text container and non-shrinking add button layout classes.

### Task 1: Image Node Auto Edit

- [x] **Step 1: Write failing tests**

In src/features/canvas/nodes/ImageEditNode.test.tsx, replace the explicit edit-button behavior tests with:

`	s
it('does not render a separate edit button for openai-image models', () => {
  renderNode({ model: 'custom-provider:openai-images:gpt-image' });

  expect(screen.queryByRole('button', { name: 'node.imageEdit.edit' })).not.toBeInTheDocument();
});

it('uses edit action when generating with openai-image references', async () => {
  const user = userEvent.setup();
  vi.mocked(graphImageResolver.collectInputImages).mockReturnValue(['source-image-path-or-url']);

  renderNode({
    model: 'custom-provider:openai-images:gpt-image',
    prompt: '@ͼ1 turn it into watercolor',
  });

  await user.click(screen.getByRole('button', { name: 'canvas.generate' }));

  await waitFor(() => {
    expect(buildNodeGeneratePayload).toHaveBeenCalledWith(
      expect.objectContaining({
        action: 'edit',
        referenceImages: ['source-image-path-or-url'],
      })
    );
  });
});

it('uses normal generation for openai-image models without references', async () => {
  const user = userEvent.setup();

  renderNode({
    model: 'custom-provider:openai-images:gpt-image',
    prompt: 'make a poster',
  });

  await user.click(screen.getByRole('button', { name: 'canvas.generate' }));

  await waitFor(() => {
    expect(buildNodeGeneratePayload).toHaveBeenCalledWith(
      expect.objectContaining({
        action: undefined,
        referenceImages: [],
      })
    );
  });
});
``n
- [x] **Step 2: Verify RED**

Run:

`ash
rtk npm exec vitest run src/features/canvas/nodes/ImageEditNode.test.tsx
` 

Expected: FAIL because the separate Edit button still renders and Generate does not automatically submit ction:  edit for references.

- [x] **Step 3: Implement minimal node change**

In src/features/canvas/nodes/ImageEditNode.tsx:

`	s
const shouldUseOpenAiImageEdit = isOpenAiImageProtocol && incomingImages.length > 0;
``n
In handleGenerate, remove the action parameter and derive payload action:

`	s
const generationAction: GenerateImageAction | undefined = shouldUseOpenAiImageEdit ? 'edit' : undefined;
// ...
action: generationAction,
``n
Remove the separate Edit UiButton block.

- [x] **Step 4: Verify GREEN**

Run the same focused test command. Expected: PASS.

### Task 2: Custom Provider Header Layout

- [x] **Step 1: Write failing layout test**

In src/components/settings/CustomProvidersPage.test.tsx, add a test asserting the header text can shrink and the add button cannot:

`	s
it('keeps the add provider button from shrinking beside long header text', () => {
  renderPage();

  expect(screen.getByTestId('custom-providers-page-heading-copy')).toHaveClass('min-w-0');
  expect(screen.getByRole('button', { name: 'settings.addCustomProvider' })).toHaveClass('shrink-0');
});
``n
If 
enderPage is not the helper name, use the existing page render helper in that file.

- [x] **Step 2: Verify RED**

Run:

`ash
rtk npm exec vitest run src/components/settings/CustomProvidersPage.test.tsx
` 

Expected: FAIL because the test id/classes do not exist yet.

- [x] **Step 3: Implement layout classes**

In src/components/settings/CustomProvidersPage.tsx, wrap heading/description copy in:

`	sx
<div data-testid=custom-providers-page-heading-copy className=min-w-0>
``n
Ensure the Add Provider button has shrink-0.

- [x] **Step 4: Verify GREEN**

Run the same focused settings page test. Expected: PASS.

### Task 3: Integrated Verification and Status Sync

- [x] **Step 1: Run focused tests**

`ash
rtk npm exec vitest run src/features/canvas/nodes/ImageEditNode.test.tsx src/components/settings/CustomProvidersPage.test.tsx
``n
- [x] **Step 2: Run build**

`ash
rtk npm run build
``n
- [x] **Step 3: Sync task/change status**

Update this plan checklist, task status, and change metadata before handoff.

## Self-Review

- Spec coverage: all requested behavior and layout changes map to Task 1 and Task 2.
- Placeholder scan: no TBD or open-ended implementation steps.
- Type consistency: frontend action remains existing GenerateImageAction, with omitted action meaning normal generation.
