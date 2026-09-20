---
type: "interface"
fqcn: "net.minecraft.world.level.levelgen.feature.stateproviders.CopyPropertiesProvider"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.levelgen.feature.stateproviders.CopyPropertiesProvider

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/core/Holder;)V` | `` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (12, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.level.levelgen.feature.stateproviders.CopyPropertiesProvider extends java.lang.Record implements net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider {
    private final net.minecraft.core.Holder<net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider> source;
    public static final com.mojang.serialization.MapCodec<net.minecraft.world.level.levelgen.feature.stateproviders.CopyPropertiesProvider> CODEC;
    public net.minecraft.world.level.levelgen.feature.stateproviders.CopyPropertiesProvider(net.minecraft.world.level.block.Block);
    public net.minecraft.world.level.levelgen.feature.stateproviders.CopyPropertiesProvider(net.minecraft.core.Holder<net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider>);
    public com.mojang.serialization.MapCodec<net.minecraft.world.level.levelgen.feature.stateproviders.CopyPropertiesProvider> codec();
    public net.minecraft.world.level.block.state.BlockState getState(net.minecraft.world.level.LevelAccessor, net.minecraft.util.RandomSource, net.minecraft.core.BlockPos);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.core.Holder<net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider> source();
    private static com.mojang.datafixers.kinds.App lambda$static$0(com.mojang.serialization.codecs.RecordCodecBuilder$Instance);
    static {};
}
```
