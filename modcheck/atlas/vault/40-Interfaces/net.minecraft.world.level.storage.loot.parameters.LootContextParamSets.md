---
type: "interface"
fqcn: "net.minecraft.world.level.storage.loot.parameters.LootContextParamSets"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.loot.parameters.LootContextParamSets

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `BLOCK` | `Lnet/minecraft/util/context/ContextKeySet;` | exact | getstatic@2 in `FabricBlockLootSubProvider.run` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `BLOCK_INTERACT` | `Lnet/minecraft/util/context/ContextKeySet;` | exact | getstatic@29 in `ComposterWrapper.getLayersToAdd` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `ENTITY` | `Lnet/minecraft/util/context/ContextKeySet;` | exact | getstatic@2 in `FabricEntityLootSubProvider.run` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (32 fields, 38 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final EMPTY : Lnet/minecraft/util/context/ContextKeySet;
public static final ALL_PARAMS : Lnet/minecraft/util/context/ContextKeySet;
public static final CHEST : Lnet/minecraft/util/context/ContextKeySet;
public static final COMMAND : Lnet/minecraft/util/context/ContextKeySet;
public static final COMMAND_SLOT_SOURCE : Lnet/minecraft/util/context/ContextKeySet;
public static final COMMAND_COMPUTE_DEFAULT : Lnet/minecraft/util/context/ContextKeySet;
public static final COMMAND_COMPUTE_POSITION : Lnet/minecraft/util/context/ContextKeySet;
public static final COMMAND_COMPUTE_ENTITY : Lnet/minecraft/util/context/ContextKeySet;
public static final SELECTOR : Lnet/minecraft/util/context/ContextKeySet;
public static final VILLAGER_TRADE : Lnet/minecraft/util/context/ContextKeySet;
public static final FISHING : Lnet/minecraft/util/context/ContextKeySet;
public static final ENTITY : Lnet/minecraft/util/context/ContextKeySet;
public static final EQUIPMENT : Lnet/minecraft/util/context/ContextKeySet;
public static final ARCHAEOLOGY : Lnet/minecraft/util/context/ContextKeySet;
public static final GIFT : Lnet/minecraft/util/context/ContextKeySet;
public static final PIGLIN_BARTER : Lnet/minecraft/util/context/ContextKeySet;
public static final VAULT : Lnet/minecraft/util/context/ContextKeySet;
public static final ADVANCEMENT_REWARD : Lnet/minecraft/util/context/ContextKeySet;
public static final ADVANCEMENT_ENTITY : Lnet/minecraft/util/context/ContextKeySet;
public static final ADVANCEMENT_LOCATION : Lnet/minecraft/util/context/ContextKeySet;
public static final BLOCK_USE : Lnet/minecraft/util/context/ContextKeySet;
public static final BLOCK : Lnet/minecraft/util/context/ContextKeySet;
public static final SHEARING : Lnet/minecraft/util/context/ContextKeySet;
public static final ENTITY_INTERACT : Lnet/minecraft/util/context/ContextKeySet;
public static final BLOCK_INTERACT : Lnet/minecraft/util/context/ContextKeySet;
public static final CONTAINER_PROCESS : Lnet/minecraft/util/context/ContextKeySet;
public static final ENCHANTED_DAMAGE : Lnet/minecraft/util/context/ContextKeySet;
public static final ENCHANTED_ITEM : Lnet/minecraft/util/context/ContextKeySet;
public static final ENCHANTED_LOCATION : Lnet/minecraft/util/context/ContextKeySet;
public static final ENCHANTED_ENTITY : Lnet/minecraft/util/context/ContextKeySet;
public static final HIT_BLOCK : Lnet/minecraft/util/context/ContextKeySet;
public <init>()V
private static register(Ljava/lang/String;Ljava/util/function/Consumer;)Lnet/minecraft/util/context/ContextKeySet;
private static register(Ljava/util/function/Consumer;Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/util/context/ContextKeySet;
public static bootstrap(Lnet/minecraft/core/Registry;)Lnet/minecraft/util/context/ContextKeySet;
public static validate()V
private static synthetic lambda$validate$0(Ljava/util/Set;Lnet/minecraft/util/context/ContextKeySet;)V
private static synthetic lambda$static$30(Lnet/minecraft/util/context/ContextKeySet$Builder;)V
private static synthetic lambda$static$29(Lnet/minecraft/util/context/ContextKeySet$Builder;)V
private static synthetic lambda$static$28(Lnet/minecraft/util/context/ContextKeySet$Builder;)V
private static synthetic lambda$static$27(Lnet/minecraft/util/context/ContextKeySet$Builder;)V
private static synthetic lambda$static$26(Lnet/minecraft/util/context/ContextKeySet$Builder;)V
private static synthetic lambda$static$25(Lnet/minecraft/util/context/ContextKeySet$Builder;)V
private static synthetic lambda$static$24(Lnet/minecraft/util/context/ContextKeySet$Builder;)V
private static synthetic lambda$static$23(Lnet/minecraft/util/context/ContextKeySet$Builder;)V
private static synthetic lambda$static$22(Lnet/minecraft/util/context/ContextKeySet$Builder;)V
private static synthetic lambda$static$21(Lnet/minecraft/util/context/ContextKeySet$Builder;)V
private static synthetic lambda$static$20(Lnet/minecraft/util/context/ContextKeySet$Builder;)V
private static synthetic lambda$static$19(Lnet/minecraft/util/context/ContextKeySet$Builder;)V
private static synthetic lambda$static$18(Lnet/minecraft/util/context/ContextKeySet$Builder;)V
private static synthetic lambda$static$17(Lnet/minecraft/util/context/ContextKeySet$Builder;)V
private static synthetic lambda$static$16(Lnet/minecraft/util/context/ContextKeySet$Builder;)V
private static synthetic lambda$static$15(Lnet/minecraft/util/context/ContextKeySet$Builder;)V
private static synthetic lambda$static$14(Lnet/minecraft/util/context/ContextKeySet$Builder;)V
private static synthetic lambda$static$13(Lnet/minecraft/util/context/ContextKeySet$Builder;)V
private static synthetic lambda$static$12(Lnet/minecraft/util/context/ContextKeySet$Builder;)V
private static synthetic lambda$static$11(Lnet/minecraft/util/context/ContextKeySet$Builder;)V
private static synthetic lambda$static$10(Lnet/minecraft/util/context/ContextKeySet$Builder;)V
private static synthetic lambda$static$9(Lnet/minecraft/util/context/ContextKeySet$Builder;)V
private static synthetic lambda$static$8(Lnet/minecraft/util/context/ContextKeySet$Builder;)V
private static synthetic lambda$static$7(Lnet/minecraft/util/context/ContextKeySet$Builder;)V
private static synthetic lambda$static$6(Lnet/minecraft/util/context/ContextKeySet$Builder;)V
private static synthetic lambda$static$5(Lnet/minecraft/util/context/ContextKeySet$Builder;)V
private static synthetic lambda$static$4(Lnet/minecraft/util/context/ContextKeySet$Builder;)V
private static synthetic lambda$static$3(Lnet/minecraft/util/context/ContextKeySet$Builder;)V
private static synthetic lambda$static$2(Lnet/minecraft/util/context/ContextKeySet$Builder;)V
private static synthetic lambda$static$1(Lnet/minecraft/util/context/ContextKeySet$Builder;)V
private static synthetic lambda$static$0(Lnet/minecraft/util/context/ContextKeySet$Builder;)V
static <clinit>()V
```
