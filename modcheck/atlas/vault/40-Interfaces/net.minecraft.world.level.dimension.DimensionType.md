---
type: "interface"
fqcn: "net.minecraft.world.level.dimension.DimensionType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.dimension.DimensionType

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `attributes` | `()Lnet/minecraft/world/attribute/EnvironmentAttributeMap;` | exact | invokevirtual@9 in `DimensionModificationImpl.applyChanges` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |

## Declared members (28 fields, 28 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final hasFixedTime : Z
private final hasSkyLight : Z
private final hasCeiling : Z
private final hasEnderDragonFight : Z
private final coordinateScale : D
private final minY : I
private final height : I
private final logicalHeight : I
private final infiniburn : Lnet/minecraft/core/HolderSet;
private final ambientLight : F
private final monsterSettings : Lnet/minecraft/world/level/dimension/DimensionType$MonsterSettings;
private final skybox : Lnet/minecraft/world/level/dimension/DimensionType$Skybox;
private final cardinalLightType : Lnet/minecraft/world/level/CardinalLighting$Type;
private final attributes : Lnet/minecraft/world/attribute/EnvironmentAttributeMap;
private final timelines : Lnet/minecraft/core/HolderSet;
private final defaultClock : Ljava/util/Optional;
public static final BITS_FOR_Y : I
public static final MIN_HEIGHT : I
public static final Y_SIZE : I
public static final MAX_Y : I
public static final MIN_Y : I
public static final WAY_ABOVE_MAX_Y : I
public static final WAY_BELOW_MIN_Y : I
public static final DIRECT_CODEC : Lcom/mojang/serialization/Codec;
public static final NETWORK_CODEC : Lcom/mojang/serialization/Codec;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public static final MOON_BRIGHTNESS_PER_PHASE : [F
public static final CODEC : Lcom/mojang/serialization/Codec;
public <init>(ZZZZDIIILnet/minecraft/core/HolderSet;FLnet/minecraft/world/level/dimension/DimensionType$MonsterSettings;Lnet/minecraft/world/level/dimension/DimensionType$Skybox;Lnet/minecraft/world/level/CardinalLighting$Type;Lnet/minecraft/world/attribute/EnvironmentAttributeMap;Lnet/minecraft/core/HolderSet;Ljava/util/Optional;)V
private static createDirectCodec(Lcom/mojang/serialization/Codec;)Lcom/mojang/serialization/Codec;
public static getTeleportationScale(Lnet/minecraft/world/level/dimension/DimensionType;Lnet/minecraft/world/level/dimension/DimensionType;)D
public static getStorageFolder(Lnet/minecraft/resources/ResourceKey;Ljava/nio/file/Path;)Ljava/nio/file/Path;
public monsterSpawnLightTest()Lnet/minecraft/util/valueproviders/IntProvider;
public monsterSpawnBlockLightLimit()I
public hasEndFlashes()Z
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public hasFixedTime()Z
public hasSkyLight()Z
public hasCeiling()Z
public hasEnderDragonFight()Z
public coordinateScale()D
public minY()I
public height()I
public logicalHeight()I
public infiniburn()Lnet/minecraft/core/HolderSet;
public ambientLight()F
public monsterSettings()Lnet/minecraft/world/level/dimension/DimensionType$MonsterSettings;
public skybox()Lnet/minecraft/world/level/dimension/DimensionType$Skybox;
public cardinalLightType()Lnet/minecraft/world/level/CardinalLighting$Type;
public attributes()Lnet/minecraft/world/attribute/EnvironmentAttributeMap;
public timelines()Lnet/minecraft/core/HolderSet;
public defaultClock()Ljava/util/Optional;
private static synthetic lambda$createDirectCodec$0(Lcom/mojang/serialization/Codec;Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
static <clinit>()V
```
