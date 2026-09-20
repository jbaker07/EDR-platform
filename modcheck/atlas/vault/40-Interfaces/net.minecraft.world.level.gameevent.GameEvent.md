---
type: "interface"
fqcn: "net.minecraft.world.level.gameevent.GameEvent"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.gameevent.GameEvent

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `BLOCK_CHANGE` | `Lnet/minecraft/core/Holder$Reference;` | exact | getstatic@46 in `ComposterWrapper.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `BLOCK_CHANGE` | `Lnet/minecraft/core/Holder$Reference;` | exact | getstatic@222 in `ComposterWrapper.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (64 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final notificationRadius : I
public static final BLOCK_ACTIVATE : Lnet/minecraft/core/Holder$Reference;
public static final BLOCK_ATTACH : Lnet/minecraft/core/Holder$Reference;
public static final BLOCK_CHANGE : Lnet/minecraft/core/Holder$Reference;
public static final BLOCK_CLOSE : Lnet/minecraft/core/Holder$Reference;
public static final BLOCK_DEACTIVATE : Lnet/minecraft/core/Holder$Reference;
public static final BLOCK_DESTROY : Lnet/minecraft/core/Holder$Reference;
public static final BLOCK_DETACH : Lnet/minecraft/core/Holder$Reference;
public static final BLOCK_OPEN : Lnet/minecraft/core/Holder$Reference;
public static final BLOCK_PLACE : Lnet/minecraft/core/Holder$Reference;
public static final BOUNCE : Lnet/minecraft/core/Holder$Reference;
public static final CONTAINER_CLOSE : Lnet/minecraft/core/Holder$Reference;
public static final CONTAINER_OPEN : Lnet/minecraft/core/Holder$Reference;
public static final DRINK : Lnet/minecraft/core/Holder$Reference;
public static final EAT : Lnet/minecraft/core/Holder$Reference;
public static final ELYTRA_GLIDE : Lnet/minecraft/core/Holder$Reference;
public static final ENTITY_DAMAGE : Lnet/minecraft/core/Holder$Reference;
public static final ENTITY_DIE : Lnet/minecraft/core/Holder$Reference;
public static final ENTITY_DISMOUNT : Lnet/minecraft/core/Holder$Reference;
public static final ENTITY_INTERACT : Lnet/minecraft/core/Holder$Reference;
public static final ENTITY_MOUNT : Lnet/minecraft/core/Holder$Reference;
public static final ENTITY_PLACE : Lnet/minecraft/core/Holder$Reference;
public static final ENTITY_ACTION : Lnet/minecraft/core/Holder$Reference;
public static final EQUIP : Lnet/minecraft/core/Holder$Reference;
public static final EXPLODE : Lnet/minecraft/core/Holder$Reference;
public static final FLAP : Lnet/minecraft/core/Holder$Reference;
public static final FLUID_PICKUP : Lnet/minecraft/core/Holder$Reference;
public static final FLUID_PLACE : Lnet/minecraft/core/Holder$Reference;
public static final HIT_GROUND : Lnet/minecraft/core/Holder$Reference;
public static final INSTRUMENT_PLAY : Lnet/minecraft/core/Holder$Reference;
public static final ITEM_INTERACT_FINISH : Lnet/minecraft/core/Holder$Reference;
public static final ITEM_INTERACT_START : Lnet/minecraft/core/Holder$Reference;
public static final JUKEBOX_PLAY : Lnet/minecraft/core/Holder$Reference;
public static final JUKEBOX_STOP_PLAY : Lnet/minecraft/core/Holder$Reference;
public static final LIGHTNING_STRIKE : Lnet/minecraft/core/Holder$Reference;
public static final NOTE_BLOCK_PLAY : Lnet/minecraft/core/Holder$Reference;
public static final PRIME_FUSE : Lnet/minecraft/core/Holder$Reference;
public static final PROJECTILE_LAND : Lnet/minecraft/core/Holder$Reference;
public static final PROJECTILE_SHOOT : Lnet/minecraft/core/Holder$Reference;
public static final SCULK_SENSOR_TENDRILS_CLICKING : Lnet/minecraft/core/Holder$Reference;
public static final SHEAR : Lnet/minecraft/core/Holder$Reference;
public static final SHRIEK : Lnet/minecraft/core/Holder$Reference;
public static final SPLASH : Lnet/minecraft/core/Holder$Reference;
public static final STEP : Lnet/minecraft/core/Holder$Reference;
public static final SWIM : Lnet/minecraft/core/Holder$Reference;
public static final TELEPORT : Lnet/minecraft/core/Holder$Reference;
public static final UNEQUIP : Lnet/minecraft/core/Holder$Reference;
public static final RESONATE_1 : Lnet/minecraft/core/Holder$Reference;
public static final RESONATE_2 : Lnet/minecraft/core/Holder$Reference;
public static final RESONATE_3 : Lnet/minecraft/core/Holder$Reference;
public static final RESONATE_4 : Lnet/minecraft/core/Holder$Reference;
public static final RESONATE_5 : Lnet/minecraft/core/Holder$Reference;
public static final RESONATE_6 : Lnet/minecraft/core/Holder$Reference;
public static final RESONATE_7 : Lnet/minecraft/core/Holder$Reference;
public static final RESONATE_8 : Lnet/minecraft/core/Holder$Reference;
public static final RESONATE_9 : Lnet/minecraft/core/Holder$Reference;
public static final RESONATE_10 : Lnet/minecraft/core/Holder$Reference;
public static final RESONATE_11 : Lnet/minecraft/core/Holder$Reference;
public static final RESONATE_12 : Lnet/minecraft/core/Holder$Reference;
public static final RESONATE_13 : Lnet/minecraft/core/Holder$Reference;
public static final RESONATE_14 : Lnet/minecraft/core/Holder$Reference;
public static final RESONATE_15 : Lnet/minecraft/core/Holder$Reference;
public static final DEFAULT_NOTIFICATION_RADIUS : I
public static final CODEC : Lcom/mojang/serialization/Codec;
public <init>(I)V
public static bootstrap(Lnet/minecraft/core/Registry;)Lnet/minecraft/core/Holder;
public notificationRadius()I
private static register(Ljava/lang/String;)Lnet/minecraft/core/Holder$Reference;
private static register(Ljava/lang/String;I)Lnet/minecraft/core/Holder$Reference;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
static <clinit>()V
```
