---
type: "interface"
fqcn: "net.minecraft.advancements.AdvancementRequirements"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.advancements.AdvancementRequirements

System: [[20-Systems/net.minecraft.advancements|net.minecraft.advancements]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/util/List;)V` | exact | invokespecial@121 in `AdvancementBuilderMixin.pureRequirements` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `<init>` | `(Ljava/util/List;)V` | exact | invokespecial@45 in `AdvancementBuilderMixin.requireCriteria` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `requirements` | `()Ljava/util/List;` | exact | invokevirtual@34 in `AdvancementBuilderMixin.pureRequirements` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `requirements` | `()Ljava/util/List;` | exact | invokevirtual@2 in `AdvancementBuilderMixin.lambda$requireCriteria$0` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| reads | `EMPTY` | `Lnet/minecraft/advancements/AdvancementRequirements;` | exact | getstatic@4 in `AdvancementBuilderMixin.getRequirements` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |

## Declared members (4 fields, 17 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final requirements : Ljava/util/List;
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public static final EMPTY : Lnet/minecraft/advancements/AdvancementRequirements;
public <init>(Ljava/util/List;)V
public static allOf(Ljava/util/Collection;)Lnet/minecraft/advancements/AdvancementRequirements;
public static anyOf(Ljava/util/Collection;)Lnet/minecraft/advancements/AdvancementRequirements;
public size()I
public test(Ljava/util/function/Predicate;)Z
public count(Ljava/util/function/Predicate;)I
private static anyMatch(Ljava/util/List;Ljava/util/function/Predicate;)Z
public validate(Ljava/util/Set;)Lcom/mojang/serialization/DataResult;
public isEmpty()Z
public toString()Ljava/lang/String;
public names()Ljava/util/Set;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public requirements()Ljava/util/List;
private static synthetic lambda$validate$1(Ljava/util/Set;Ljava/util/Set;)Ljava/lang/String;
private static synthetic lambda$validate$0()Ljava/lang/String;
static <clinit>()V
```
