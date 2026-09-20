---
type: "interface"
fqcn: "net.minecraft.util.InclusiveRange"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.InclusiveRange

System: [[20-Systems/net.minecraft.util|net.minecraft.util]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/lang/Comparable;)V` | exact | invokespecial@10 in `ModPackResourcesUtil.getMetadataPack` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (3 fields, 18 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final minInclusive : Ljava/lang/Comparable;
private final maxInclusive : Ljava/lang/Comparable;
public static final INT : Lcom/mojang/serialization/Codec;
public <init>(Ljava/lang/Comparable;Ljava/lang/Comparable;)V
public <init>(Ljava/lang/Comparable;)V
public static codec(Lcom/mojang/serialization/Codec;)Lcom/mojang/serialization/Codec;
public static codec(Lcom/mojang/serialization/Codec;Ljava/lang/Comparable;Ljava/lang/Comparable;)Lcom/mojang/serialization/Codec;
public static create(Ljava/lang/Comparable;Ljava/lang/Comparable;)Lcom/mojang/serialization/DataResult;
public map(Ljava/util/function/Function;)Lnet/minecraft/util/InclusiveRange;
public isValueInRange(Ljava/lang/Comparable;)Z
public contains(Lnet/minecraft/util/InclusiveRange;)Z
public toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public minInclusive()Ljava/lang/Comparable;
public maxInclusive()Ljava/lang/Comparable;
private static synthetic lambda$create$0()Ljava/lang/String;
private static synthetic lambda$codec$0(Ljava/lang/Comparable;Ljava/lang/Comparable;Lnet/minecraft/util/InclusiveRange;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$codec$2(Ljava/lang/Comparable;Lnet/minecraft/util/InclusiveRange;)Ljava/lang/String;
private static synthetic lambda$codec$1(Ljava/lang/Comparable;Lnet/minecraft/util/InclusiveRange;)Ljava/lang/String;
static <clinit>()V
```
