---
type: "interface"
fqcn: "net.minecraft.util.datafix.DataFixTypes"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.datafix.DataFixTypes

System: [[20-Systems/net.minecraft.util.datafix|net.minecraft.util.datafix]]

`enum` public final; extends `java/lang/Enum`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `updateToCurrentVersion` | `(Lcom/mojang/datafixers/DataFixer;Lnet/minecraft/nbt/CompoundTag;I)Lne` | exact | invokevirtual@84 in `StructureTemplateManagerMixin$1.load` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| reads | `STRUCTURE` | `Lnet/minecraft/util/datafix/DataFixTypes;` | exact | getstatic@73 in `StructureTemplateManagerMixin$1.load` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |

## Declared members (34 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final LEVEL : Lnet/minecraft/util/datafix/DataFixTypes;
public static final LEVEL_SUMMARY : Lnet/minecraft/util/datafix/DataFixTypes;
public static final PLAYER : Lnet/minecraft/util/datafix/DataFixTypes;
public static final CHUNK : Lnet/minecraft/util/datafix/DataFixTypes;
public static final HOTBAR : Lnet/minecraft/util/datafix/DataFixTypes;
public static final OPTIONS : Lnet/minecraft/util/datafix/DataFixTypes;
public static final STRUCTURE : Lnet/minecraft/util/datafix/DataFixTypes;
public static final STATS : Lnet/minecraft/util/datafix/DataFixTypes;
public static final SAVED_DATA_COMMAND_STORAGE : Lnet/minecraft/util/datafix/DataFixTypes;
public static final SAVED_DATA_CUSTOM_BOSS_EVENTS : Lnet/minecraft/util/datafix/DataFixTypes;
public static final SAVED_DATA_ENDER_DRAGON_FIGHT : Lnet/minecraft/util/datafix/DataFixTypes;
public static final SAVED_DATA_GAME_RULES : Lnet/minecraft/util/datafix/DataFixTypes;
public static final SAVED_DATA_FORCED_CHUNKS : Lnet/minecraft/util/datafix/DataFixTypes;
public static final SAVED_DATA_MAP_DATA : Lnet/minecraft/util/datafix/DataFixTypes;
public static final SAVED_DATA_MAP_INDEX : Lnet/minecraft/util/datafix/DataFixTypes;
public static final SAVED_DATA_RAIDS : Lnet/minecraft/util/datafix/DataFixTypes;
public static final SAVED_DATA_RANDOM_SEQUENCES : Lnet/minecraft/util/datafix/DataFixTypes;
public static final SAVED_DATA_SCHEDULED_EVENTS : Lnet/minecraft/util/datafix/DataFixTypes;
public static final SAVED_DATA_SCOREBOARD : Lnet/minecraft/util/datafix/DataFixTypes;
public static final SAVED_DATA_STOPWATCHES : Lnet/minecraft/util/datafix/DataFixTypes;
public static final SAVED_DATA_STRUCTURE_FEATURE_INDICES : Lnet/minecraft/util/datafix/DataFixTypes;
public static final SAVED_DATA_WANDERING_TRADER : Lnet/minecraft/util/datafix/DataFixTypes;
public static final SAVED_DATA_WEATHER : Lnet/minecraft/util/datafix/DataFixTypes;
public static final SAVED_DATA_WORLD_BORDER : Lnet/minecraft/util/datafix/DataFixTypes;
public static final SAVED_DATA_WORLD_CLOCKS : Lnet/minecraft/util/datafix/DataFixTypes;
public static final SAVED_DATA_WORLD_GEN_SETTINGS : Lnet/minecraft/util/datafix/DataFixTypes;
public static final ADVANCEMENTS : Lnet/minecraft/util/datafix/DataFixTypes;
public static final POI_CHUNK : Lnet/minecraft/util/datafix/DataFixTypes;
public static final WORLD_GEN_SETTINGS : Lnet/minecraft/util/datafix/DataFixTypes;
public static final ENTITY_CHUNK : Lnet/minecraft/util/datafix/DataFixTypes;
public static final DEBUG_PROFILE : Lnet/minecraft/util/datafix/DataFixTypes;
public static final TYPES_FOR_LEVEL_LIST : Ljava/util/Set;
private final type : Lcom/mojang/datafixers/DSL$TypeReference;
private static final synthetic $VALUES : [Lnet/minecraft/util/datafix/DataFixTypes;
public static values()[Lnet/minecraft/util/datafix/DataFixTypes;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/util/datafix/DataFixTypes;
private <init>(Ljava/lang/String;ILcom/mojang/datafixers/DSL$TypeReference;)V
private static currentVersion()I
public wrapCodec(Lcom/mojang/serialization/Codec;Lcom/mojang/datafixers/DataFixer;I)Lcom/mojang/serialization/Codec;
public update(Lcom/mojang/datafixers/DataFixer;Lcom/mojang/serialization/Dynamic;II)Lcom/mojang/serialization/Dynamic;
public updateToCurrentVersion(Lcom/mojang/datafixers/DataFixer;Lcom/mojang/serialization/Dynamic;I)Lcom/mojang/serialization/Dynamic;
public update(Lcom/mojang/datafixers/DataFixer;Lnet/minecraft/nbt/CompoundTag;II)Lnet/minecraft/nbt/CompoundTag;
public updateToCurrentVersion(Lcom/mojang/datafixers/DataFixer;Lnet/minecraft/nbt/CompoundTag;I)Lnet/minecraft/nbt/CompoundTag;
private static synthetic $values()[Lnet/minecraft/util/datafix/DataFixTypes;
static <clinit>()V
```
