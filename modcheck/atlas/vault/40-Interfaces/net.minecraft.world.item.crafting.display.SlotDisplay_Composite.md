---
type: "interface"
fqcn: "net.minecraft.world.item.crafting.display.SlotDisplay$Composite"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.crafting.display.SlotDisplay$Composite

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Ljava/util/List;)V` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `"<init>"(Ljava/util/List;)V` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `"<init>"(Ljava/util/List;)V` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |

## Declared members (16, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.item.crafting.display.SlotDisplay$Composite extends java.lang.Record implements net.minecraft.world.item.crafting.display.SlotDisplay {
    private final java.util.List<net.minecraft.world.item.crafting.display.SlotDisplay> contents;
    public static final com.mojang.serialization.MapCodec<net.minecraft.world.item.crafting.display.SlotDisplay$Composite> MAP_CODEC;
    public static final net.minecraft.network.codec.StreamCodec<net.minecraft.network.RegistryFriendlyByteBuf, net.minecraft.world.item.crafting.display.SlotDisplay$Composite> STREAM_CODEC;
    public static final net.minecraft.world.item.crafting.display.SlotDisplay$Type<net.minecraft.world.item.crafting.display.SlotDisplay$Composite> TYPE;
    public net.minecraft.world.item.crafting.display.SlotDisplay$Composite(java.util.List<net.minecraft.world.item.crafting.display.SlotDisplay>);
    public net.minecraft.world.item.crafting.display.SlotDisplay$Type<net.minecraft.world.item.crafting.display.SlotDisplay$Composite> type();
    public <T> java.util.stream.Stream<T> resolve(net.minecraft.util.context.ContextMap, net.minecraft.world.item.crafting.display.DisplayContentsFactory<T>);
    public boolean isEnabled(net.minecraft.world.flag.FeatureFlagSet);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public java.util.List<net.minecraft.world.item.crafting.display.SlotDisplay> contents();
    private static boolean lambda$isEnabled$0(net.minecraft.world.flag.FeatureFlagSet, net.minecraft.world.item.crafting.display.SlotDisplay);
    private static java.util.stream.Stream lambda$resolve$0(net.minecraft.util.context.ContextMap, net.minecraft.world.item.crafting.display.DisplayContentsFactory, net.minecraft.world.item.crafting.display.SlotDisplay);
    private static com.mojang.datafixers.kinds.App lambda$static$0(com.mojang.serialization.codecs.RecordCodecBuilder$Instance);
    static {};
}
```
