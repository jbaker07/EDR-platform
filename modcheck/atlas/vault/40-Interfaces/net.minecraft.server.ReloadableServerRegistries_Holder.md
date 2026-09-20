---
type: "interface"
fqcn: "net.minecraft.server.ReloadableServerRegistries$Holder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.ReloadableServerRegistries$Holder

System: [[20-Systems/net.minecraft.server|net.minecraft.server]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `lookup()Lnet/minecraft/core/HolderLookup$Provider;` | `` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |

## Declared members (5, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.ReloadableServerRegistries$Holder {
    private final net.minecraft.core.HolderLookup$Provider registries;
    public net.minecraft.server.ReloadableServerRegistries$Holder(net.minecraft.core.HolderLookup$Provider);
    public net.minecraft.core.HolderLookup$Provider lookup();
    public net.minecraft.world.level.storage.loot.LootTable getLootTable(net.minecraft.resources.ResourceKey<net.minecraft.world.level.storage.loot.LootTable>);
    private static java.util.Optional lambda$getLootTable$0(net.minecraft.resources.ResourceKey, net.minecraft.core.HolderLookup$RegistryLookup);
}
```
