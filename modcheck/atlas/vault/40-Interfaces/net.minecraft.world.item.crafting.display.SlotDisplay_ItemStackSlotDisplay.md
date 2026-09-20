---
type: "interface"
fqcn: "net.minecraft.world.item.crafting.display.SlotDisplay$ItemStackSlotDisplay"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.crafting.display.SlotDisplay$ItemStackSlotDisplay

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/world/item/ItemStackTemplate;)V` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `"<init>"(Lnet/minecraft/world/item/ItemStackTemplate;)V` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |

## Declared members (14, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.item.crafting.display.SlotDisplay$ItemStackSlotDisplay extends java.lang.Record implements net.minecraft.world.item.crafting.display.SlotDisplay {
    private final net.minecraft.world.item.ItemStackTemplate stack;
    public static final com.mojang.serialization.MapCodec<net.minecraft.world.item.crafting.display.SlotDisplay$ItemStackSlotDisplay> MAP_CODEC;
    public static final net.minecraft.network.codec.StreamCodec<net.minecraft.network.RegistryFriendlyByteBuf, net.minecraft.world.item.crafting.display.SlotDisplay$ItemStackSlotDisplay> STREAM_CODEC;
    public static final net.minecraft.world.item.crafting.display.SlotDisplay$Type<net.minecraft.world.item.crafting.display.SlotDisplay$ItemStackSlotDisplay> TYPE;
    public net.minecraft.world.item.crafting.display.SlotDisplay$ItemStackSlotDisplay(net.minecraft.world.item.ItemStackTemplate);
    public net.minecraft.world.item.crafting.display.SlotDisplay$Type<net.minecraft.world.item.crafting.display.SlotDisplay$ItemStackSlotDisplay> type();
    public <T> java.util.stream.Stream<T> resolve(net.minecraft.util.context.ContextMap, net.minecraft.world.item.crafting.display.DisplayContentsFactory<T>);
    public boolean isEnabled(net.minecraft.world.flag.FeatureFlagSet);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.world.item.ItemStackTemplate stack();
    private static com.mojang.datafixers.kinds.App lambda$static$0(com.mojang.serialization.codecs.RecordCodecBuilder$Instance);
    static {};
}
```
