---
type: "interface"
fqcn: "net.minecraft.world.entity.EntitySelector"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.EntitySelector

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

`class` public final; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `ENTITY_STILL_ALIVE` | `Ljava/util/function/Predicate;` | exact | getstatic@7 in `EntityApiLookupImpl.find` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |

## Declared members (8 fields, 14 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final ENTITY_STILL_ALIVE : Ljava/util/function/Predicate;
public static final LIVING_ENTITY_STILL_ALIVE : Ljava/util/function/Predicate;
public static final ENTITY_NOT_BEING_RIDDEN : Ljava/util/function/Predicate;
public static final CONTAINER_ENTITY_SELECTOR : Ljava/util/function/Predicate;
public static final NO_CREATIVE_OR_SPECTATOR : Ljava/util/function/Predicate;
public static final NO_SPECTATORS : Ljava/util/function/Predicate;
public static final CAN_BE_COLLIDED_WITH : Ljava/util/function/Predicate;
public static final CAN_BE_PICKED : Ljava/util/function/Predicate;
private <init>()V
public static withinDistance(DDDD)Ljava/util/function/Predicate;
public static pushableBy(Lnet/minecraft/world/entity/Entity;)Ljava/util/function/Predicate;
public static notRiding(Lnet/minecraft/world/entity/Entity;)Ljava/util/function/Predicate;
private static synthetic lambda$notRiding$0(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/Entity;)Z
private static synthetic lambda$pushableBy$0(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/scores/Team;Lnet/minecraft/world/scores/Team$CollisionRule;Lnet/minecraft/world/entity/Entity;)Z
private static synthetic lambda$withinDistance$0(DDDDLnet/minecraft/world/entity/Entity;)Z
private static synthetic lambda$static$5(Lnet/minecraft/world/entity/Entity;)Z
private static synthetic lambda$static$4(Lnet/minecraft/world/entity/Entity;)Z
private static synthetic lambda$static$3(Lnet/minecraft/world/entity/Entity;)Z
private static synthetic lambda$static$2(Lnet/minecraft/world/entity/Entity;)Z
private static synthetic lambda$static$1(Lnet/minecraft/world/entity/Entity;)Z
private static synthetic lambda$static$0(Lnet/minecraft/world/entity/Entity;)Z
static <clinit>()V
```
