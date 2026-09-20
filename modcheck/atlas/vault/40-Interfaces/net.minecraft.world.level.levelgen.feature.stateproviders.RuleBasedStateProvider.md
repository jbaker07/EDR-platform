---
type: "interface"
fqcn: "net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `builder()Lnet/minecraft/world/level/levelgen/feature/stateproviders` | `` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (21, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider extends java.lang.Record implements net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider {
    private final net.minecraft.core.Holder<net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider> fallback;
    private final java.util.List<net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider$Rule> rules;
    public static final com.mojang.serialization.MapCodec<net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider> CODEC;
    private net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider(java.util.Optional<net.minecraft.core.Holder<net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider>>, java.util.List<net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider$Rule>);
    public net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider(net.minecraft.core.Holder<net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider>, java.util.List<net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider$Rule>);
    public static net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider ifTrueThenProvide(net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate, net.minecraft.world.level.block.Block);
    public static net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider ifTrueThenProvide(net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate, net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider);
    public com.mojang.serialization.MapCodec<net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider> codec();
    public net.minecraft.world.level.block.state.BlockState getState(net.minecraft.world.level.LevelAccessor, net.minecraft.util.RandomSource, net.minecraft.core.BlockPos);
    public net.minecraft.world.level.block.state.BlockState getOptionalState(net.minecraft.world.level.LevelAccessor, net.minecraft.util.RandomSource, net.minecraft.core.BlockPos);
    public static net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider$Builder builder();
    public static net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider$Builder builder(net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.core.Holder<net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider> fallback();
    public java.util.List<net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider$Rule> rules();
    private static com.mojang.datafixers.kinds.App lambda$static$0(com.mojang.serialization.codecs.RecordCodecBuilder$Instance);
    private static java.util.List lambda$static$2(net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider);
    private static java.util.Optional lambda$static$1(net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider);
    static {};
}
```
