---
type: "interface"
fqcn: "net.minecraft.resources.RegistryDataLoader$RegistryData"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.resources.RegistryDataLoader$RegistryData

System: [[20-Systems/net.minecraft.resources|net.minecraft.resources]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/resources/ResourceKey;Lcom/mojang/serialization/Codec;` | exact | invokespecial@64 in `DynamicRegistriesImpl.register` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/resources/ResourceKey;Lcom/mojang/serialization/Codec;` | exact | invokespecial@55 in `DynamicRegistriesImpl.addSyncedRegistry` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/resources/ResourceKey;Lcom/mojang/serialization/Codec;` | exact | invokespecial@64 in `DynamicRegistriesImpl.registerReloadable` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `elementCodec` | `()Lcom/mojang/serialization/Codec;` | exact | invokevirtual@21 in `FabricDynamicRegistryProvider$RegistryEntries.create` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@1 in `FabricDynamicRegistryProvider$Entries.lambda$new$1` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@2 in `FabricDynamicRegistryProvider$Entries.lambda$new$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@2 in `FabricDynamicRegistryProvider$RegistryEntries.create` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@17 in `FabricDynamicRegistryProvider$RegistryEntries.create` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@85 in `FabricDataGenHelper.createWorldLookupProvider` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@97 in `FabricDataGenHelper.createWorldLookupProvider` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@88 in `FabricDataGenHelper.createReloadableLookupProvider` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@100 in `FabricDataGenHelper.createReloadableLookupProvider` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@1 in `RegistryDataLoaderMixin.lambda$loadFromResources$0` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@121 in `DynamicRegistriesImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@4 in `RegistryDataCollectorMixin.lambda$skipEmptyRegistries$0` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@17 in `RegistryDataCollectorMixin.lambda$skipEmptyRegistries$0` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (3 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final key : Lnet/minecraft/resources/ResourceKey;
private final elementCodec : Lcom/mojang/serialization/Codec;
private final validator : Lnet/minecraft/resources/RegistryValidator;
private <init>(Lnet/minecraft/resources/ResourceKey;Lcom/mojang/serialization/Codec;)V
public <init>(Lnet/minecraft/resources/ResourceKey;Lcom/mojang/serialization/Codec;Lnet/minecraft/resources/RegistryValidator;)V
public runWithArguments(Ljava/util/function/BiConsumer;)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public key()Lnet/minecraft/resources/ResourceKey;
public elementCodec()Lcom/mojang/serialization/Codec;
public validator()Lnet/minecraft/resources/RegistryValidator;
```
