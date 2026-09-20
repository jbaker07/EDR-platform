---
type: "interface"
fqcn: "net.minecraft.world.entity.ai.village.poi.PoiTypes"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.ai.village.poi.PoiTypes

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

`class` public; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `register` | `(Lnet/minecraft/core/Registry;Lnet/minecraft/resources/ResourceKey;Lja` | exact | invokestatic@13 in `PoiHelper.register` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |

## Declared members (25 fields, 14 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final ARMORER : Lnet/minecraft/resources/ResourceKey;
public static final BUTCHER : Lnet/minecraft/resources/ResourceKey;
public static final CARTOGRAPHER : Lnet/minecraft/resources/ResourceKey;
public static final CLERIC : Lnet/minecraft/resources/ResourceKey;
public static final FARMER : Lnet/minecraft/resources/ResourceKey;
public static final FISHERMAN : Lnet/minecraft/resources/ResourceKey;
public static final FLETCHER : Lnet/minecraft/resources/ResourceKey;
public static final LEATHERWORKER : Lnet/minecraft/resources/ResourceKey;
public static final LIBRARIAN : Lnet/minecraft/resources/ResourceKey;
public static final MASON : Lnet/minecraft/resources/ResourceKey;
public static final SHEPHERD : Lnet/minecraft/resources/ResourceKey;
public static final TOOLSMITH : Lnet/minecraft/resources/ResourceKey;
public static final WEAPONSMITH : Lnet/minecraft/resources/ResourceKey;
public static final HOME : Lnet/minecraft/resources/ResourceKey;
public static final MEETING : Lnet/minecraft/resources/ResourceKey;
public static final BEEHIVE : Lnet/minecraft/resources/ResourceKey;
public static final BEE_NEST : Lnet/minecraft/resources/ResourceKey;
public static final NETHER_PORTAL : Lnet/minecraft/resources/ResourceKey;
public static final LODESTONE : Lnet/minecraft/resources/ResourceKey;
public static final LIGHTNING_ROD : Lnet/minecraft/resources/ResourceKey;
public static final TEST_INSTANCE : Lnet/minecraft/resources/ResourceKey;
private static final BEDS : Ljava/util/Set;
private static final CAULDRONS : Ljava/util/Set;
private static final LIGHTNING_RODS : Ljava/util/Set;
private static final TYPE_BY_STATE : Ljava/util/Map;
public <init>()V
private static getBlockStates(Lnet/minecraft/world/level/block/Block;)Ljava/util/Set;
private static createKey(Ljava/lang/String;)Lnet/minecraft/resources/ResourceKey;
private static register(Lnet/minecraft/core/Registry;Lnet/minecraft/resources/ResourceKey;Ljava/util/Set;II)Lnet/minecraft/world/entity/ai/village/poi/PoiType;
private static registerBlockStates(Lnet/minecraft/core/Holder;Ljava/util/Set;)V
public static forState(Lnet/minecraft/world/level/block/state/BlockState;)Ljava/util/Optional;
public static hasPoi(Lnet/minecraft/world/level/block/state/BlockState;)Z
public static bootstrap(Lnet/minecraft/core/Registry;)Lnet/minecraft/world/entity/ai/village/poi/PoiType;
private static synthetic lambda$registerBlockStates$0(Lnet/minecraft/core/Holder;Lnet/minecraft/world/level/block/state/BlockState;)V
private static synthetic lambda$static$3(Lnet/minecraft/world/level/block/Block;)Ljava/util/stream/Stream;
private static synthetic lambda$static$2(Lnet/minecraft/world/level/block/Block;)Ljava/util/stream/Stream;
private static synthetic lambda$static$1(Lnet/minecraft/world/level/block/state/BlockState;)Z
private static synthetic lambda$static$0(Lnet/minecraft/world/level/block/Block;)Ljava/util/stream/Stream;
static <clinit>()V
```
