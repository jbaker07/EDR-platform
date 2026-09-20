---
type: "interface"
fqcn: "net.minecraft.world.level.block.BedBlock"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.BedBlock

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `net/minecraft/world/level/block/AbstractBedBlock`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `OCCUPIED` | `Lnet/minecraft/world/level/block/state/properties/BooleanProperty;` | inherited_exact | getstatic@8 in `LivingEntityMixin.setOccupiedState` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| reads | `OCCUPIED` | `Lnet/minecraft/world/level/block/state/properties/BooleanProperty;` | inherited_exact | getstatic@52 in `LivingEntityMixin.setOccupiedState` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| reads | `OCCUPIED` | `Lnet/minecraft/world/level/block/state/properties/BooleanProperty;` | inherited_exact | getstatic@65 in `LivingEntityMixin.setOccupiedState` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |

## Declared members (2 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final color : Lnet/minecraft/world/item/DyeColor;
private static final SHAPES : Ljava/util/Map;
public <init>(Lnet/minecraft/world/item/DyeColor;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V
protected getShape(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/phys/shapes/CollisionContext;)Lnet/minecraft/world/phys/shapes/VoxelShape;
protected getBedEnvironmentAttribute()Lnet/minecraft/world/attribute/EnvironmentAttribute;
protected destroyOnUse(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/player/Player;)Lnet/minecraft/world/InteractionResult;
protected destroyOnLeave(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;)V
public getColor()Lnet/minecraft/world/item/DyeColor;
private static synthetic lambda$static$0()Ljava/util/Map;
static <clinit>()V
```
