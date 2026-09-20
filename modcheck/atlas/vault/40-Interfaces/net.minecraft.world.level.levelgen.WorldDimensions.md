---
type: "interface"
fqcn: "net.minecraft.world.level.levelgen.WorldDimensions"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.levelgen.WorldDimensions

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `checkStability` | `(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/world/level/dimen` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| wraps | `bake` | `(Lnet/minecraft/core/Registry;)Lnet/minecraft/world/level/levelgen/Wor` | name_only | @WrapMethod | both | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| wraps | `lambda$static$0` | `(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mo` | name_only | @Redirect at ['INVOKE'] | both | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |

## Declared members (3 fields, 29 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final dimensions : Ljava/util/Map;
public static final CODEC : Lcom/mojang/serialization/MapCodec;
private static final BUILTIN_ORDER : Ljava/util/Set;
public <init>(Ljava/util/Map;)V
public <init>(Lnet/minecraft/core/Registry;)V
public static keysInOrder(Ljava/util/Set;)Ljava/util/stream/Stream;
public replaceOverworldGenerator(Lnet/minecraft/core/HolderLookup$Provider;Lnet/minecraft/world/level/chunk/ChunkGenerator;)Lnet/minecraft/world/level/levelgen/WorldDimensions;
public static withOverworld(Lnet/minecraft/core/HolderLookup;Ljava/util/Map;Lnet/minecraft/world/level/chunk/ChunkGenerator;)Ljava/util/Map;
public static withOverworld(Ljava/util/Map;Lnet/minecraft/core/Holder;Lnet/minecraft/world/level/chunk/ChunkGenerator;)Ljava/util/Map;
public overworld()Lnet/minecraft/world/level/chunk/ChunkGenerator;
public get(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;
public levels()Lcom/google/common/collect/ImmutableSet;
public isDebug()Z
private static specialWorldProperty(Lnet/minecraft/core/Registry;)Lnet/minecraft/world/level/storage/PrimaryLevelData$SpecialWorldProperty;
private static checkStability(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/world/level/dimension/LevelStem;)Lcom/mojang/serialization/Lifecycle;
private static isVanillaLike(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/world/level/dimension/LevelStem;)Z
private static isStableOverworld(Lnet/minecraft/world/level/dimension/LevelStem;)Z
private static isStableNether(Lnet/minecraft/world/level/dimension/LevelStem;)Z
private static isStableEnd(Lnet/minecraft/world/level/dimension/LevelStem;)Z
public bake(Lnet/minecraft/core/Registry;)Lnet/minecraft/world/level/levelgen/WorldDimensions$Complete;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public dimensions()Ljava/util/Map;
private static synthetic lambda$bake$3(Lnet/minecraft/core/WritableRegistry;Lnet/minecraft/world/level/levelgen/WorldDimensions$1Entry;)V
private synthetic lambda$bake$0(Lnet/minecraft/core/Registry;Ljava/util/List;Lnet/minecraft/resources/ResourceKey;)V
private static synthetic lambda$bake$2(Ljava/util/List;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/world/level/dimension/LevelStem;)V
private synthetic lambda$bake$1(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;
private static synthetic lambda$specialWorldProperty$0(Lnet/minecraft/world/level/dimension/LevelStem;)Lnet/minecraft/world/level/storage/PrimaryLevelData$SpecialWorldProperty;
private static synthetic lambda$keysInOrder$0(Lnet/minecraft/resources/ResourceKey;)Z
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
static <clinit>()V
```
