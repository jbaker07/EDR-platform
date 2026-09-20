---
type: "interface"
fqcn: "net.minecraft.world.level.levelgen.feature.stateproviders.CopyPropertiesProvider"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.levelgen.feature.stateproviders.CopyPropertiesProvider

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`record` public final; extends `java/lang/Record`; implements `net/minecraft/world/level/levelgen/feature/stateproviders/BlockStateProvider`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/core/Holder;)V` | exact | invokespecial@12 in `BlockTransformerHelperImpl.createStripping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/core/Holder;)V` | exact | invokespecial@12 in `BlockTransformerHelperImpl.createOxidationScraping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/core/Holder;)V` | exact | invokespecial@12 in `BlockTransformerHelperImpl.createWaxScraping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (2 fields, 10 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final source : Lnet/minecraft/core/Holder;
public static final CODEC : Lcom/mojang/serialization/MapCodec;
public <init>(Lnet/minecraft/world/level/block/Block;)V
public <init>(Lnet/minecraft/core/Holder;)V
public codec()Lcom/mojang/serialization/MapCodec;
public getState(Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/util/RandomSource;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/BlockState;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public source()Lnet/minecraft/core/Holder;
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
static <clinit>()V
```
