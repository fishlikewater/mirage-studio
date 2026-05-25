# OpenAI Image Protocol Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a custom-provider `openai-image` protocol that supports OpenAI Images generation and explicit edit actions using existing `@`-resolved reference images.

**Architecture:** Reuse the existing custom-provider model pipeline and add a small action flag to the generation payload. Frontend config treats `openai-image` like an OpenAPI-style Base URL/API Key connection, while Rust routes the protocol to a focused OpenAI Images adapter that builds JSON generation requests, multipart edit requests, and parses `b64_json` or URL results.

**Tech Stack:** React, TypeScript, Vitest, Tauri commands, Rust, reqwest, serde_json.

---

## File Structure

- Modify `src/stores/customProviderConfig.ts`: add `openai-image` protocol normalization and validation behavior.
- Modify `src/stores/customProviderConfig.test.ts`: cover `openai-image` provider normalization and validation.
- Modify `src/features/canvas/models/types.ts`: extend runtime protocol type.
- Modify `src/features/canvas/models/runtimeRegistry.ts`: carry `openai-image` runtime metadata.
- Modify `src/features/canvas/models/runtimeRegistry.test.ts`: assert `openai-image` runtime config.
- Modify `vitest.config.ts`: constrain Vitest test discovery to `src/**` so project-local worktrees are not collected as part of this workspace.
- Modify `src/components/settings/CustomProviderEditorDialog.tsx`: add protocol option and OpenAI image description text.
- Modify `src/components/settings/CustomProviderEditorDialog.test.tsx`: cover selecting and saving `openai-image`.
- Modify `src/components/settings/CustomProvidersPage.tsx`: summarize `openai-image`.
- Modify `src/i18n/locales/zh.json` and `src/i18n/locales/en.json`: add UI labels.
- Modify `src/features/canvas/application/ports.ts`, `src/features/canvas/application/buildNodeGeneratePayload.ts`, `src/commands/ai.ts`, `src/features/canvas/infrastructure/tauriAiGateway.ts`: carry `action?: "generate" | "edit"` without changing existing callers.
- Modify `src/features/canvas/application/buildNodeGeneratePayload.test.ts`: cover action passthrough.
- Modify `src/features/canvas/nodes/ImageEditNode.tsx`: add an edit button for `openai-image`, reusing `incomingImages` from existing `@`/graph reference logic.
- Modify `src/features/canvas/nodes/ImageEditNode.test.tsx`: cover edit button visibility, disabled state, and `action: "edit"` submission.
- Modify `src-tauri/src/ai/mod.rs` and `src-tauri/src/commands/ai.rs`: carry action and route `openai-image`.
- Add `src-tauri/src/ai/providers/openai_image/mod.rs`: OpenAI Images request builder and response parser.
- Modify `src-tauri/src/ai/providers/mod.rs`: expose the new module.

## Current Execution Status

- Spec approved by user on 2026-05-21.
- Task 1 completed: frontend protocol/runtime tests red-green verified.
- Task 2 completed: settings UI option/summary tests red-green verified.
- Task 3 completed: frontend action payload and ImageEditNode edit button tests red-green verified.
- Implementation complete through Task 6 checks where runnable.
- Rust adapter/routing compile checks pass, but runtime Rust unit test execution is blocked by the local Windows test-binary entrypoint issue (`STATUS_ENTRYPOINT_NOT_FOUND`), which also reproduces on existing `xais_task` and `openapi_compat` tests.
- Official docs check on 2026-05-21: OpenAI Images edit endpoint uses multipart image file parts, so backend adapter will use multipart for real OpenAI compatibility instead of the earlier JSON `images` draft.

---

### Task 1: Frontend Protocol Config

**Files:**
- Modify: `src/stores/customProviderConfig.ts`
- Modify: `src/stores/customProviderConfig.test.ts`
- Modify: `src/features/canvas/models/types.ts`
- Modify: `src/features/canvas/models/runtimeRegistry.ts`
- Modify: `src/features/canvas/models/runtimeRegistry.test.ts`

- [x] **Step 1: Write failing config/runtime tests**

Add tests that describe the new protocol before production changes:

```ts
it('normalizes openai-image providers into openapi-style connection fields', () => {
  const providers = normalizeCustomProviders([
    {
      id: ' openai image ',
      name: 'OpenAI Images',
      protocol: 'openai-image',
      connection: {
        openapi: {
          baseUrl: ' https://api.openai.com/v1/ ',
          apiKey: ' sk-test ',
        },
      },
      models: [
        {
          id: 'gpt-image',
          displayName: 'GPT Image',
          remoteModelId: 'gpt-image-1',
          enabled: true,
        },
      ],
    } as unknown as never,
  ]);

  expect(providers[0]).toMatchObject({
    id: 'openai-image',
    protocol: 'openai-image',
    baseUrl: 'https://api.openai.com/v1',
    apiKey: 'sk-test',
    connection: {
      openapi: {
        baseUrl: 'https://api.openai.com/v1',
        apiKey: 'sk-test',
      },
    },
  });
});
```

Add a runtime registry test:

```ts
it('exposes openai-image runtime provider metadata', () => {
  const providers = listRuntimeModelProviders(openAiImageProviders);
  const runtimeModels = listRuntimeImageModels(openAiImageProviders);
  const model = runtimeModels.find(
    (item) => item.id === 'custom-provider:openai-images:gpt-image'
  );

  expect(providers[providers.length - 1]).toMatchObject({
    protocol: 'openai-image',
    configured: true,
  });
  expect(model?.runtimeProvider).toMatchObject({
    kind: 'custom-provider',
    protocol: 'openai-image',
    baseUrl: 'https://api.openai.com/v1',
    apiKey: 'sk-openai',
    remoteModelId: 'gpt-image-1',
  });
});
```

- [x] **Step 2: Verify RED**

Run:

```bash
rtk npm exec vitest run src/stores/customProviderConfig.test.ts src/features/canvas/models/runtimeRegistry.test.ts
```

Expected: FAIL because `openai-image` is normalized to `openapi` or rejected by TypeScript.

- [x] **Step 3: Implement protocol config**

Change these types:

```ts
export type CustomProviderProtocol = 'openapi' | 'xais-task' | 'openai-image';
export type RuntimeCustomProviderProtocol = 'openapi' | 'xais-task' | 'openai-image';
```

Update normalization:

```ts
function normalizeProtocol(value: string | null | undefined): CustomProviderProtocol {
  if (value === 'xais-task' || value === 'openai-image') {
    return value;
  }
  return DEFAULT_PROTOCOL;
}
```

Keep `openai-image` in the `openapi` connection branch so it uses Base URL/API Key.

- [x] **Step 4: Verify GREEN**

Run:

```bash
rtk npm exec vitest run src/stores/customProviderConfig.test.ts src/features/canvas/models/runtimeRegistry.test.ts
```

Expected: PASS.

---

### Task 2: Settings UI Labels

**Files:**
- Modify: `src/components/settings/CustomProviderEditorDialog.tsx`
- Modify: `src/components/settings/CustomProviderEditorDialog.test.tsx`
- Modify: `src/components/settings/CustomProvidersPage.tsx`
- Modify: `src/i18n/locales/zh.json`
- Modify: `src/i18n/locales/en.json`

- [x] **Step 1: Write failing settings tests**

Add a dialog test:

```ts
it('switches to openai-image fields and saves openapi-style connection data', async () => {
  const user = userEvent.setup();
  const onSave = vi.fn();

  render(
    <CustomProviderEditorDialog
      isOpen
      mode="create"
      initialProvider={null}
      onClose={vi.fn()}
      onSave={onSave}
    />
  );

  await user.click(screen.getByRole('button', { name: 'settings.customProviderProtocol' }));
  await user.click(screen.getByRole('option', { name: 'settings.customProviderProtocolOpenaiImage' }));
  expect(screen.getByLabelText('settings.customProviderBaseUrl')).toBeInTheDocument();

  await user.type(screen.getByLabelText('settings.customProviderName'), 'OpenAI Images');
  await user.type(screen.getByLabelText('settings.customProviderBaseUrl'), 'https://api.openai.com/v1');
  await user.type(screen.getByLabelText('settings.customProviderApiKey'), 'sk-openai');
  await user.type(screen.getByLabelText('settings.customProviderModelName'), 'GPT Image');
  await user.type(screen.getByLabelText('settings.customProviderModelId'), 'gpt-image-1');
  await user.click(screen.getByRole('button', { name: 'common.save' }));

  expect(onSave.mock.calls[0][0]).toMatchObject({
    protocol: 'openai-image',
    connection: {
      openapi: {
        baseUrl: 'https://api.openai.com/v1',
        apiKey: 'sk-openai',
      },
    },
  });
});
```

- [x] **Step 2: Verify RED**

Run:

```bash
rtk npm exec vitest run src/components/settings/CustomProviderEditorDialog.test.tsx src/components/settings/CustomProvidersPage.test.tsx
```

Expected: FAIL because the new option/label does not exist.

- [x] **Step 3: Implement settings UI**

Add protocol option:

```tsx
<option value="openai-image">{t('settings.customProviderProtocolOpenaiImage')}</option>
```

Use `draft.protocol === 'xais-task'` as the only XAIS-specific branch; both `openapi` and `openai-image` use Base URL/API Key fields. Add description key selection for `openai-image`.

Update `CustomProvidersPage` summary:

```ts
if (provider.protocol === 'openai-image') {
  return t('settings.customProviderProtocolOpenaiImage');
}
```

Add i18n keys:

```json
"customProviderProtocolOpenaiImage": "OpenAI Image",
"customProviderConnectionOpenaiImageDesc": "Use OpenAI-compatible Images API generation and edit endpoints."
```

- [x] **Step 4: Verify GREEN**

Run:

```bash
rtk npm exec vitest run src/components/settings/CustomProviderEditorDialog.test.tsx src/components/settings/CustomProvidersPage.test.tsx
```

Expected: PASS.

---

### Task 3: Frontend Action Payload and Image Edit Button

**Files:**
- Modify: `src/features/canvas/application/ports.ts`
- Modify: `src/features/canvas/application/buildNodeGeneratePayload.ts`
- Modify: `src/features/canvas/application/buildNodeGeneratePayload.test.ts`
- Modify: `src/commands/ai.ts`
- Modify: `src/features/canvas/infrastructure/tauriAiGateway.ts`
- Modify: `src/features/canvas/nodes/ImageEditNode.tsx`
- Modify: `src/features/canvas/nodes/ImageEditNode.test.tsx`
- Modify: `src/i18n/locales/zh.json`
- Modify: `src/i18n/locales/en.json`

- [x] **Step 1: Write failing payload and node tests**

Add payload test:

```ts
it('keeps explicit edit action for openai-image requests', () => {
  const payload = buildNodeGeneratePayload({
    prompt: '改成水彩',
    requestModel: 'custom-provider:openai-images:gpt-image',
    size: '1K',
    aspectRatio: '1:1',
    referenceImages: ['C:/tmp/ref.png'],
    providerRuntime: {
      kind: 'custom-provider',
      protocol: 'openai-image',
      baseUrl: 'https://api.openai.com/v1',
      apiKey: 'sk-openai',
      remoteModelId: 'gpt-image-1',
    },
    action: 'edit',
  });

  expect(payload.action).toBe('edit');
});
```

Add `ImageEditNode` tests:

```ts
it('shows disabled edit button for openai-image without @ reference images', () => {
  renderNode({
    model: 'custom-provider:openai-images:gpt-image',
  });

  expect(screen.getByRole('button', { name: 'node.imageEdit.edit' })).toBeDisabled();
});

it('submits edit action with @ resolved reference images for openai-image', async () => {
  const user = userEvent.setup();
  renderNode({
    model: 'custom-provider:openai-images:gpt-image',
    prompt: '@图1 改成水彩',
  });

  await user.click(screen.getByRole('button', { name: 'node.imageEdit.edit' }));

  expect(mockSubmitGenerateImageJob).toHaveBeenCalledWith(
    expect.objectContaining({
      action: 'edit',
      referenceImages: ['source-image-path-or-url'],
    })
  );
});
```

Use the existing test helpers/mocks in `ImageEditNode.test.tsx`; adapt the expected reference image value to the helper fixture already used by that file.

- [x] **Step 2: Verify RED**

Run:

```bash
rtk npm exec vitest run src/features/canvas/application/buildNodeGeneratePayload.test.ts src/features/canvas/nodes/ImageEditNode.test.tsx
```

Expected: FAIL because `action` and the edit button do not exist.

- [x] **Step 3: Implement payload action**

Add:

```ts
export type GenerateImageAction = 'generate' | 'edit';
```

to the frontend port/types and pass `action` through `buildNodeGeneratePayload`, `commands/ai.ts`, and `tauriAiGateway.ts`.

Default action is omitted or `"generate"` for existing callers.

- [x] **Step 4: Implement edit button**

Refactor `handleGenerate` to accept an action:

```ts
const handleGenerate = useCallback(async (action: GenerateImageAction = 'generate') => {
  if (action === 'edit' && incomingImages.length === 0) {
    return;
  }
  // existing logic
  action,
}, [...]);
```

Add derived state:

```ts
const isOpenAiImageProtocol = generationContext.providerRuntime?.protocol === 'openai-image';
const canEditWithOpenAiImage = incomingImages.length > 0 && generationContext.isConfigured;
```

Add button next to generate:

```tsx
{isOpenAiImageProtocol && (
  <UiButton
    onClick={(event) => {
      event.stopPropagation();
      void handleGenerate('edit');
    }}
    disabled={!canEditWithOpenAiImage}
    variant="muted"
    className={`shrink-0 ${NODE_CONTROL_PRIMARY_BUTTON_CLASS}`}
  >
    <Sparkles className={NODE_CONTROL_ICON_CLASS} strokeWidth={2.8} />
    {t('node.imageEdit.edit')}
  </UiButton>
)}
```

Keep Ctrl/Cmd+Enter mapped to normal generation.

- [x] **Step 5: Verify GREEN**

Run:

```bash
rtk npm exec vitest run src/features/canvas/application/buildNodeGeneratePayload.test.ts src/features/canvas/nodes/ImageEditNode.test.tsx
```

Expected: PASS.

---

### Task 4: Rust OpenAI Image Adapter

**Files:**
- Add: `src-tauri/src/ai/providers/openai_image/mod.rs`
- Modify: `src-tauri/src/ai/providers/mod.rs`
- Modify: `src-tauri/src/ai/mod.rs`

- [x] **Step 1: Write failing Rust adapter tests**

Create tests in the new module:

```rust
#[test]
fn build_generation_body_maps_prompt_model_and_count() {
    let body = build_generation_body(&request(), &runtime()).expect("body");
    assert_eq!(body["model"], "gpt-image-1");
    assert!(body["prompt"].as_str().unwrap().contains("改成电影海报"));
    assert_eq!(body["n"], 1);
}

#[test]
fn collect_edit_images_maps_reference_images() {
    let mut request = request();
    request.reference_images = Some(vec!["data:image/png;base64,abc".to_string()]);
    let references = collect_edit_images(&request).expect("references");
    assert_eq!(references.len(), 1);
    assert_eq!(references[0].mime_type, "image/png");
    assert_eq!(references[0].bytes, b"abc");
}

#[test]
fn collect_edit_images_rejects_missing_reference_images() {
    let error = collect_edit_images(&request()).expect_err("missing refs should fail");
    assert!(error.to_string().contains("reference images"));
}

#[test]
fn parse_image_response_prefers_b64_json() {
    let result = parse_image_response(&json!({
        "data": [{ "b64_json": "abc123" }]
    })).expect("image");

    assert_eq!(result, "data:image/png;base64,abc123");
}

#[test]
fn parse_image_response_accepts_url() {
    let result = parse_image_response(&json!({
        "data": [{ "url": "https://example.com/result.png" }]
    })).expect("image");

    assert_eq!(result, "https://example.com/result.png");
}
```

- [x] **Step 2: Verify RED**

Run:

```bash
rtk cargo test openai_image
```

from `src-tauri`.

Expected: FAIL because the module/functions do not exist.

- [x] **Step 3: Implement adapter**

Create focused helpers:

```rust
pub fn build_generation_body(request: &GenerateRequest, runtime: &RuntimeProviderConfig) -> Result<Value, AIError>
pub fn collect_edit_images(request: &GenerateRequest) -> Result<Vec<OpenAiEditImage>, AIError>
pub fn parse_image_response(value: &Value) -> Result<String, AIError>
pub async fn generate(request: &GenerateRequest, runtime: &RuntimeProviderConfig) -> Result<String, AIError>
pub async fn edit(request: &GenerateRequest, runtime: &RuntimeProviderConfig) -> Result<String, AIError>
```

Use endpoints:

```rust
format!("{}/images/generations", base_url.trim_end_matches('/'))
format!("{}/images/edits", base_url.trim_end_matches('/'))
```

Use multipart image file parts for edit:

```rust
multipart::Form::new()
    .text("model", remote_model_id)
    .text("prompt", request.prompt.clone())
    .text("n", "1")
    .part("image[]", multipart::Part::bytes(image.bytes).mime_str(&image.mime_type)?)
```

- [ ] **Step 4: Verify GREEN**

Run:

```bash
rtk cargo test openai_image
```

Expected: PASS.

---

### Task 5: Tauri Command Routing

**Files:**
- Modify: `src-tauri/src/ai/mod.rs`
- Modify: `src-tauri/src/commands/ai.rs`
- Modify: `src-tauri/src/ai/providers/openai_image/mod.rs`

- [x] **Step 1: Write failing command tests**

Add unit tests for pure protocol/action helpers:

```rust
#[test]
fn resolves_custom_openai_image_runtime() {
    let runtime = crate::ai::RuntimeProviderConfig {
        kind: "custom-provider".to_string(),
        provider_profile_id: None,
        provider_display_name: None,
        protocol: Some("openai-image".to_string()),
        base_url: Some("https://api.openai.com/v1".to_string()),
        api_key: Some("sk-openai".to_string()),
        submit_base_url: None,
        wait_base_url: None,
        asset_base_url: None,
        default_output_format: None,
        remote_model_id: Some("gpt-image-1".to_string()),
    };

    assert!(is_custom_openai_image_runtime(Some(&runtime)));
}
```

Add action default test if helper is added:

```rust
#[test]
fn generate_request_action_defaults_to_generate() {
    assert_eq!(normalize_action(None), "generate");
}
```

- [x] **Step 2: Verify RED**

Run:

```bash
rtk cargo test custom_openai_image
```

from `src-tauri`.

Expected: FAIL because route helper/action does not exist.

- [x] **Step 3: Implement routing**

Add `action: Option<String>` to DTO and `GenerateRequest`.

Add:

```rust
fn is_custom_openai_image_runtime(runtime: Option<&crate::ai::RuntimeProviderConfig>) -> bool {
    matches!(resolve_custom_provider_protocol(runtime), Some("openai-image"))
}

fn normalize_action(action: Option<&str>) -> &str {
    match action {
        Some("edit") => "edit",
        _ => "generate",
    }
}
```

In `submit_generate_image_job` and `generate_image`, route custom `openai-image` before built-in provider resolution:

```rust
let result = match normalize_action(req.action.as_deref()) {
    "edit" => crate::ai::providers::openai_image::edit(&req, &runtime).await,
    _ => crate::ai::providers::openai_image::generate(&req, &runtime).await,
};
```

Treat it as non-resumable, like `openapi_compat`.

- [ ] **Step 4: Verify GREEN**

Run:

```bash
rtk cargo test custom_openai_image
rtk cargo test openai_image
```

Expected: PASS.

---

### Task 6: Integration Checks and Status Sync

**Files:**
- Modify: `.cowork-flow/changes/openai-image-protocol/change.yaml`
- Create or modify: `.cowork-flow/tasks/05-21-openai-image-protocol/prd.md`
- Create or modify: `.cowork-flow/tasks/05-21-openai-image-protocol/implement.jsonl`
- Create or modify: `.cowork-flow/tasks/05-21-openai-image-protocol/check.jsonl`
- Create or modify: `.cowork-flow/tasks/05-21-openai-image-protocol/debug.jsonl`

- [ ] **Step 1: Create/bind task and context**

Run:

```bash
rtk python3 ./.cowork-flow/scripts/task.py create "OpenAI image protocol" --slug openai-image-protocol
rtk python3 ./.cowork-flow/scripts/task.py init-context .cowork-flow/tasks/05-21-openai-image-protocol fullstack
```

Write `prd.md` summarizing the approved spec and plan. Add context entries for:

- `.cowork-flow/changes/openai-image-protocol/proposal.md`
- `.cowork-flow/changes/openai-image-protocol/specs/custom-provider-openai-image/spec.md`
- `.cowork-flow/changes/openai-image-protocol/design.md`
- `.cowork-flow/plans/2026-05-21-openai-image-protocol.md`
- `.cowork-flow/spec/frontend/type-safety.md`
- `.cowork-flow/spec/frontend/component-guidelines.md`
- `.cowork-flow/spec/backend/error-handling.md`
- `.cowork-flow/spec/backend/quality-guidelines.md`

- [x] **Step 2: Run focused frontend tests**

Run:

```bash
rtk npm exec vitest run src/stores/customProviderConfig.test.ts src/features/canvas/models/runtimeRegistry.test.ts src/components/settings/CustomProviderEditorDialog.test.tsx src/components/settings/CustomProvidersPage.test.tsx src/features/canvas/application/buildNodeGeneratePayload.test.ts src/features/canvas/nodes/ImageEditNode.test.tsx
```

Expected: PASS.

- [ ] **Step 3: Run focused Rust tests**

Run from `src-tauri`:

```bash
rtk cargo test openai_image
rtk cargo test custom_openai_image
```

Expected: PASS.

- [x] **Step 4: Run broader verification**

Run:

```bash
rtk npm run build
```

Run from `src-tauri`:

```bash
rtk cargo check
```

Expected: PASS.

- [x] **Step 5: Sync metadata**

Update `change.yaml`:

```yaml
status: active
plan: .cowork-flow/plans/2026-05-21-openai-image-protocol.md
task: .cowork-flow/tasks/05-21-openai-image-protocol
```

Run:

```bash
rtk python3 ./.cowork-flow/scripts/change.py validate openai-image-protocol
rtk python3 ./.cowork-flow/scripts/task.py validate .cowork-flow/tasks/05-21-openai-image-protocol
```

Expected: both pass.

---

## Self-Review

- Spec coverage: protocol configuration, generation action, edit action, `@` reference image reuse, backend endpoints, b64/url parsing, and validation are all mapped to tasks.
- Placeholder scan: no unresolved markers or open-ended implementation placeholders.
- Type consistency: frontend `action?: "generate" | "edit"` maps to Rust `action: Option<String>` and defaults to generation.
