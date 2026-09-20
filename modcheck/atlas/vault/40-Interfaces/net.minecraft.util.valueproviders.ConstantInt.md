---
type: "interface"
fqcn: "net.minecraft.util.valueproviders.ConstantInt"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.valueproviders.ConstantInt

System: [[20-Systems/net.minecraft.util.valueproviders|net.minecraft.util.valueproviders]]

`record` public final; extends `java/lang/Record`; implements `net/minecraft/util/valueproviders/IntProvider`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `of` | `(I)Lnet/minecraft/util/valueproviders/ConstantInt;` | exact | invokestatic@6 in `BiomeModifications.lambda$addSpawn$0` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (3 fields, 12 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final value : I
public static final ZERO : Lnet/minecraft/util/valueproviders/ConstantInt;
public static final MAP_CODEC : Lcom/mojang/serialization/MapCodec;
public <init>(I)V
public static of(I)Lnet/minecraft/util/valueproviders/ConstantInt;
public sample(Lnet/minecraft/util/RandomSource;)I
public minInclusive()I
public maxInclusive()I
public codec()Lcom/mojang/serialization/MapCodec;
public toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public value()I
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
static <clinit>()V
```
