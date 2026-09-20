---
type: "interface"
fqcn: "net.minecraft.advancements.Advancement"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.advancements.Advancement

System: [[20-Systems/net.minecraft.advancements|net.minecraft.advancements]]

`record` public final; extends `java/lang/Record`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `criteria` | `()Ljava/util/Map;` | exact | invokevirtual@27 in `FabricAdvancementBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `display` | `()Ljava/util/Optional;` | exact | invokevirtual@65 in `FabricAdvancementBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `display` | `()Ljava/util/Optional;` | exact | invokevirtual@6 in `AdvancementRenderContext.display` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `parent` | `()Ljava/util/Optional;` | exact | invokevirtual@9 in `FabricAdvancementBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `requirements` | `()Lnet/minecraft/advancements/AdvancementRequirements;` | exact | invokevirtual@48 in `FabricAdvancementBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `rewards` | `()Lnet/minecraft/advancements/AdvancementRewards;` | exact | invokevirtual@57 in `FabricAdvancementBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `sendsTelemetryEvent` | `()Z` | exact | invokevirtual@83 in `FabricAdvancementBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| reads | `CODEC` | `Lcom/mojang/serialization/Codec;` | exact | getstatic@112 in `FabricAdvancementProvider.lambda$run$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `CODEC` | `Lcom/mojang/serialization/Codec;` | exact | getstatic@0 in `FabricRecipeProvider.lambda$run$2` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (10 fields, 31 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final parent : Ljava/util/Optional;
private final display : Ljava/util/Optional;
private final rewards : Lnet/minecraft/advancements/AdvancementRewards;
private final criteria : Ljava/util/Map;
private final requirements : Lnet/minecraft/advancements/AdvancementRequirements;
private final sendsTelemetryEvent : Z
private final name : Ljava/util/Optional;
private static final CRITERIA_CODEC : Lcom/mojang/serialization/Codec;
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public <init>(Ljava/util/Optional;Ljava/util/Optional;Lnet/minecraft/advancements/AdvancementRewards;Ljava/util/Map;Lnet/minecraft/advancements/AdvancementRequirements;Z)V
public <init>(Ljava/util/Optional;Ljava/util/Optional;Lnet/minecraft/advancements/AdvancementRewards;Ljava/util/Map;Lnet/minecraft/advancements/AdvancementRequirements;ZLjava/util/Optional;)V
private static validate(Lnet/minecraft/advancements/Advancement;)Lcom/mojang/serialization/DataResult;
private static decorateName(Lnet/minecraft/advancements/DisplayInfo;)Lnet/minecraft/network/chat/Component;
public static name(Lnet/minecraft/advancements/AdvancementHolder;)Lnet/minecraft/network/chat/Component;
public isRoot()Z
public validate(Lnet/minecraft/util/ProblemReporter;Lnet/minecraft/core/HolderGetter$Provider;)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public parent()Ljava/util/Optional;
public display()Ljava/util/Optional;
public rewards()Lnet/minecraft/advancements/AdvancementRewards;
public criteria()Ljava/util/Map;
public requirements()Lnet/minecraft/advancements/AdvancementRequirements;
public sendsTelemetryEvent()Z
public name()Ljava/util/Optional;
private static synthetic lambda$validate$3(Lnet/minecraft/util/ProblemReporter;Lnet/minecraft/core/HolderGetter$Provider;Ljava/lang/String;Lnet/minecraft/advancements/triggers/Criterion;)V
private static synthetic lambda$name$0(Lnet/minecraft/advancements/AdvancementHolder;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$decorateName$0(Lnet/minecraft/network/chat/Component;Lnet/minecraft/network/chat/Style;)Lnet/minecraft/network/chat/Style;
private static synthetic lambda$validate$2(Lnet/minecraft/advancements/Advancement;Lnet/minecraft/advancements/AdvancementRequirements;)Lnet/minecraft/advancements/Advancement;
private static synthetic lambda$validate$1()Ljava/lang/String;
private static synthetic lambda$validate$0()Ljava/lang/String;
private static synthetic lambda$static$6(Ljava/util/Optional;Ljava/util/Optional;Lnet/minecraft/advancements/AdvancementRequirements;Ljava/lang/Boolean;)Lnet/minecraft/advancements/Advancement;
private static synthetic lambda$static$2(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
private static synthetic lambda$static$4(Ljava/util/Optional;Ljava/util/Optional;Lnet/minecraft/advancements/AdvancementRewards;Ljava/util/Map;Ljava/util/Optional;Ljava/lang/Boolean;)Lnet/minecraft/advancements/Advancement;
private static synthetic lambda$static$5(Ljava/util/Map;)Lnet/minecraft/advancements/AdvancementRequirements;
private static synthetic lambda$static$3(Lnet/minecraft/advancements/Advancement;)Ljava/util/Optional;
private static synthetic lambda$static$0(Ljava/util/Map;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$1()Ljava/lang/String;
static <clinit>()V
```
