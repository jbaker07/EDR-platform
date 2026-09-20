---
type: "interface"
fqcn: "net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider$Builder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider$Builder

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `build` | `()Lnet/minecraft/world/level/levelgen/feature/stateproviders/RuleBased` | exact | invokevirtual@18 in `BlockTransformerHelperImpl.createStripping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `build` | `()Lnet/minecraft/world/level/levelgen/feature/stateproviders/RuleBased` | exact | invokevirtual@20 in `BlockTransformerHelperImpl.createTilling` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `build` | `()Lnet/minecraft/world/level/levelgen/feature/stateproviders/RuleBased` | exact | invokevirtual@20 in `BlockTransformerHelperImpl.createFlattening` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `build` | `()Lnet/minecraft/world/level/levelgen/feature/stateproviders/RuleBased` | exact | invokevirtual@18 in `BlockTransformerHelperImpl.createOxidationScraping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `build` | `()Lnet/minecraft/world/level/levelgen/feature/stateproviders/RuleBased` | exact | invokevirtual@18 in `BlockTransformerHelperImpl.createWaxScraping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `ifTrueThenProvide` | `(Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;Ln` | exact | invokevirtual@15 in `BlockTransformerHelperImpl.createStripping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `ifTrueThenProvide` | `(Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;Ln` | exact | invokevirtual@17 in `BlockTransformerHelperImpl.createTilling` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `ifTrueThenProvide` | `(Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;Ln` | exact | invokevirtual@17 in `BlockTransformerHelperImpl.createFlattening` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `ifTrueThenProvide` | `(Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;Ln` | exact | invokevirtual@15 in `BlockTransformerHelperImpl.createOxidationScraping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `ifTrueThenProvide` | `(Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;Ln` | exact | invokevirtual@15 in `BlockTransformerHelperImpl.createWaxScraping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (2 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final fallback : Lnet/minecraft/world/level/levelgen/feature/stateproviders/BlockStateProvider;
private final rules : Ljava/util/List;
public <init>(Lnet/minecraft/world/level/levelgen/feature/stateproviders/BlockStateProvider;)V
public ifTrueThenProvide(Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;Lnet/minecraft/world/level/levelgen/feature/stateproviders/BlockStateProvider;)Lnet/minecraft/world/level/levelgen/feature/stateproviders/RuleBasedStateProvider$Builder;
public ifTrueThenProvide(Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/levelgen/feature/stateproviders/RuleBasedStateProvider$Builder;
public ifTrueThenProvide(Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/level/levelgen/feature/stateproviders/RuleBasedStateProvider$Builder;
public build()Lnet/minecraft/world/level/levelgen/feature/stateproviders/RuleBasedStateProvider;
```
