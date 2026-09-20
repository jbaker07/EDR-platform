---
type: "interface"
fqcn: "net.minecraft.world.item.component.CustomData"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.component.CustomData

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `matchedBy(Lnet/minecraft/nbt/CompoundTag;)Z` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `of(Lnet/minecraft/nbt/CompoundTag;)Lnet/minecraft/world/item/c` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |

## Declared members (19, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.item.component.CustomData {
    public static final net.minecraft.world.item.component.CustomData EMPTY;
    public static final com.mojang.serialization.Codec<net.minecraft.nbt.CompoundTag> COMPOUND_TAG_CODEC;
    public static final com.mojang.serialization.Codec<net.minecraft.world.item.component.CustomData> CODEC;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, net.minecraft.world.item.component.CustomData> STREAM_CODEC;
    private final net.minecraft.nbt.CompoundTag tag;
    private net.minecraft.world.item.component.CustomData(net.minecraft.nbt.CompoundTag);
    public static net.minecraft.world.item.component.CustomData of(net.minecraft.nbt.CompoundTag);
    public boolean matchedBy(net.minecraft.nbt.CompoundTag);
    public static void update(net.minecraft.core.component.DataComponentType<net.minecraft.world.item.component.CustomData>, net.minecraft.world.item.ItemStack, java.util.function.Consumer<net.minecraft.nbt.CompoundTag>);
    public static void set(net.minecraft.core.component.DataComponentType<net.minecraft.world.item.component.CustomData>, net.minecraft.world.item.ItemStack, net.minecraft.nbt.CompoundTag);
    public net.minecraft.world.item.component.CustomData update(java.util.function.Consumer<net.minecraft.nbt.CompoundTag>);
    public boolean isEmpty();
    public net.minecraft.nbt.CompoundTag copyTag();
    public boolean equals(java.lang.Object);
    public int hashCode();
    public java.lang.String toString();
    private static net.minecraft.nbt.CompoundTag lambda$static$1(net.minecraft.world.item.component.CustomData);
    private static net.minecraft.nbt.CompoundTag lambda$static$0(net.minecraft.world.item.component.CustomData);
    static {};
}
```
