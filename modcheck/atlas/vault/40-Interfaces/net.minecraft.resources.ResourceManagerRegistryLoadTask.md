---
type: "interface"
fqcn: "net.minecraft.resources.ResourceManagerRegistryLoadTask"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.resources.ResourceManagerRegistryLoadTask

System: [[20-Systems/net.minecraft.resources|net.minecraft.resources]]

`class` public; extends `net/minecraft/resources/RegistryLoadTask`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `lambda$load$2` | `(Lnet/minecraft/resources/FileToIdConverter;Lnet/minecraft/resources/R` | name_only | @ModifyExpressionValue at ['NEW'] | both | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| injects_into | `load` | `(Lnet/minecraft/resources/RegistryOps$RegistryInfoLookup;Ljava/util/co` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| reads | `resourceManager` | `Lnet/minecraft/server/packs/resources/ResourceManager;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | declared |
| reads | `resourceManager` | `Lnet/minecraft/server/packs/resources/ResourceManager;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | declared |
| wraps | `lambda$load$2` | `(Lnet/minecraft/resources/FileToIdConverter;Lnet/minecraft/resources/R` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| wraps | `lambda$load$2` | `(Lnet/minecraft/resources/FileToIdConverter;Lnet/minecraft/resources/R` | name_only | @WrapOperation at ['NEW'] | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| wraps | `lambda$load$2` | `(Lnet/minecraft/resources/FileToIdConverter;Lnet/minecraft/resources/R` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |

## Declared members (2 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final REGISTRATION_INFO_CACHE : Ljava/util/function/Function;
private final resourceManager : Lnet/minecraft/server/packs/resources/ResourceManager;
public <init>(Lnet/minecraft/resources/RegistryDataLoader$RegistryData;Lcom/mojang/serialization/Lifecycle;Ljava/util/Map;Lnet/minecraft/server/packs/resources/ResourceManager;)V
public load(Lnet/minecraft/resources/RegistryOps$RegistryInfoLookup;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;
private synthetic lambda$load$3(Ljava/util/Map;)V
private synthetic lambda$load$1(Lnet/minecraft/resources/RegistryOps$RegistryInfoLookup;Lnet/minecraft/resources/FileToIdConverter;Ljava/util/concurrent/Executor;Ljava/util/Map;)Ljava/util/concurrent/CompletionStage;
private synthetic lambda$load$2(Lnet/minecraft/resources/FileToIdConverter;Lnet/minecraft/resources/RegistryOps;Lnet/minecraft/resources/Identifier;Lnet/minecraft/server/packs/resources/Resource;)Lnet/minecraft/resources/RegistryLoadTask$PendingRegistration;
private synthetic lambda$load$0(Lnet/minecraft/resources/FileToIdConverter;)Ljava/util/Map;
private static synthetic lambda$static$0(Ljava/util/Optional;)Lnet/minecraft/core/RegistrationInfo;
private static synthetic lambda$static$1(Ljava/lang/Boolean;)Lcom/mojang/serialization/Lifecycle;
static <clinit>()V
```
