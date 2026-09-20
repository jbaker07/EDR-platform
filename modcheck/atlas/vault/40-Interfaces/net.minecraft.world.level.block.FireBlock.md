---
type: "interface"
fqcn: "net.minecraft.world.level.block.FireBlock"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.FireBlock

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `net/minecraft/world/level/block/BaseFireBlock`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getBurnOdds` | `(Lnet/minecraft/world/level/block/state/BlockState;)I` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | declared |
| calls | `getIgniteOdds` | `(Lnet/minecraft/world/level/block/state/BlockState;)I` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | declared |
| injects_into | `<init>` | `(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| injects_into | `getBurnOdds` | `(Lnet/minecraft/world/level/block/state/BlockState;)I` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| injects_into | `getIgniteOdds` | `(Lnet/minecraft/world/level/block/state/BlockState;)I` | exact | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |

## Declared members (19 fields, 28 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final MAX_AGE : I
public static final AGE : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
public static final NORTH : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final EAST : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final SOUTH : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final WEST : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final UP : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final PROPERTY_BY_DIRECTION : Ljava/util/Map;
private final shapes : Ljava/util/function/Function;
private static final IGNITE_INSTANT : I
private static final IGNITE_EASY : I
private static final IGNITE_MEDIUM : I
private static final IGNITE_HARD : I
private static final BURN_INSTANT : I
private static final BURN_EASY : I
private static final BURN_MEDIUM : I
private static final BURN_HARD : I
private final igniteOdds : Lit/unimi/dsi/fastutil/objects/Object2IntMap;
private final burnOdds : Lit/unimi/dsi/fastutil/objects/Object2IntMap;
public <init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V
private makeShapes()Ljava/util/function/Function;
protected updateShape(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/LevelReader;Lnet/minecraft/world/level/ScheduledTickAccess;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/util/RandomSource;)Lnet/minecraft/world/level/block/state/BlockState;
protected getShape(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/phys/shapes/CollisionContext;)Lnet/minecraft/world/phys/shapes/VoxelShape;
public getStateForPlacement(Lnet/minecraft/world/item/context/BlockPlaceContext;)Lnet/minecraft/world/level/block/state/BlockState;
protected getStateForPlacement(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/BlockState;
protected canSurvive(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/LevelReader;Lnet/minecraft/core/BlockPos;)Z
protected tick(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/util/RandomSource;)V
protected isNearRain(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;)Z
private getBurnOdds(Lnet/minecraft/world/level/block/state/BlockState;)I
private getIgniteOdds(Lnet/minecraft/world/level/block/state/BlockState;)I
private checkBurnOut(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;ILnet/minecraft/util/RandomSource;I)V
private getStateWithAge(Lnet/minecraft/world/level/LevelReader;Lnet/minecraft/core/BlockPos;I)Lnet/minecraft/world/level/block/state/BlockState;
private isValidFireLocation(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)Z
private getIgniteOdds(Lnet/minecraft/world/level/LevelReader;Lnet/minecraft/core/BlockPos;)I
protected canBurn(Lnet/minecraft/world/level/block/state/BlockState;)Z
protected onPlace(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Z)V
private static getFireTickDelay(Lnet/minecraft/util/RandomSource;)I
protected createBlockStateDefinition(Lnet/minecraft/world/level/block/state/StateDefinition$Builder;)V
private setFlammable(Lnet/minecraft/world/level/block/Block;II)V
public static bootStrap()V
private static synthetic lambda$bootStrap$3(Lnet/minecraft/world/level/block/FireBlock;Lnet/minecraft/world/level/block/Block;)V
private static synthetic lambda$bootStrap$2(Lnet/minecraft/world/level/block/FireBlock;Lnet/minecraft/world/level/block/Block;)V
private static synthetic lambda$bootStrap$1(Lnet/minecraft/world/level/block/FireBlock;Lnet/minecraft/world/level/block/Block;)V
private static synthetic lambda$bootStrap$0(Lnet/minecraft/world/level/block/FireBlock;Lnet/minecraft/world/level/block/Block;)V
private static synthetic lambda$makeShapes$0(Ljava/util/Map;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/phys/shapes/VoxelShape;
private static synthetic lambda$static$0(Ljava/util/Map$Entry;)Z
static <clinit>()V
```
