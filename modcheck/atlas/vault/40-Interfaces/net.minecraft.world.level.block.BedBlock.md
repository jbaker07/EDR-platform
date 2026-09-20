---
type: "interface"
fqcn: "net.minecraft.world.level.block.BedBlock"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.BedBlock

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| reads | `OCCUPIEDLnet/minecraft/world/level/block/state/properties/BooleanPro` | `` | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |

## Declared members (10, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.block.BedBlock extends net.minecraft.world.level.block.AbstractBedBlock {
    private final net.minecraft.world.item.DyeColor color;
    private static final java.util.Map<net.minecraft.core.Direction, net.minecraft.world.phys.shapes.VoxelShape> SHAPES;
    public net.minecraft.world.level.block.BedBlock(net.minecraft.world.item.DyeColor, net.minecraft.world.level.block.state.BlockBehaviour$Properties);
    protected net.minecraft.world.phys.shapes.VoxelShape getShape(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.BlockGetter, net.minecraft.core.BlockPos, net.minecraft.world.phys.shapes.CollisionContext);
    protected net.minecraft.world.attribute.EnvironmentAttribute<net.minecraft.world.attribute.BedRule> getBedEnvironmentAttribute();
    protected net.minecraft.world.InteractionResult destroyOnUse(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.entity.player.Player);
    protected void destroyOnLeave(net.minecraft.world.level.Level, net.minecraft.core.BlockPos);
    public net.minecraft.world.item.DyeColor getColor();
    private static java.util.Map lambda$static$0();
    static {};
}
```
