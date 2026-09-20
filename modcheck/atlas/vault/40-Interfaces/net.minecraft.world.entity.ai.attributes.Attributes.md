---
type: "interface"
fqcn: "net.minecraft.world.entity.ai.attributes.Attributes"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.ai.attributes.Attributes

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `MAX_HEALTH` | `Lnet/minecraft/core/Holder;` | exact | getstatic@29 in `HudStatusBarHeightRegistryImpl.lambda$static$1` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `WATER_MOVEMENT_EFFICIENCY` | `Lnet/minecraft/core/Holder;` | exact | getstatic@24 in `SimpleConfiguredFluidBehavior.lambda$static$1` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| reads | `WATER_MOVEMENT_EFFICIENCY` | `Lnet/minecraft/core/Holder;` | exact | getstatic@4 in `SimpleConfiguredFluidBehavior.lambda$static$0` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |

## Declared members (41 fields, 4 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final DEFAULT_ATTACK_SPEED : D
public static final AIR_DRAG_MODIFIER : Lnet/minecraft/core/Holder;
public static final ARMOR : Lnet/minecraft/core/Holder;
public static final ARMOR_TOUGHNESS : Lnet/minecraft/core/Holder;
public static final ATTACK_DAMAGE : Lnet/minecraft/core/Holder;
public static final ATTACK_KNOCKBACK : Lnet/minecraft/core/Holder;
public static final ATTACK_SPEED : Lnet/minecraft/core/Holder;
public static final BELOW_NAME_DISTANCE : Lnet/minecraft/core/Holder;
public static final BLOCK_BREAK_SPEED : Lnet/minecraft/core/Holder;
public static final BLOCK_INTERACTION_RANGE : Lnet/minecraft/core/Holder;
public static final BOUNCINESS : Lnet/minecraft/core/Holder;
public static final BURNING_TIME : Lnet/minecraft/core/Holder;
public static final CAMERA_DISTANCE : Lnet/minecraft/core/Holder;
public static final EXPLOSION_KNOCKBACK_RESISTANCE : Lnet/minecraft/core/Holder;
public static final ENTITY_INTERACTION_RANGE : Lnet/minecraft/core/Holder;
public static final FALL_DAMAGE_MULTIPLIER : Lnet/minecraft/core/Holder;
public static final FLYING_SPEED : Lnet/minecraft/core/Holder;
public static final FOLLOW_RANGE : Lnet/minecraft/core/Holder;
public static final FRICTION_MODIFIER : Lnet/minecraft/core/Holder;
public static final GRAVITY : Lnet/minecraft/core/Holder;
public static final JUMP_STRENGTH : Lnet/minecraft/core/Holder;
public static final KNOCKBACK_RESISTANCE : Lnet/minecraft/core/Holder;
public static final LUCK : Lnet/minecraft/core/Holder;
public static final MAX_ABSORPTION : Lnet/minecraft/core/Holder;
public static final MAX_HEALTH : Lnet/minecraft/core/Holder;
public static final MINING_EFFICIENCY : Lnet/minecraft/core/Holder;
public static final MOVEMENT_EFFICIENCY : Lnet/minecraft/core/Holder;
public static final MOVEMENT_SPEED : Lnet/minecraft/core/Holder;
public static final NAME_TAG_DISTANCE : Lnet/minecraft/core/Holder;
public static final OXYGEN_BONUS : Lnet/minecraft/core/Holder;
public static final SAFE_FALL_DISTANCE : Lnet/minecraft/core/Holder;
public static final SCALE : Lnet/minecraft/core/Holder;
public static final SNEAKING_SPEED : Lnet/minecraft/core/Holder;
public static final SPAWN_REINFORCEMENTS_CHANCE : Lnet/minecraft/core/Holder;
public static final STEP_HEIGHT : Lnet/minecraft/core/Holder;
public static final SUBMERGED_MINING_SPEED : Lnet/minecraft/core/Holder;
public static final SWEEPING_DAMAGE_RATIO : Lnet/minecraft/core/Holder;
public static final TEMPT_RANGE : Lnet/minecraft/core/Holder;
public static final WATER_MOVEMENT_EFFICIENCY : Lnet/minecraft/core/Holder;
public static final WAYPOINT_TRANSMIT_RANGE : Lnet/minecraft/core/Holder;
public static final WAYPOINT_RECEIVE_RANGE : Lnet/minecraft/core/Holder;
public <init>()V
private static register(Ljava/lang/String;Lnet/minecraft/world/entity/ai/attributes/Attribute;)Lnet/minecraft/core/Holder;
public static bootstrap(Lnet/minecraft/core/Registry;)Lnet/minecraft/core/Holder;
static <clinit>()V
```
