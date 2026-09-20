---
type: "interface"
fqcn: "net.minecraft.world.level.storage.loot.LootContext$Builder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.loot.LootContext$Builder

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/world/level/storage/loot/LootParams;)V` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `create(Ljava/util/Optional;)Lnet/minecraft/world/level/storage/loo` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (8, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.storage.loot.LootContext$Builder {
    private final net.minecraft.world.level.storage.loot.LootParams params;
    private net.minecraft.util.RandomSource random;
    public net.minecraft.world.level.storage.loot.LootContext$Builder(net.minecraft.world.level.storage.loot.LootParams);
    public net.minecraft.world.level.storage.loot.LootContext$Builder withOptionalRandomSeed(long);
    public net.minecraft.world.level.storage.loot.LootContext$Builder withOptionalRandomSource(net.minecraft.util.RandomSource);
    public net.minecraft.server.level.ServerLevel getLevel();
    public net.minecraft.world.level.storage.loot.LootContext create(java.util.Optional<net.minecraft.resources.Identifier>);
    private static java.util.Optional lambda$create$0(java.util.Optional, net.minecraft.server.MinecraftServer);
}
```
