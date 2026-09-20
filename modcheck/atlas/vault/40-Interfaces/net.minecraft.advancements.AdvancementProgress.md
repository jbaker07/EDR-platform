---
type: "interface"
fqcn: "net.minecraft.advancements.AdvancementProgress"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.advancements.AdvancementProgress

System: [[20-Systems/net.minecraft.advancements|net.minecraft.advancements]]

`class` public; extends `java/lang/Object`; implements `java/lang/Comparable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getPercent` | `()F` | exact | invokevirtual@12 in `AdvancementRenderContext.isObtained` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (7 fields, 29 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final OBTAINED_TIME_FORMAT : Ljava/time/format/DateTimeFormatter;
private static final OBTAINED_TIME_CODEC : Lcom/mojang/serialization/Codec;
private static final CRITERIA_CODEC : Lcom/mojang/serialization/Codec;
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
private final criteria : Ljava/util/Map;
private requirements : Lnet/minecraft/advancements/AdvancementRequirements;
private <init>(Ljava/util/Map;)V
public <init>()V
public update(Lnet/minecraft/advancements/AdvancementRequirements;)V
public isDone()Z
public hasProgress()Z
public grantProgress(Ljava/lang/String;)Z
public revokeProgress(Ljava/lang/String;)Z
public toString()Ljava/lang/String;
public getCriterion(Ljava/lang/String;)Lnet/minecraft/advancements/CriterionProgress;
private isCriterionDone(Ljava/lang/String;)Z
public getPercent()F
public getProgressText()Lnet/minecraft/network/chat/Component;
private countCompletedRequirements()I
public getRemainingCriteria()Ljava/lang/Iterable;
public getCompletedCriteria()Ljava/lang/Iterable;
public getFirstProgressDate()Ljava/time/Instant;
public compareTo(Lnet/minecraft/advancements/AdvancementProgress;)I
public synthetic compareTo(Ljava/lang/Object;)I
private static synthetic lambda$update$0(Ljava/util/Set;Ljava/util/Map$Entry;)Z
private static synthetic lambda$static$8(Lnet/minecraft/advancements/AdvancementProgress;)Ljava/util/Map;
private static synthetic lambda$static$5(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
private static synthetic lambda$static$7(Ljava/util/Map;Ljava/lang/Boolean;)Lnet/minecraft/advancements/AdvancementProgress;
private static synthetic lambda$static$6(Lnet/minecraft/advancements/AdvancementProgress;)Ljava/util/Map;
private static synthetic lambda$static$2(Ljava/util/Map;)Ljava/util/Map;
private static synthetic lambda$static$4(Ljava/util/Map$Entry;)Ljava/time/Instant;
private static synthetic lambda$static$3(Ljava/util/Map$Entry;)Z
private static synthetic lambda$static$1(Ljava/util/Map;)Ljava/util/Map;
private static synthetic lambda$static$0(Ljava/time/Instant;)Ljava/time/temporal/TemporalAccessor;
static <clinit>()V
```
