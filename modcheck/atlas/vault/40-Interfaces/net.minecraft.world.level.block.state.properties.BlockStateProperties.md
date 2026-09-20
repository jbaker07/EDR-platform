---
type: "interface"
fqcn: "net.minecraft.world.level.block.state.properties.BlockStateProperties"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.state.properties.BlockStateProperties

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `WATERLOGGED` | `Lnet/minecraft/world/level/block/state/properties/BooleanProperty;` | exact | getstatic@17 in `FireBlockMixin.getFabricBurnChance` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| reads | `WATERLOGGED` | `Lnet/minecraft/world/level/block/state/properties/BooleanProperty;` | exact | getstatic@27 in `FireBlockMixin.getFabricBurnChance` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| reads | `WATERLOGGED` | `Lnet/minecraft/world/level/block/state/properties/BooleanProperty;` | exact | getstatic@17 in `FireBlockMixin.getFabricSpreadChance` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| reads | `WATERLOGGED` | `Lnet/minecraft/world/level/block/state/properties/BooleanProperty;` | exact | getstatic@27 in `FireBlockMixin.getFabricSpreadChance` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |

## Declared members (141 fields, 4 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final ATTACHED : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final BERRIES : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final BLOOM : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final BOTTOM : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final CAN_SUMMON : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final CONDITIONAL : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final DISARMED : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final DRAG : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final ENABLED : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final EXTENDED : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final EYE : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final FALLING : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final HANGING : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final HAS_BOTTLE_0 : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final HAS_BOTTLE_1 : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final HAS_BOTTLE_2 : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final HAS_RECORD : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final HAS_BOOK : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final INVERTED : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final IN_WALL : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final LIT : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final LOCKED : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final NATURAL : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final OCCUPIED : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final OPEN : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final PERSISTENT : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final POWERED : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final SHORT : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final SHRIEKING : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final SIGNAL_FIRE : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final SNOWY : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final TIP : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final TRIGGERED : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final UNSTABLE : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final WATERLOGGED : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final HORIZONTAL_AXIS : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final AXIS : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final UP : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final DOWN : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final NORTH : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final EAST : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final SOUTH : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final WEST : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final FACING : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final FACING_HOPPER : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final HORIZONTAL_FACING : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final FLOWER_AMOUNT : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
public static final SEGMENT_AMOUNT : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
public static final ORIENTATION : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final ATTACH_FACE : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final BELL_ATTACHMENT : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final EAST_WALL : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final NORTH_WALL : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final SOUTH_WALL : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final WEST_WALL : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final EAST_REDSTONE : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final NORTH_REDSTONE : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final SOUTH_REDSTONE : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final WEST_REDSTONE : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final DOUBLE_BLOCK_HALF : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final HALF : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final SIDE_CHAIN_PART : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final RAIL_SHAPE : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final RAIL_SHAPE_STRAIGHT : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final MAX_AGE_1 : I
public static final MAX_AGE_2 : I
public static final MAX_AGE_3 : I
public static final MAX_AGE_4 : I
public static final MAX_AGE_5 : I
public static final MAX_AGE_7 : I
public static final MAX_AGE_15 : I
public static final MAX_AGE_25 : I
public static final AGE_1 : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
public static final AGE_2 : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
public static final AGE_3 : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
public static final AGE_4 : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
public static final AGE_5 : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
public static final AGE_7 : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
public static final AGE_15 : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
public static final AGE_25 : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
public static final BITES : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
public static final CANDLES : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
public static final DELAY : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
public static final MAX_DISTANCE : I
public static final DISTANCE : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
public static final EGGS : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
public static final HATCH : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
public static final LAYERS : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
public static final MIN_LEVEL : I
public static final MIN_LEVEL_CAULDRON : I
public static final MAX_LEVEL_3 : I
public static final MAX_LEVEL_8 : I
public static final LEVEL_CAULDRON : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
public static final LEVEL_COMPOSTER : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
public static final LEVEL_FLOWING : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
public static final LEVEL_HONEY : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
public static final MAX_LEVEL_15 : I
public static final LEVEL : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
public static final MOISTURE : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
public static final NOTE : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
public static final PICKLES : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
public static final POWER : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
public static final STAGE : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
public static final STABILITY_MAX_DISTANCE : I
public static final STABILITY_DISTANCE : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
public static final MIN_RESPAWN_ANCHOR_CHARGES : I
public static final MAX_RESPAWN_ANCHOR_CHARGES : I
public static final RESPAWN_ANCHOR_CHARGES : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
public static final DRIED_GHAST_HYDRATION_LEVELS : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
public static final ROTATION_16 : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
public static final BED_PART : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final CHEST_TYPE : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final MODE_COMPARATOR : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final DOOR_HINGE : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final NOTEBLOCK_INSTRUMENT : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final PISTON_TYPE : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final SLAB_TYPE : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final STAIRS_SHAPE : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final STRUCTUREBLOCK_MODE : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final BAMBOO_LEAVES : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final TILT : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final VERTICAL_DIRECTION : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final SPELEOTHEM_THICKNESS : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final SCULK_SENSOR_PHASE : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final SLOT_0_OCCUPIED : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final SLOT_1_OCCUPIED : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final SLOT_2_OCCUPIED : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final SLOT_3_OCCUPIED : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final SLOT_4_OCCUPIED : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final SLOT_5_OCCUPIED : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final DUSTED : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
public static final CRACKED : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final CRAFTING : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final TRIAL_SPAWNER_STATE : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final VAULT_STATE : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final CREAKING_HEART_STATE : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final OMINOUS : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final TEST_BLOCK_MODE : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final MAP : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final COPPER_GOLEM_POSE : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final POTENT_SULFUR_STATE : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public <init>()V
private static synthetic lambda$static$1(Lnet/minecraft/world/level/block/state/properties/RailShape;)Z
private static synthetic lambda$static$0(Lnet/minecraft/core/Direction;)Z
static <clinit>()V
```
