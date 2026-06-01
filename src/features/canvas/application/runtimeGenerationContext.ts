import type { RuntimeImageModelDefinition, RuntimeProviderConfig } from '@/features/canvas/models';

export interface RuntimeGenerationContext {
  isConfigured: boolean;
  providerRuntime?: RuntimeProviderConfig;
}

function trim(value: string | null | undefined): string {
  return (value ?? '').trim();
}

export function resolveGenerationContext(
  model: RuntimeImageModelDefinition
): RuntimeGenerationContext {
  if (model.runtimeProvider.kind === 'custom-provider') {
    const apiKey = trim(model.runtimeProvider.apiKey);
    const remoteModelId = trim(model.runtimeProvider.remoteModelId);
    const baseUrl = trim(model.runtimeProvider.baseUrl);
    const isConfigured = apiKey.length > 0 && baseUrl.length > 0 && remoteModelId.length > 0;

    return {
      isConfigured,
      providerRuntime: isConfigured
        ? {
            ...model.runtimeProvider,
            apiKey,
            baseUrl,
            remoteModelId,
          }
        : undefined,
    };
  }

  return {
    isConfigured: false,
  };
}
