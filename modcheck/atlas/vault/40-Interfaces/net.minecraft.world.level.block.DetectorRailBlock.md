---
type: "interface"
fqcn: "net.minecraft.world.level.block.DetectorRailBlock"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.DetectorRailBlock

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `net/minecraft/world/level/block/BaseRailBlock`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getInteractingMinecartOfType` | `(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Ljava/l` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | declared |
| injects_into | `getAnalogOutputSignal` | `(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/worl` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| reads | `POWERED` | `Lnet/minecraft/world/level/block/state/properties/BooleanProperty;` | exact | getstatic@1 in `DetectorRailBlockMixin.getCustomComparatorOutput` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |

## Declared members (3 fields, 20 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final SHAPE : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final POWERED : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
private static final PRESSED_CHECK_PERIOD : I
public <init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V
protected isSignalSource(Lnet/minecraft/world/level/block/state/BlockState;)Z
protected ownSignal(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)I
protected entityInside(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/InsideBlockEffectApplier;Z)V
protected tick(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/util/RandomSource;)V
protected getDirectSignal(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;)I
private checkPressed(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
protected updatePowerToConnected(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Z)V
protected onPlace(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Z)V
public getShapeProperty()Lnet/minecraft/world/level/block/state/properties/Property;
protected hasAnalogOutputSignal(Lnet/minecraft/world/level/block/state/BlockState;)Z
protected getAnalogOutputSignal(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;)I
private getInteractingMinecartOfType(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Ljava/lang/Class;Ljava/util/function/Predicate;)Ljava/util/List;
private getSearchBB(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/phys/AABB;
protected rotate(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/Rotation;)Lnet/minecraft/world/level/block/state/BlockState;
protected mirror(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/Mirror;)Lnet/minecraft/world/level/block/state/BlockState;
protected createBlockStateDefinition(Lnet/minecraft/world/level/block/state/StateDefinition$Builder;)V
private static synthetic lambda$getAnalogOutputSignal$0(Lnet/minecraft/world/entity/Entity;)Z
private static synthetic lambda$checkPressed$0(Lnet/minecraft/world/entity/Entity;)Z
static <clinit>()V
```
