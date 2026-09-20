---
type: "mechanism"
module: "fabric-resource-conditions-api-v1"
version: "6.1.5+3434d6d95d"
sha256: "1d7d9bea7e90eacfb57ac9e6d1f09036c8f118fc5ab50328f55c4d0b8519782c"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-resource-conditions-api-v1

**Version** `6.1.5+3434d6d95d` -- **artifact sha256** `1d7d9bea7e90eacfb57ac9e6d1f09036c8f118fc5ab50328f55c4d0b8519782c`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3"}`
- entrypoints: `{"main": ["net.fabricmc.fabric.impl.resource.conditions.ResourceConditionsImpl"]}`
- mixin configs: `["fabric-resource-conditions-api-v1.mixins.json"]`
- access widener: `fabric-resource-conditions-api-v1.classtweaker`

## Events this module publishes

- none found by extraction

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.data.DataProvider|DataProvider]] | `lambda$static$0` | injects_into `@Inject at HEAD` | both | `DataProviderMixin.fabric_injectResourceConditionsSortOrder` |
| [[40-Interfaces/net.minecraft.resources.RegistryLoadTask_PendingRegistration|RegistryLoadTask$PendingRegistration]] | `loadFromResource` | injects_into `@Inject at INVOKE Lcom/mojang/serialization/Decoder;parse(Lcom/mojang/serialization/DynamicOps;Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;` | both | `RegistryLoadTaskPendingRegistrationMixin.loadFromResource` |
| [[40-Interfaces/net.minecraft.server.ReloadableServerResources|ReloadableServerResources]] | `loadResources` | injects_into `@Inject at HEAD` | both | `ReloadableServerResourcesMixin.hookReload` |
| [[40-Interfaces/net.minecraft.server.packs.repository.Pack|Pack]] | `readPackMetadata` | injects_into `@ModifyVariable at STORE` | both | `PackMixin.applyOverlayConditions` |
| [[40-Interfaces/net.minecraft.server.packs.resources.SimpleJsonResourceReloadListener|SimpleJsonResourceReloadListener]] | `lambda$prepare$0` | injects_into `@Inject at HEAD` | both | `SimpleJsonResourceReloadListenerMixin.skipData` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.resource.conditions.v1.ResourceCondition|ResourceCondition]] (interface, 6 members)
- [[40-Interfaces/net.fabricmc.fabric.api.resource.conditions.v1.ResourceConditionType|ResourceConditionType]] (interface, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.resource.conditions.v1.ResourceConditions|ResourceConditions]] (class, 18 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
