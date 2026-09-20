---
type: "interface"
fqcn: "net.minecraft.world.level.levelgen.structure.templatesystem.StructureTemplate"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.levelgen.structure.templatesystem.StructureTemplate

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `()V` | exact | invokespecial@52 in `StructureTemplateManagerMixin$1.load` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| calls | `load` | `(Lnet/minecraft/core/HolderGetter;Lnet/minecraft/nbt/CompoundTag;)V` | exact | invokevirtual@87 in `StructureTemplateManagerMixin$1.load` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |

## Declared members (16 fields, 47 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
public static final PALETTE_TAG : Ljava/lang/String;
public static final PALETTE_LIST_TAG : Ljava/lang/String;
public static final ENTITIES_TAG : Ljava/lang/String;
public static final BLOCKS_TAG : Ljava/lang/String;
public static final BLOCK_TAG_POS : Ljava/lang/String;
public static final BLOCK_TAG_STATE : Ljava/lang/String;
public static final BLOCK_TAG_NBT : Ljava/lang/String;
public static final ENTITY_TAG_POS : Ljava/lang/String;
public static final ENTITY_TAG_BLOCKPOS : Ljava/lang/String;
public static final ENTITY_TAG_NBT : Ljava/lang/String;
public static final SIZE_TAG : Ljava/lang/String;
private final palettes : Ljava/util/List;
private final entityInfoList : Ljava/util/List;
private size : Lnet/minecraft/core/Vec3i;
private author : Ljava/lang/String;
public <init>()V
public getSize()Lnet/minecraft/core/Vec3i;
public setAuthor(Ljava/lang/String;)V
public getAuthor()Ljava/lang/String;
public fillFromWorld(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Vec3i;ZLjava/util/List;)V
private static addToLists(Lnet/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate$StructureBlockInfo;Ljava/util/List;Ljava/util/List;Ljava/util/List;)V
private static buildInfoList(Ljava/util/List;Ljava/util/List;Ljava/util/List;)Ljava/util/List;
private fillEntityList(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/BlockPos;Lnet/minecraft/util/ProblemReporter;)V
public filterBlocks(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/levelgen/structure/templatesystem/StructurePlaceSettings;Lnet/minecraft/world/level/block/Block;)Ljava/util/List;
public getJigsaws(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/Rotation;)Ljava/util/List;
public filterBlocks(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/levelgen/structure/templatesystem/StructurePlaceSettings;Lnet/minecraft/world/level/block/Block;Z)Lit/unimi/dsi/fastutil/objects/ObjectArrayList;
public calculateConnectedPosition(Lnet/minecraft/world/level/levelgen/structure/templatesystem/StructurePlaceSettings;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/levelgen/structure/templatesystem/StructurePlaceSettings;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/core/BlockPos;
public static calculateRelativePosition(Lnet/minecraft/world/level/levelgen/structure/templatesystem/StructurePlaceSettings;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/core/BlockPos;
public placeInWorld(Lnet/minecraft/world/level/ServerLevelAccessor;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/levelgen/structure/templatesystem/StructurePlaceSettings;Lnet/minecraft/util/RandomSource;I)Z
public static updateShapeAtEdge(Lnet/minecraft/world/level/LevelAccessor;ILnet/minecraft/world/phys/shapes/DiscreteVoxelShape;Lnet/minecraft/core/BlockPos;)V
public static updateShapeAtEdge(Lnet/minecraft/world/level/LevelAccessor;ILnet/minecraft/world/phys/shapes/DiscreteVoxelShape;III)V
public static processBlockInfos(Lnet/minecraft/world/level/ServerLevelAccessor;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/levelgen/structure/templatesystem/StructurePlaceSettings;Ljava/util/List;)Ljava/util/List;
private placeEntities(Lnet/minecraft/world/level/ServerLevelAccessor;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/Mirror;Lnet/minecraft/world/level/block/Rotation;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/levelgen/structure/BoundingBox;ZLnet/minecraft/util/ProblemReporter;)V
private static createEntityIgnoreException(Lnet/minecraft/util/ProblemReporter;Lnet/minecraft/world/level/ServerLevelAccessor;Lnet/minecraft/nbt/CompoundTag;)Ljava/util/Optional;
public getSize(Lnet/minecraft/world/level/block/Rotation;)Lnet/minecraft/core/Vec3i;
public static transform(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/Mirror;Lnet/minecraft/world/level/block/Rotation;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/core/BlockPos;
public static transform(Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/level/block/Mirror;Lnet/minecraft/world/level/block/Rotation;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/phys/Vec3;
public getZeroPositionWithTransform(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/Mirror;Lnet/minecraft/world/level/block/Rotation;)Lnet/minecraft/core/BlockPos;
public static getZeroPositionWithTransform(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/Mirror;Lnet/minecraft/world/level/block/Rotation;II)Lnet/minecraft/core/BlockPos;
public getBoundingBox(Lnet/minecraft/world/level/levelgen/structure/templatesystem/StructurePlaceSettings;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/levelgen/structure/BoundingBox;
public getBoundingBox(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/Rotation;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/Mirror;)Lnet/minecraft/world/level/levelgen/structure/BoundingBox;
protected static getBoundingBox(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/Rotation;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/Mirror;Lnet/minecraft/core/Vec3i;)Lnet/minecraft/world/level/levelgen/structure/BoundingBox;
public save(Lnet/minecraft/nbt/CompoundTag;)Lnet/minecraft/nbt/CompoundTag;
public load(Lnet/minecraft/core/HolderGetter;Lnet/minecraft/nbt/CompoundTag;)V
private loadPalette(Lnet/minecraft/core/HolderGetter;Lnet/minecraft/nbt/ListTag;Lnet/minecraft/nbt/ListTag;)V
private newIntegerList([I)Lnet/minecraft/nbt/ListTag;
private newDoubleList([D)Lnet/minecraft/nbt/ListTag;
public static getJointType(Lnet/minecraft/nbt/CompoundTag;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/level/block/entity/JigsawBlockEntity$JointType;
public static getDefaultJointType(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/level/block/entity/JigsawBlockEntity$JointType;
private static synthetic lambda$getJointType$0(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/level/block/entity/JigsawBlockEntity$JointType;
private static synthetic lambda$loadPalette$0(Lnet/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate$SimplePalette;Ljava/util/List;Ljava/util/List;Ljava/util/List;Lnet/minecraft/nbt/CompoundTag;)V
private synthetic lambda$load$0(Lnet/minecraft/nbt/CompoundTag;)V
private synthetic lambda$load$1(Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/core/BlockPos;Lnet/minecraft/nbt/CompoundTag;)V
private static synthetic lambda$placeEntities$0(Lnet/minecraft/world/level/block/Rotation;Lnet/minecraft/world/level/block/Mirror;Lnet/minecraft/world/phys/Vec3;ZLnet/minecraft/world/level/ServerLevelAccessor;Lnet/minecraft/world/entity/Entity;)V
private static synthetic lambda$updateShapeAtEdge$0(Lnet/minecraft/core/BlockPos$MutableBlockPos;IIILnet/minecraft/core/BlockPos$MutableBlockPos;Lnet/minecraft/world/level/LevelAccessor;ILnet/minecraft/core/Direction;III)V
private static synthetic lambda$fillEntityList$0(Lnet/minecraft/world/entity/Entity;)Z
private static synthetic lambda$buildInfoList$2(Lnet/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate$StructureBlockInfo;)I
private static synthetic lambda$buildInfoList$1(Lnet/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate$StructureBlockInfo;)I
private static synthetic lambda$buildInfoList$0(Lnet/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate$StructureBlockInfo;)I
private static synthetic lambda$fillFromWorld$1(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/Level;Lnet/minecraft/util/ProblemReporter$ScopedCollector;Ljava/util/List;Ljava/util/List;Ljava/util/List;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
private static synthetic lambda$fillFromWorld$0(Ljava/util/List;Lnet/minecraft/world/level/block/state/BlockState;)Z
static <clinit>()V
```
