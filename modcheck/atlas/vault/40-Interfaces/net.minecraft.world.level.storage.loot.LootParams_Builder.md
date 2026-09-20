---
type: "interface"
fqcn: "net.minecraft.world.level.storage.loot.LootParams$Builder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.loot.LootParams$Builder

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/server/level/ServerLevel;)V` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `create(Lnet/minecraft/util/context/ContextKeySet;)Lnet/minecraft/w` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `withParameter(Lnet/minecraft/util/context/ContextKey;Ljava/lang/Object;)L` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (13, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.storage.loot.LootParams$Builder {
    private final net.minecraft.server.level.ServerLevel level;
    private final net.minecraft.util.context.ContextMap$Builder params;
    private final java.util.Map<net.minecraft.resources.Identifier, net.minecraft.world.level.storage.loot.LootParams$DynamicDrop> dynamicDrops;
    private float luck;
    public net.minecraft.world.level.storage.loot.LootParams$Builder(net.minecraft.server.level.ServerLevel);
    public net.minecraft.server.level.ServerLevel getLevel();
    public <T> net.minecraft.world.level.storage.loot.LootParams$Builder withParameter(net.minecraft.util.context.ContextKey<T>, T);
    public <T> net.minecraft.world.level.storage.loot.LootParams$Builder withOptionalParameter(net.minecraft.util.context.ContextKey<T>, T);
    public <T> T getParameter(net.minecraft.util.context.ContextKey<T>);
    public <T> T getOptionalParameter(net.minecraft.util.context.ContextKey<T>);
    public net.minecraft.world.level.storage.loot.LootParams$Builder withDynamicDrop(net.minecraft.resources.Identifier, net.minecraft.world.level.storage.loot.LootParams$DynamicDrop);
    public net.minecraft.world.level.storage.loot.LootParams$Builder withLuck(float);
    public net.minecraft.world.level.storage.loot.LootParams create(net.minecraft.util.context.ContextKeySet);
}
```
