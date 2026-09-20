---
type: "interface"
fqcn: "net.minecraft.world.level.storage.loot.providers.number.ints.ResolvableInt"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.loot.providers.number.ints.ResolvableInt

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `get(Lnet/minecraft/world/level/storage/loot/LootContext;I)I` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (7, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.world.level.storage.loot.providers.number.ints.ResolvableInt {
    public static final com.mojang.serialization.Codec<net.minecraft.world.level.storage.loot.providers.number.ints.ResolvableInt> CODEC;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, net.minecraft.world.level.storage.loot.providers.number.ints.ResolvableInt> STREAM_CODEC;
    private static com.mojang.datafixers.util.Either<net.minecraft.world.level.storage.loot.providers.number.ints.ResolvableInt$Constant, net.minecraft.world.level.storage.loot.providers.number.ints.ResolvableInt$Reference> wrap(net.minecraft.world.level.storage.loot.providers.number.ints.ResolvableInt);
    public abstract int get(net.minecraft.world.level.storage.loot.LootContext, int);
    public static net.minecraft.world.level.storage.loot.providers.number.ints.ResolvableInt fromKey(net.minecraft.resources.ResourceKey<net.minecraft.world.level.storage.loot.providers.number.ints.ContextIntProvider>);
    public static <T> int getFromItem(net.minecraft.world.item.ItemStack, net.minecraft.core.component.DataComponentType<T>, java.util.function.Function<T, net.minecraft.world.level.storage.loot.providers.number.ints.ResolvableInt>, net.minecraft.world.level.storage.loot.LootContext, int);
    static {};
}
```
