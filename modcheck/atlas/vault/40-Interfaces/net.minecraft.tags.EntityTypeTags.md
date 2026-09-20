---
type: "interface"
fqcn: "net.minecraft.tags.EntityTypeTags"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.tags.EntityTypeTags

System: [[20-Systems/net.minecraft.tags|net.minecraft.tags]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `CAN_FLOAT_WHILE_RIDDEN` | `Lnet/minecraft/tags/TagKey;` | exact | getstatic@127 in `SimpleConfiguredFluidBehavior.travelInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |

## Declared members (49 fields, 2 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final SKELETONS : Lnet/minecraft/tags/TagKey;
public static final ZOMBIES : Lnet/minecraft/tags/TagKey;
public static final RAIDERS : Lnet/minecraft/tags/TagKey;
public static final UNDEAD : Lnet/minecraft/tags/TagKey;
public static final BURN_IN_DAYLIGHT : Lnet/minecraft/tags/TagKey;
public static final BEEHIVE_INHABITORS : Lnet/minecraft/tags/TagKey;
public static final ARROWS : Lnet/minecraft/tags/TagKey;
public static final IMPACT_PROJECTILES : Lnet/minecraft/tags/TagKey;
public static final POWDER_SNOW_WALKABLE_MOBS : Lnet/minecraft/tags/TagKey;
public static final AXOLOTL_ALWAYS_HOSTILES : Lnet/minecraft/tags/TagKey;
public static final AXOLOTL_HUNT_TARGETS : Lnet/minecraft/tags/TagKey;
public static final FREEZE_IMMUNE_ENTITY_TYPES : Lnet/minecraft/tags/TagKey;
public static final FREEZE_HURTS_EXTRA_TYPES : Lnet/minecraft/tags/TagKey;
public static final CAN_BREATHE_UNDER_WATER : Lnet/minecraft/tags/TagKey;
public static final FROG_FOOD : Lnet/minecraft/tags/TagKey;
public static final FALL_DAMAGE_IMMUNE : Lnet/minecraft/tags/TagKey;
public static final DISMOUNTS_UNDERWATER : Lnet/minecraft/tags/TagKey;
public static final NON_CONTROLLING_RIDER : Lnet/minecraft/tags/TagKey;
public static final DEFLECTS_PROJECTILES : Lnet/minecraft/tags/TagKey;
public static final CAN_TURN_IN_BOATS : Lnet/minecraft/tags/TagKey;
public static final ILLAGER : Lnet/minecraft/tags/TagKey;
public static final AQUATIC : Lnet/minecraft/tags/TagKey;
public static final ARTHROPOD : Lnet/minecraft/tags/TagKey;
public static final IGNORES_POISON_AND_REGEN : Lnet/minecraft/tags/TagKey;
public static final INVERTED_HEALING_AND_HARM : Lnet/minecraft/tags/TagKey;
public static final WITHER_FRIENDS : Lnet/minecraft/tags/TagKey;
public static final ILLAGER_FRIENDS : Lnet/minecraft/tags/TagKey;
public static final NOT_SCARY_FOR_PUFFERFISH : Lnet/minecraft/tags/TagKey;
public static final SENSITIVE_TO_IMPALING : Lnet/minecraft/tags/TagKey;
public static final SENSITIVE_TO_BANE_OF_ARTHROPODS : Lnet/minecraft/tags/TagKey;
public static final SENSITIVE_TO_SMITE : Lnet/minecraft/tags/TagKey;
public static final NO_ANGER_FROM_WIND_CHARGE : Lnet/minecraft/tags/TagKey;
public static final IMMUNE_TO_OOZING : Lnet/minecraft/tags/TagKey;
public static final IMMUNE_TO_INFESTED : Lnet/minecraft/tags/TagKey;
public static final REDIRECTABLE_PROJECTILE : Lnet/minecraft/tags/TagKey;
public static final BOAT : Lnet/minecraft/tags/TagKey;
public static final CAN_EQUIP_SADDLE : Lnet/minecraft/tags/TagKey;
public static final CAN_EQUIP_HARNESS : Lnet/minecraft/tags/TagKey;
public static final CAN_WEAR_HORSE_ARMOR : Lnet/minecraft/tags/TagKey;
public static final CAN_WEAR_NAUTILUS_ARMOR : Lnet/minecraft/tags/TagKey;
public static final FOLLOWABLE_FRIENDLY_MOBS : Lnet/minecraft/tags/TagKey;
public static final CANNOT_BE_PUSHED_ONTO_BOATS : Lnet/minecraft/tags/TagKey;
public static final ACCEPTS_IRON_GOLEM_GIFT : Lnet/minecraft/tags/TagKey;
public static final CANDIDATE_FOR_IRON_GOLEM_GIFT : Lnet/minecraft/tags/TagKey;
public static final NAUTILUS_HOSTILES : Lnet/minecraft/tags/TagKey;
public static final CAN_FLOAT_WHILE_RIDDEN : Lnet/minecraft/tags/TagKey;
public static final CANNOT_BE_AGE_LOCKED : Lnet/minecraft/tags/TagKey;
public static final NOT_AFFECTED_BY_GEYSERS : Lnet/minecraft/tags/TagKey;
public static final CANNOT_BE_DISMOUNTED_BY_ITEM_USAGE : Lnet/minecraft/tags/TagKey;
private static create(Ljava/lang/String;)Lnet/minecraft/tags/TagKey;
static <clinit>()V
```
