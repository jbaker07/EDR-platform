---
type: "interface"
fqcn: "net.minecraft.world.item.BucketItem"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.BucketItem

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`class` public; extends `net/minecraft/world/item/Item`; implements `net/minecraft/world/item/DispensibleContainerItem`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `playEmptySound` | `(Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/level/L` | name_only | @ModifyVariable at ['STORE'] | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `content` | `Lnet/minecraft/world/level/material/Fluid;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | declared |

## Declared members (1 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
protected final content : Lnet/minecraft/world/level/material/Fluid;
public <init>(Lnet/minecraft/world/level/material/Fluid;Lnet/minecraft/world/item/Item$Properties;)V
public use(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/InteractionHand;)Lnet/minecraft/world/InteractionResult;
public getFluidContext()Lnet/minecraft/world/level/ClipContext$Fluid;
public static getEmptySuccessItem(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/player/Player;)Lnet/minecraft/world/item/ItemStack;
public checkExtraContent(Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/core/BlockPos;)V
public emptyContents(Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/phys/BlockHitResult;)Z
protected playEmptySound(Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/core/BlockPos;)V
public getContent()Lnet/minecraft/world/level/material/Fluid;
private static synthetic lambda$use$0(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/sounds/SoundEvent;)V
```
