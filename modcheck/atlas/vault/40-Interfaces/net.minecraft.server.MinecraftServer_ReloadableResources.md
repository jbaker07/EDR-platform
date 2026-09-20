---
type: "interface"
fqcn: "net.minecraft.server.MinecraftServer$ReloadableResources"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.MinecraftServer$ReloadableResources

System: [[20-Systems/net.minecraft.server|net.minecraft.server]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `managers()Lnet/minecraft/server/ReloadableServerResources;` | `` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `resourceManager()Lnet/minecraft/server/packs/resources/CloseableResourceMan` | `` | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |

## Declared members (9, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
final class net.minecraft.server.MinecraftServer$ReloadableResources extends java.lang.Record implements java.lang.AutoCloseable {
    private final net.minecraft.server.packs.resources.CloseableResourceManager resourceManager;
    private final net.minecraft.server.ReloadableServerResources managers;
    private net.minecraft.server.MinecraftServer$ReloadableResources(net.minecraft.server.packs.resources.CloseableResourceManager, net.minecraft.server.ReloadableServerResources);
    public void close();
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.server.packs.resources.CloseableResourceManager resourceManager();
    public net.minecraft.server.ReloadableServerResources managers();
}
```
