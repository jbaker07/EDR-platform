---
type: "interface"
fqcn: "net.minecraft.core.LayeredRegistryAccess"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.LayeredRegistryAccess

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `compositeAccess` | `()Lnet/minecraft/core/RegistryAccess$Frozen;` | exact | invokevirtual@78 in `CreateWorldScreenMixin.createLevelDataForServers` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `compositeAccess` | `()Lnet/minecraft/core/RegistryAccess$Frozen;` | exact | invokevirtual@5 in `ReloadableServerResourcesMixin.init` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `compositeAccess` | `()Lnet/minecraft/core/RegistryAccess$Frozen;` | exact | invokevirtual@6 in `MinecraftServerMixin.saveRegistryEntryInfo` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `compositeAccess` | `()Lnet/minecraft/core/RegistryAccess$Frozen;` | exact | invokevirtual@13 in `WorldOpenFlowsMixin.injectHereForCustomScreen` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getLayer` | `(Ljava/lang/Object;)Lnet/minecraft/core/RegistryAccess$Frozen;` | exact | invokevirtual@2 in `TagAliasLoader.applyToDynamicRegistries` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |

## Declared members (3 fields, 14 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final keys : Ljava/util/List;
private final values : Ljava/util/List;
private final composite : Lnet/minecraft/core/RegistryAccess$Frozen;
public <init>(Ljava/util/List;)V
private <init>(Ljava/util/List;Ljava/util/List;)V
private getLayerIndexOrThrow(Ljava/lang/Object;)I
public getLayer(Ljava/lang/Object;)Lnet/minecraft/core/RegistryAccess$Frozen;
public getAccessForLoading(Ljava/lang/Object;)Lnet/minecraft/core/RegistryAccess$Frozen;
public getAccessFrom(Ljava/lang/Object;)Lnet/minecraft/core/RegistryAccess$Frozen;
private getCompositeAccessForLayers(II)Lnet/minecraft/core/RegistryAccess$Frozen;
public replaceFrom(Ljava/lang/Object;[Lnet/minecraft/core/RegistryAccess$Frozen;)Lnet/minecraft/core/LayeredRegistryAccess;
public replaceFrom(Ljava/lang/Object;Ljava/util/List;)Lnet/minecraft/core/LayeredRegistryAccess;
public compositeAccess()Lnet/minecraft/core/RegistryAccess$Frozen;
private static collectRegistries(Ljava/util/stream/Stream;)Ljava/util/Map;
private static synthetic lambda$collectRegistries$0(Ljava/util/Map;Lnet/minecraft/core/RegistryAccess;)V
private static synthetic lambda$collectRegistries$1(Ljava/util/Map;Lnet/minecraft/core/RegistryAccess$RegistryEntry;)V
private static synthetic lambda$new$0(Ljava/util/List;)Ljava/util/List;
```
