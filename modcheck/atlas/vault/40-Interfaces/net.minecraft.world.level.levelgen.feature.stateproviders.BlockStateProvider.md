---
type: "interface"
fqcn: "net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `of(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/worl` | `` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |

## Declared members (16, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider {
    public static final com.mojang.serialization.Codec<net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider> TYPED_CODEC;
    public static final com.mojang.serialization.Codec<com.mojang.datafixers.util.Either<net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider>> STATE_OR_PROVIDER_CODEC;
    public static final com.mojang.serialization.Codec<net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider> DIRECT_CODEC;
    public static final com.mojang.serialization.Codec<net.minecraft.core.Holder<net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider>> CODEC;
    public static net.minecraft.world.level.levelgen.feature.stateproviders.SimpleStateProvider of(net.minecraft.world.level.block.state.BlockState);
    public static net.minecraft.world.level.levelgen.feature.stateproviders.SimpleStateProvider of(net.minecraft.world.level.block.Block);
    public static net.minecraft.core.Holder<net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider> holderOf(net.minecraft.world.level.block.state.BlockState);
    public static net.minecraft.core.Holder<net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider> holderOf(net.minecraft.world.level.block.Block);
    public abstract com.mojang.serialization.MapCodec<? extends net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider> codec();
    public abstract net.minecraft.world.level.block.state.BlockState getState(net.minecraft.world.level.LevelAccessor, net.minecraft.util.RandomSource, net.minecraft.core.BlockPos);
    public default net.minecraft.world.level.block.state.BlockState getOptionalState(net.minecraft.world.level.LevelAccessor, net.minecraft.util.RandomSource, net.minecraft.core.BlockPos);
    private static com.mojang.datafixers.util.Either lambda$static$3(net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider);
    private static net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider lambda$static$1(com.mojang.datafixers.util.Either);
    private static net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider lambda$static$2(net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider);
    private static com.mojang.serialization.MapCodec lambda$static$0(com.mojang.serialization.MapCodec);
    static {};
}
```
