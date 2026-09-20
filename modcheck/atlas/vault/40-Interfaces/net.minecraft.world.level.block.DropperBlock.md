---
type: "interface"
fqcn: "net.minecraft.world.level.block.DropperBlock"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.DropperBlock

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `net/minecraft/world/level/block/DispenserBlock`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `dispenseFrom` | `(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/level/bl` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (2 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final DISPENSE_BEHAVIOUR : Lnet/minecraft/core/dispenser/DispenseItemBehavior;
public <init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V
protected getDispenseMethod(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/item/ItemStack;)Lnet/minecraft/core/dispenser/DispenseItemBehavior;
public newBlockEntity(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/level/block/entity/BlockEntity;
protected dispenseFrom(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;)V
static <clinit>()V
```
