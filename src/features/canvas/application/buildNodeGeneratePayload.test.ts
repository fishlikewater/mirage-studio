import { describe, expect, it } from 'vitest';

import { buildNodeGeneratePayload } from './buildNodeGeneratePayload';

describe('buildNodeGeneratePayload', () => {
  it('keeps providerRuntime for custom provider models', () => {
    const payload = buildNodeGeneratePayload({
      prompt: 'sunset landscape',
      requestModel: 'custom-provider:gateway-a:model-main',
      size: '1K',
      aspectRatio: '9:16',
      referenceImages: ['https://example.com/ref.png'],
      extraParams: {},
      providerRuntime: {
        kind: 'custom-provider',
        providerProfileId: 'gateway-a',
        providerDisplayName: 'Acme Gateway',
        protocol: 'openapi',
        baseUrl: 'https://sg2c.dchai.cn/v1',
        apiKey: 'token-1',
        remoteModelId: 'Nano_Banana_Pro_2K_0',
      },
    });

    expect(payload.providerRuntime?.remoteModelId).toBe('Nano_Banana_Pro_2K_0');
    expect(payload.model).toBe('custom-provider:gateway-a:model-main');
  });

  it('keeps explicit edit action for openai-image requests', () => {
    const payload = buildNodeGeneratePayload({
      prompt: 'turn it into watercolor',
      requestModel: 'custom-provider:openai-images:gpt-image',
      size: '1K',
      aspectRatio: '1:1',
      referenceImages: ['source-image-path-or-url'],
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
});
