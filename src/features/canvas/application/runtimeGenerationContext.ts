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

    if (model.runtimeProvider.protocol === 'xais-task') {
      const submitBaseUrl = trim(model.runtimeProvider.submitBaseUrl);
      const waitBaseUrl = trim(model.runtimeProvider.waitBaseUrl);
      const assetBaseUrl = trim(model.runtimeProvider.assetBaseUrl);
      const isConfigured =
        apiKey.length > 0 &&
        submitBaseUrl.length > 0 &&
        waitBaseUrl.length > 0 &&
        assetBaseUrl.length > 0 &&
        remoteModelId.length > 0;

      return {
        isConfigured,
        providerRuntime: isConfigured
          ? {
              ...model.runtimeProvider,
              apiKey,
              submitBaseUrl,
              waitBaseUrl,
              assetBaseUrl,
              remoteModelId,
            }
          : undefined,
      };
    }

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
