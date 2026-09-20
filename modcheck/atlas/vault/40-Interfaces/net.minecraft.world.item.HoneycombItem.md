---
type: "interface"
fqcn: "net.minecraft.world.item.HoneycombItem"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.HoneycombItem

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`class` public; extends `net/minecraft/world/item/Item`; implements `net/minecraft/world/item/SignApplicator`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `lambda$static$0` | `?` | ambiguous | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| reads | `WAXABLES` | `Ljava/util/function/Supplier;` | exact | getstatic@14 in `OxidizableBlocksRegistryImpl.registerWaxable` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |

## Declared members (3 fields, 22 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final WAXABLES : Ljava/util/function/Supplier;
public static final WAX_OFF_BY_BLOCK : Ljava/util/function/Supplier;
public static final WAXED_RECIPES : Lcom/google/common/collect/ImmutableMap;
public <init>(Lnet/minecraft/world/item/Item$Properties;)V
public useOn(Lnet/minecraft/world/item/context/UseOnContext;)Lnet/minecraft/world/InteractionResult;
public static getWaxed(Lnet/minecraft/world/level/block/state/BlockState;)Ljava/util/Optional;
public tryApplyToSign(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/level/block/entity/SignBlockEntity;Lnet/minecraft/world/level/block/entity/SignTextSlot;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/player/Player;)Z
public canApplyToSign(Lnet/minecraft/world/level/block/entity/SignText;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/player/Player;)Z
private static synthetic lambda$getWaxed$0(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/block/state/BlockState;
private static synthetic lambda$useOn$0(Lnet/minecraft/world/item/context/UseOnContext;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/InteractionResult;
private static synthetic lambda$static$13(Lcom/google/common/collect/ImmutableMap$Builder;Lnet/minecraft/world/item/HoneycombItem$WaxedRecipeGroup;Lnet/minecraft/world/level/block/Block;)V
private static synthetic lambda$static$12(Lnet/minecraft/world/level/block/Block;)Lcom/mojang/datafixers/util/Pair;
private static synthetic lambda$static$11(Lnet/minecraft/world/level/block/Block;)Lcom/mojang/datafixers/util/Pair;
private static synthetic lambda$static$10(Lnet/minecraft/world/level/block/Block;)Lcom/mojang/datafixers/util/Pair;
private static synthetic lambda$static$9(Lnet/minecraft/world/level/block/Block;)Lcom/mojang/datafixers/util/Pair;
private static synthetic lambda$static$8(Lnet/minecraft/world/level/block/Block;)Lcom/mojang/datafixers/util/Pair;
private static synthetic lambda$static$7(Lnet/minecraft/world/level/block/Block;)Lcom/mojang/datafixers/util/Pair;
private static synthetic lambda$static$6(Lnet/minecraft/world/level/block/Block;)Lcom/mojang/datafixers/util/Pair;
private static synthetic lambda$static$5(Lnet/minecraft/world/level/block/Block;)Lcom/mojang/datafixers/util/Pair;
private static synthetic lambda$static$4(Lnet/minecraft/world/level/block/Block;)Lcom/mojang/datafixers/util/Pair;
private static synthetic lambda$static$3(Lnet/minecraft/world/level/block/Block;)Lcom/mojang/datafixers/util/Pair;
private static synthetic lambda$static$2()Lcom/google/common/collect/BiMap;
private static synthetic lambda$static$0()Lcom/google/common/collect/BiMap;
private static synthetic lambda$static$1(Lcom/google/common/collect/ImmutableBiMap$Builder;Lnet/minecraft/world/level/block/WeatheringCopperCollection;)V
static <clinit>()V
```
