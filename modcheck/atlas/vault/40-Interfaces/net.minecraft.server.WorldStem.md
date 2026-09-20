---
type: "interface"
fqcn: "net.minecraft.server.WorldStem"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.WorldStem

System: [[20-Systems/net.minecraft.server|net.minecraft.server]]

`record` public final; extends `java/lang/Record`; implements `java/lang/AutoCloseable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `registries` | `()Lnet/minecraft/core/LayeredRegistryAccess;` | exact | invokevirtual@3 in `MinecraftServerMixin.saveRegistryEntryInfo` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `registries` | `()Lnet/minecraft/core/LayeredRegistryAccess;` | exact | invokevirtual@10 in `WorldOpenFlowsMixin.injectHereForCustomScreen` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `resourceManager` | `()Lnet/minecraft/server/packs/resources/CloseableResourceManager;` | exact | invokevirtual@3 in `MinecraftServerMixin.init` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (4 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final resourceManager : Lnet/minecraft/server/packs/resources/CloseableResourceManager;
private final dataPackResources : Lnet/minecraft/server/ReloadableServerResources;
private final registries : Lnet/minecraft/core/LayeredRegistryAccess;
private final worldDataAndGenSettings : Lnet/minecraft/world/level/storage/LevelDataAndDimensions$WorldDataAndGenSettings;
public <init>(Lnet/minecraft/server/packs/resources/CloseableResourceManager;Lnet/minecraft/server/ReloadableServerResources;Lnet/minecraft/core/LayeredRegistryAccess;Lnet/minecraft/world/level/storage/LevelDataAndDimensions$WorldDataAndGenSettings;)V
public close()V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public resourceManager()Lnet/minecraft/server/packs/resources/CloseableResourceManager;
public dataPackResources()Lnet/minecraft/server/ReloadableServerResources;
public registries()Lnet/minecraft/core/LayeredRegistryAccess;
public worldDataAndGenSettings()Lnet/minecraft/world/level/storage/LevelDataAndDimensions$WorldDataAndGenSettings;
```
