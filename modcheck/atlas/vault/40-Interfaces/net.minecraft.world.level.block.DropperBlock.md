---
type: "interface"
fqcn: "net.minecraft.world.level.block.DropperBlock"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.DropperBlock

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `dispenseFrom` | `@Inject at INVOKE Lnet/minecraft/core/dispenser/DispenseItemBehavior;dispense(Ln` | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (7, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.block.DropperBlock extends net.minecraft.world.level.block.DispenserBlock {
    private static final org.slf4j.Logger LOGGER;
    private static final net.minecraft.core.dispenser.DispenseItemBehavior DISPENSE_BEHAVIOUR;
    public net.minecraft.world.level.block.DropperBlock(net.minecraft.world.level.block.state.BlockBehaviour$Properties);
    protected net.minecraft.core.dispenser.DispenseItemBehavior getDispenseMethod(net.minecraft.world.level.Level, net.minecraft.world.item.ItemStack);
    public net.minecraft.world.level.block.entity.BlockEntity newBlockEntity(net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    protected void dispenseFrom(net.minecraft.server.level.ServerLevel, net.minecraft.world.level.block.state.BlockState, net.minecraft.core.BlockPos);
    static {};
}
```
