---
type: "interface"
fqcn: "net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider$Builder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider$Builder

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `build()Lnet/minecraft/world/level/levelgen/feature/stateproviders` | `` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `ifTrueThenProvide(Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPr` | `` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (7, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider$Builder {
    private final net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider fallback;
    private final java.util.List<net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider$Rule> rules;
    public net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider$Builder(net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider);
    public net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider$Builder ifTrueThenProvide(net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate, net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider);
    public net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider$Builder ifTrueThenProvide(net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate, net.minecraft.world.level.block.Block);
    public net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider$Builder ifTrueThenProvide(net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate, net.minecraft.world.level.block.state.BlockState);
    public net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider build();
}
```
