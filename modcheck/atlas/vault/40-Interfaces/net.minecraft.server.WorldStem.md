---
type: "interface"
fqcn: "net.minecraft.server.WorldStem"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.WorldStem

System: [[20-Systems/net.minecraft.server|net.minecraft.server]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `registries()Lnet/minecraft/core/LayeredRegistryAccess;` | `` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `registries()Lnet/minecraft/core/LayeredRegistryAccess;` | `` | client | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `resourceManager()Lnet/minecraft/server/packs/resources/CloseableResourceMan` | `` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (13, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.server.WorldStem extends java.lang.Record implements java.lang.AutoCloseable {
    private final net.minecraft.server.packs.resources.CloseableResourceManager resourceManager;
    private final net.minecraft.server.ReloadableServerResources dataPackResources;
    private final net.minecraft.core.LayeredRegistryAccess<net.minecraft.server.RegistryLayer> registries;
    private final net.minecraft.world.level.storage.LevelDataAndDimensions$WorldDataAndGenSettings worldDataAndGenSettings;
    public net.minecraft.server.WorldStem(net.minecraft.server.packs.resources.CloseableResourceManager, net.minecraft.server.ReloadableServerResources, net.minecraft.core.LayeredRegistryAccess<net.minecraft.server.RegistryLayer>, net.minecraft.world.level.storage.LevelDataAndDimensions$WorldDataAndGenSettings);
    public void close();
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.server.packs.resources.CloseableResourceManager resourceManager();
    public net.minecraft.server.ReloadableServerResources dataPackResources();
    public net.minecraft.core.LayeredRegistryAccess<net.minecraft.server.RegistryLayer> registries();
    public net.minecraft.world.level.storage.LevelDataAndDimensions$WorldDataAndGenSettings worldDataAndGenSettings();
}
```
