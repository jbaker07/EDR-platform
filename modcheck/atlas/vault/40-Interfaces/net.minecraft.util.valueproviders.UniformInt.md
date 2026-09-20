---
type: "interface"
fqcn: "net.minecraft.util.valueproviders.UniformInt"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.valueproviders.UniformInt

System: [[20-Systems/net.minecraft.util.valueproviders|net.minecraft.util.valueproviders]]

`record` public final; extends `java/lang/Record`; implements `net/minecraft/util/valueproviders/IntProvider`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `of` | `(II)Lnet/minecraft/util/valueproviders/UniformInt;` | exact | invokestatic@14 in `BiomeModifications.lambda$addSpawn$0` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (3 fields, 13 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final minInclusive : I
private final maxInclusive : I
public static final MAP_CODEC : Lcom/mojang/serialization/MapCodec;
public <init>(II)V
public static of(II)Lnet/minecraft/util/valueproviders/UniformInt;
public sample(Lnet/minecraft/util/RandomSource;)I
public codec()Lcom/mojang/serialization/MapCodec;
public toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public minInclusive()I
public maxInclusive()I
private static synthetic lambda$static$1(Lnet/minecraft/util/valueproviders/UniformInt;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$2(Lnet/minecraft/util/valueproviders/UniformInt;)Ljava/lang/String;
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
static <clinit>()V
```
