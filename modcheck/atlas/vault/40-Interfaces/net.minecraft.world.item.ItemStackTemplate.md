---
type: "interface"
fqcn: "net.minecraft.world.item.ItemStackTemplate"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.ItemStackTemplate

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/core/Holder;ILnet/minecraft/core/component/D` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `"<init>"(Lnet/minecraft/core/Holder;ILnet/minecraft/core/component/D` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |

## Declared members (30, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.item.ItemStackTemplate extends java.lang.Record implements net.minecraft.world.item.ItemInstance {
    private final net.minecraft.core.Holder<net.minecraft.world.item.Item> item;
    private final int count;
    private final net.minecraft.core.component.DataComponentPatch components;
    private static final org.slf4j.Logger LOGGER;
    public static final com.mojang.serialization.MapCodec<net.minecraft.world.item.ItemStackTemplate> MAP_CODEC;
    public static final com.mojang.serialization.Codec<net.minecraft.world.item.ItemStackTemplate> CODEC;
    public static final net.minecraft.network.codec.StreamCodec<net.minecraft.network.RegistryFriendlyByteBuf, net.minecraft.world.item.ItemStackTemplate> STREAM_CODEC;
    public net.minecraft.world.item.ItemStackTemplate(net.minecraft.world.item.Item);
    public net.minecraft.world.item.ItemStackTemplate(net.minecraft.world.item.Item, int);
    public net.minecraft.world.item.ItemStackTemplate(net.minecraft.world.item.Item, net.minecraft.core.component.DataComponentPatch);
    public net.minecraft.world.item.ItemStackTemplate(net.minecraft.core.Holder<net.minecraft.world.item.Item>, int, net.minecraft.core.component.DataComponentPatch);
    public static net.minecraft.world.item.ItemStackTemplate fromNonEmptyStack(net.minecraft.world.item.ItemStack);
    public static net.minecraft.world.item.ItemStackTemplate fromStack(net.minecraft.world.item.ItemStack);
    public static net.minecraft.world.item.ItemStackTemplate fromNonEmptyStack(net.minecraft.world.item.ItemStack, int);
    public net.minecraft.world.item.ItemStackTemplate withCount(int);
    public net.minecraft.world.item.ItemStack create();
    private net.minecraft.world.item.ItemStack validate(net.minecraft.world.item.ItemStack);
    public net.minecraft.world.item.ItemStack apply(net.minecraft.core.component.DataComponentPatch);
    public net.minecraft.world.item.ItemStack apply(int, net.minecraft.core.component.DataComponentPatch);
    public net.minecraft.core.Holder<net.minecraft.world.item.Item> typeHolder();
    public <T> T get(net.minecraft.core.component.DataComponentType<? extends T>);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.core.Holder<net.minecraft.world.item.Item> item();
    public int count();
    public net.minecraft.core.component.DataComponentPatch components();
    private static net.minecraft.world.item.ItemStackTemplate lambda$static$1(net.minecraft.core.Holder);
    private static com.mojang.datafixers.kinds.App lambda$static$0(com.mojang.serialization.codecs.RecordCodecBuilder$Instance);
    static {};
}
```
