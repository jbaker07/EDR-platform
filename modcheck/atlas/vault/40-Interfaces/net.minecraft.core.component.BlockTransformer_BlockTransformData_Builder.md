---
type: "interface"
fqcn: "net.minecraft.core.component.BlockTransformer$BlockTransformData$Builder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.component.BlockTransformer$BlockTransformData$Builder

System: [[20-Systems/net.minecraft.core.component|net.minecraft.core.component]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `build()Lnet/minecraft/core/component/BlockTransformer$BlockTransf` | `` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `disallowedFaces(Ljava/util/List;)Lnet/minecraft/core/component/BlockTransfo` | `` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `particle(Lnet/minecraft/core/component/BlockTransformer$TransformPar` | `` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `sound(Lnet/minecraft/core/Holder;)Lnet/minecraft/core/component/B` | `` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (21, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.core.component.BlockTransformer$BlockTransformData$Builder {
    private final net.minecraft.core.Holder<net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider> targetStateProvider;
    private net.minecraft.core.Holder<net.minecraft.sounds.SoundEvent> sound;
    private net.minecraft.core.component.BlockTransformer$TransformParticle particle;
    private java.util.List<net.minecraft.core.Direction> disallowedFaces;
    private java.util.Optional<net.minecraft.resources.ResourceKey<net.minecraft.world.level.storage.loot.LootTable>> loot;
    private net.minecraft.core.component.BlockTransformer$DropStrategy dropStrategy;
    private boolean updateFromNeighbors;
    private net.minecraft.core.component.BlockTransformer$TransformType transformType;
    private boolean consumeOnUse;
    private int itemDamagePerUse;
    private net.minecraft.core.component.BlockTransformer$BlockTransformData$Builder(net.minecraft.core.Holder<net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider>);
    public net.minecraft.core.component.BlockTransformer$BlockTransformData$Builder sound(net.minecraft.core.Holder<net.minecraft.sounds.SoundEvent>);
    public net.minecraft.core.component.BlockTransformer$BlockTransformData$Builder particle(net.minecraft.core.component.BlockTransformer$TransformParticle);
    public net.minecraft.core.component.BlockTransformer$BlockTransformData$Builder disallowedFaces(java.util.List<net.minecraft.core.Direction>);
    public net.minecraft.core.component.BlockTransformer$BlockTransformData$Builder loot(net.minecraft.resources.ResourceKey<net.minecraft.world.level.storage.loot.LootTable>);
    public net.minecraft.core.component.BlockTransformer$BlockTransformData$Builder dropStrategy(net.minecraft.core.component.BlockTransformer$DropStrategy);
    public net.minecraft.core.component.BlockTransformer$BlockTransformData$Builder updateFromNeighbors(boolean);
    public net.minecraft.core.component.BlockTransformer$BlockTransformData$Builder transformType(net.minecraft.core.component.BlockTransformer$TransformType);
    public net.minecraft.core.component.BlockTransformer$BlockTransformData$Builder consumeOnUse(boolean);
    public net.minecraft.core.component.BlockTransformer$BlockTransformData$Builder itemDamagePerUse(int);
    public net.minecraft.core.component.BlockTransformer$BlockTransformData build();
}
```
