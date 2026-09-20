---
type: "interface"
fqcn: "net.minecraft.advancements.Advancement$Builder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.advancements.Advancement$Builder

System: [[20-Systems/net.minecraft.advancements|net.minecraft.advancements]]

`class` public; extends `java/lang/Object`; implements `net/fabricmc/fabric/api/advancement/v1/FabricAdvancementBuilder`, `net/fabricmc/fabric/api/datagen/v1/advancement/FabricAdvancementBuilder`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `()V` | exact | invokespecial@4 in `FabricAdvancementBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `advancement` | `()Lnet/minecraft/advancements/Advancement$Builder;` | exact | invokestatic@0 in `FabricAdvancementProvider.createPlaceholder` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `build` | `(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/advancements/Adva` | exact | invokevirtual@60 in `AdvancementUtil.modifyAdvancement` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `build` | `(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/advancements/Adva` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | declared |
| calls | `build` | `(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/advancements/Adva` | exact | invokevirtual@4 in `FabricAdvancementProvider.createPlaceholder` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `requirements` | `(Lnet/minecraft/advancements/AdvancementRequirements;)Lnet/minecraft/a` | exact | invokevirtual@51 in `FabricAdvancementBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `rewards` | `(Lnet/minecraft/advancements/AdvancementRewards;)Lnet/minecraft/advanc` | exact | invokevirtual@60 in `FabricAdvancementBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `sendsTelemetryEvent` | `()Lnet/minecraft/advancements/Advancement$Builder;` | exact | invokevirtual@90 in `FabricAdvancementBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| injects_into | `addCriterion` | `(Ljava/lang/String;Lnet/minecraft/advancements/triggers/Criterion;)Lne` | exact | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| reads | `criteria` | `Lcom/google/common/collect/ImmutableMap$Builder;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | declared |
| reads | `display` | `Ljava/util/Optional;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | declared |
| reads | `parent` | `Ljava/util/Optional;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | declared |
| reads | `requirements` | `Ljava/util/Optional;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | declared |
| reads | `rewards` | `Lnet/minecraft/advancements/AdvancementRewards;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | declared |
| reads | `sendsTelemetryEvent` | `Z` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | declared |
| wraps | `build` | `(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/advancements/Adva` | exact | @ModifyReceiver at ['INVOKE'] | both | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |

## Declared members (7 fields, 19 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private parent : Ljava/util/Optional;
private display : Ljava/util/Optional;
private rewards : Lnet/minecraft/advancements/AdvancementRewards;
private final criteria : Lcom/google/common/collect/ImmutableMap$Builder;
private requirements : Ljava/util/Optional;
private requirementsStrategy : Lnet/minecraft/advancements/AdvancementRequirements$Strategy;
private sendsTelemetryEvent : Z
public <init>()V
public static advancement()Lnet/minecraft/advancements/Advancement$Builder;
public static recipeAdvancement()Lnet/minecraft/advancements/Advancement$Builder;
public parent(Lnet/minecraft/advancements/AdvancementHolder;)Lnet/minecraft/advancements/Advancement$Builder;
public parent(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/advancements/Advancement$Builder;
public rootDisplay(Lnet/minecraft/world/item/ItemStackTemplate;Lnet/minecraft/network/chat/Component;Lnet/minecraft/network/chat/Component;Lnet/minecraft/resources/Identifier;Lnet/minecraft/advancements/AdvancementType;ZZZ)Lnet/minecraft/advancements/Advancement$Builder;
public rootDisplay(Lnet/minecraft/world/item/Item;Lnet/minecraft/network/chat/Component;Lnet/minecraft/network/chat/Component;Lnet/minecraft/resources/Identifier;Lnet/minecraft/advancements/AdvancementType;ZZZ)Lnet/minecraft/advancements/Advancement$Builder;
public display(Lnet/minecraft/world/item/ItemStackTemplate;Lnet/minecraft/network/chat/Component;Lnet/minecraft/network/chat/Component;Lnet/minecraft/advancements/AdvancementType;ZZZ)Lnet/minecraft/advancements/Advancement$Builder;
public display(Lnet/minecraft/world/item/Item;Lnet/minecraft/network/chat/Component;Lnet/minecraft/network/chat/Component;Lnet/minecraft/advancements/AdvancementType;ZZZ)Lnet/minecraft/advancements/Advancement$Builder;
public display(Lnet/minecraft/advancements/DisplayInfo;)Lnet/minecraft/advancements/Advancement$Builder;
public rewards(Lnet/minecraft/advancements/AdvancementRewards$Builder;)Lnet/minecraft/advancements/Advancement$Builder;
public rewards(Lnet/minecraft/advancements/AdvancementRewards;)Lnet/minecraft/advancements/Advancement$Builder;
public addCriterion(Ljava/lang/String;Lnet/minecraft/advancements/triggers/Criterion;)Lnet/minecraft/advancements/Advancement$Builder;
public requirements(Lnet/minecraft/advancements/AdvancementRequirements$Strategy;)Lnet/minecraft/advancements/Advancement$Builder;
public requirements(Lnet/minecraft/advancements/AdvancementRequirements;)Lnet/minecraft/advancements/Advancement$Builder;
public sendsTelemetryEvent()Lnet/minecraft/advancements/Advancement$Builder;
public build(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/advancements/AdvancementHolder;
public save(Lnet/minecraft/data/worldgen/BootstrapContext;Ljava/lang/String;)Lnet/minecraft/advancements/AdvancementHolder;
private synthetic lambda$build$0(Ljava/util/Map;)Lnet/minecraft/advancements/AdvancementRequirements;
```
