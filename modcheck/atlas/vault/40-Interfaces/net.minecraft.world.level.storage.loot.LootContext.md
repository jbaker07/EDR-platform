---
type: "interface"
fqcn: "net.minecraft.world.level.storage.loot.LootContext"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.loot.LootContext

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getLevel()Lnet/minecraft/server/level/ServerLevel;` | `` | both | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |

## Declared members (19, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.storage.loot.LootContext {
    private final net.minecraft.world.level.storage.loot.LootParams params;
    private final net.minecraft.util.RandomSource random;
    private final net.minecraft.core.HolderGetter$Provider lootDataResolver;
    private final java.util.Set<net.minecraft.world.level.storage.loot.LootContext$VisitedEntry<?>> visitedElements;
    private net.minecraft.world.level.storage.loot.LootContext(net.minecraft.world.level.storage.loot.LootParams, net.minecraft.util.RandomSource, net.minecraft.core.HolderGetter$Provider);
    public boolean hasParameter(net.minecraft.util.context.ContextKey<?>);
    public <T> T getOptional(net.minecraft.util.context.ContextKey<T>);
    public void addDynamicDrops(net.minecraft.resources.Identifier, java.util.function.Consumer<net.minecraft.world.item.ItemStack>);
    public boolean hasVisitedElement(net.minecraft.world.level.storage.loot.LootContext$VisitedEntry<?>);
    public boolean pushVisitedElement(net.minecraft.world.level.storage.loot.LootContext$VisitedEntry<?>);
    public void popVisitedElement(net.minecraft.world.level.storage.loot.LootContext$VisitedEntry<?>);
    public net.minecraft.core.HolderGetter$Provider getResolver();
    public net.minecraft.util.RandomSource getRandom();
    public float getLuck();
    public net.minecraft.server.level.ServerLevel getLevel();
    public static net.minecraft.world.level.storage.loot.LootContext$VisitedEntry<net.minecraft.world.level.storage.loot.LootTable> createVisitedEntry(net.minecraft.world.level.storage.loot.LootTable);
    public static net.minecraft.world.level.storage.loot.LootContext$VisitedEntry<net.minecraft.world.level.storage.loot.predicates.LootItemCondition> createVisitedEntry(net.minecraft.world.level.storage.loot.predicates.LootItemCondition);
    public static net.minecraft.world.level.storage.loot.LootContext$VisitedEntry<net.minecraft.world.level.storage.loot.functions.LootItemFunction> createVisitedEntry(net.minecraft.world.level.storage.loot.functions.LootItemFunction);
    public static net.minecraft.world.level.storage.loot.LootContext$VisitedEntry<net.minecraft.world.item.slot.SlotSource> createVisitedEntry(net.minecraft.world.item.slot.SlotSource);
}
```
