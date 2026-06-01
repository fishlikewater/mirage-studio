import {
  DEFAULT_IMAGE_MODEL_ID,
  getImageModel,
} from './registry';
import type {
  RuntimeImageModelDefinition,
  RuntimeModelProviderDefinition,
} from './types';
import {
  buildCustomProviderModelId,
  isCustomProviderConfigured,
  resolveOpenApiConnection,
  type CustomProviderConfig,
} from '@/stores/customProviderConfig';

const SUPPLIER_CONFIGURATION_PROVIDER_ID = '__unconfigured__';
const SUPPLIER_CONFIGURATION_MODEL_ID = buildCustomProviderModelId(
  SUPPLIER_CONFIGURATION_PROVIDER_ID,
  'configure-supplier'
);

export function buildCustomRuntimeProviderId(providerId: string): string {
  return `custom-provider:${providerId}`;
}

export function listRuntimeModelProviders(
  customProviders: CustomProviderConfig[]
): RuntimeModelProviderDefinition[] {
  const customs = customProviders.map((provider) => ({
    id: buildCustomRuntimeProviderId(provider.id),
    name: provider.name,
    label: provider.name,
    runtimeKind: 'custom-provider' as const,
    configured: isCustomProviderConfigured(provider),
    providerProfileId: provider.id,
    protocol: provider.protocol,
  }));

  return customs;
}

export function listRuntimeImageModels(
  customProviders: CustomProviderConfig[]
): RuntimeImageModelDefinition[] {
  const baseModel = getImageModel(DEFAULT_IMAGE_MODEL_ID);
  const customs = customProviders.flatMap((provider) => {
    const openapiConnection = resolveOpenApiConnection(provider);

    return provider.models
      .filter((model) => model.enabled)
      .map<RuntimeImageModelDefinition>((model) => ({
        ...baseModel,
        id: buildCustomProviderModelId(provider.id, model.id),
        displayName: model.displayName,
        providerId: buildCustomRuntimeProviderId(provider.id),
        description: `${provider.name} / ${model.remoteModelId}`,
        pricing: undefined,
        resolveRequest: ({ referenceImageCount }) => ({
          requestModel: buildCustomProviderModelId(provider.id, model.id),
          modeLabel: referenceImageCount > 0 ? '编辑模式' : '生成模式',
        }),
        runtimeProvider: {
          kind: 'custom-provider',
          providerProfileId: provider.id,
          providerDisplayName: provider.name,
          protocol: provider.protocol,
          baseUrl: openapiConnection.baseUrl,
          apiKey: openapiConnection.apiKey,
          remoteModelId: model.remoteModelId,
        },
        supportsResolutionSelection: false,
      }));
  });

  if (customs.length > 0) {
    return customs;
  }

  return [
    {
      ...baseModel,
      id: SUPPLIER_CONFIGURATION_MODEL_ID,
      displayName: '配置供应商',
      providerId: buildCustomRuntimeProviderId(SUPPLIER_CONFIGURATION_PROVIDER_ID),
      description: '请先在设置中添加供应商和模型。',
      pricing: undefined,
      resolveRequest: () => ({
        requestModel: SUPPLIER_CONFIGURATION_MODEL_ID,
        modeLabel: '供应商未配置',
      }),
      runtimeProvider: {
        kind: 'custom-provider',
        providerProfileId: SUPPLIER_CONFIGURATION_PROVIDER_ID,
        providerDisplayName: '供应商',
        protocol: 'openapi',
        baseUrl: '',
        apiKey: '',
        remoteModelId: '',
      },
      supportsResolutionSelection: false,
    },
  ];
}

export function getRuntimeImageModel(
  modelId: string,
  customProviders: CustomProviderConfig[]
): RuntimeImageModelDefinition {
  const runtimeModels = listRuntimeImageModels(customProviders);
  return (
    runtimeModels.find((model) => model.id === modelId) ??
    runtimeModels[0]
  );
}
