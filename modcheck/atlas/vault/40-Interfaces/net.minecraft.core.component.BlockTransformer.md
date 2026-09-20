---
type: "interface"
fqcn: "net.minecraft.core.component.BlockTransformer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.component.BlockTransformer

System: [[20-Systems/net.minecraft.core.component|net.minecraft.core.component]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Ljava/util/List;)V` | `` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `transforms()Ljava/util/List;` | `` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| injects_into | `<clinit>` | `@ModifyArg at INVOKE Lcom/mojang/serialization/Codec;listOf(II)Lcom/mojang/seria` | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (14, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.core.component.BlockTransformer extends java.lang.Record {
    private final java.util.List<net.minecraft.core.component.BlockTransformer$BlockTransformData> transforms;
    public static final com.mojang.serialization.Codec<net.minecraft.core.component.BlockTransformer> DIRECT_CODEC;
    public static final com.mojang.serialization.Codec<net.minecraft.core.Holder<net.minecraft.core.component.BlockTransformer>> CODEC;
    public static final net.minecraft.network.codec.StreamCodec<net.minecraft.network.RegistryFriendlyByteBuf, net.minecraft.core.Holder<net.minecraft.core.component.BlockTransformer>> STREAM_CODEC;
    public net.minecraft.core.component.BlockTransformer(java.util.List<net.minecraft.core.component.BlockTransformer$BlockTransformData>);
    public net.minecraft.world.InteractionResult transformBlock(net.minecraft.world.item.context.UseOnContext);
    private static boolean playerHasBlockingItemUseIntent(net.minecraft.world.item.context.UseOnContext);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public java.util.List<net.minecraft.core.component.BlockTransformer$BlockTransformData> transforms();
    private static void lambda$transformBlock$0(net.minecraft.server.level.ServerLevel, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.Level, net.minecraft.world.item.ItemStack, net.minecraft.world.entity.player.Player, net.minecraft.core.component.BlockTransformer$BlockTransformData, net.minecraft.core.Direction, net.minecraft.resources.ResourceKey);
    private static void lambda$transformBlock$1(net.minecraft.core.component.BlockTransformer$BlockTransformData, net.minecraft.core.BlockPos, net.minecraft.core.Direction, net.minecraft.server.level.ServerLevel, net.minecraft.world.item.ItemStack);
    static {};
}
```
