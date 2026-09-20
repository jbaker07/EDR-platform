---
type: "interface"
fqcn: "net.minecraft.world.item.alchemy.Potions"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.alchemy.Potions

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `WATER` | `Lnet/minecraft/core/Holder;` | exact | getstatic@2 in `BrewingProviderMixin.preventDuplicatingDefaultTransformations` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `WATER` | `Lnet/minecraft/core/Holder;` | exact | getstatic@50 in `BrewingProviderMixin.preventDuplicatingDefaultTransformations` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `WATER` | `Lnet/minecraft/core/Holder;` | exact | getstatic@15 in `FluidStorage.lambda$static$4` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `WATER` | `Lnet/minecraft/core/Holder;` | exact | getstatic@48 in `WaterPotionStorage.isWaterPotion` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (46 fields, 4 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final WATER : Lnet/minecraft/core/Holder;
public static final MUNDANE : Lnet/minecraft/core/Holder;
public static final THICK : Lnet/minecraft/core/Holder;
public static final AWKWARD : Lnet/minecraft/core/Holder;
public static final NIGHT_VISION : Lnet/minecraft/core/Holder;
public static final LONG_NIGHT_VISION : Lnet/minecraft/core/Holder;
public static final INVISIBILITY : Lnet/minecraft/core/Holder;
public static final LONG_INVISIBILITY : Lnet/minecraft/core/Holder;
public static final LEAPING : Lnet/minecraft/core/Holder;
public static final LONG_LEAPING : Lnet/minecraft/core/Holder;
public static final STRONG_LEAPING : Lnet/minecraft/core/Holder;
public static final FIRE_RESISTANCE : Lnet/minecraft/core/Holder;
public static final LONG_FIRE_RESISTANCE : Lnet/minecraft/core/Holder;
public static final SWIFTNESS : Lnet/minecraft/core/Holder;
public static final LONG_SWIFTNESS : Lnet/minecraft/core/Holder;
public static final STRONG_SWIFTNESS : Lnet/minecraft/core/Holder;
public static final SLOWNESS : Lnet/minecraft/core/Holder;
public static final LONG_SLOWNESS : Lnet/minecraft/core/Holder;
public static final STRONG_SLOWNESS : Lnet/minecraft/core/Holder;
public static final TURTLE_MASTER : Lnet/minecraft/core/Holder;
public static final LONG_TURTLE_MASTER : Lnet/minecraft/core/Holder;
public static final STRONG_TURTLE_MASTER : Lnet/minecraft/core/Holder;
public static final WATER_BREATHING : Lnet/minecraft/core/Holder;
public static final LONG_WATER_BREATHING : Lnet/minecraft/core/Holder;
public static final HEALING : Lnet/minecraft/core/Holder;
public static final STRONG_HEALING : Lnet/minecraft/core/Holder;
public static final HARMING : Lnet/minecraft/core/Holder;
public static final STRONG_HARMING : Lnet/minecraft/core/Holder;
public static final POISON : Lnet/minecraft/core/Holder;
public static final LONG_POISON : Lnet/minecraft/core/Holder;
public static final STRONG_POISON : Lnet/minecraft/core/Holder;
public static final REGENERATION : Lnet/minecraft/core/Holder;
public static final LONG_REGENERATION : Lnet/minecraft/core/Holder;
public static final STRONG_REGENERATION : Lnet/minecraft/core/Holder;
public static final STRENGTH : Lnet/minecraft/core/Holder;
public static final LONG_STRENGTH : Lnet/minecraft/core/Holder;
public static final STRONG_STRENGTH : Lnet/minecraft/core/Holder;
public static final WEAKNESS : Lnet/minecraft/core/Holder;
public static final LONG_WEAKNESS : Lnet/minecraft/core/Holder;
public static final LUCK : Lnet/minecraft/core/Holder;
public static final SLOW_FALLING : Lnet/minecraft/core/Holder;
public static final LONG_SLOW_FALLING : Lnet/minecraft/core/Holder;
public static final WIND_CHARGED : Lnet/minecraft/core/Holder;
public static final WEAVING : Lnet/minecraft/core/Holder;
public static final OOZING : Lnet/minecraft/core/Holder;
public static final INFESTED : Lnet/minecraft/core/Holder;
public <init>()V
private static register(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/world/item/alchemy/Potion;)Lnet/minecraft/core/Holder;
public static bootstrap(Lnet/minecraft/core/Registry;)Lnet/minecraft/core/Holder;
static <clinit>()V
```
