---
type: "interface"
fqcn: "net.minecraft.world.entity.EntitySpawnReason"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.EntitySpawnReason

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

`enum` public final; extends `java/lang/Enum`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `LOAD` | `Lnet/minecraft/world/entity/EntitySpawnReason;` | exact | getstatic@33 in `EntityApiLookupImpl.lambda$checkSelfImplementingTypes$0` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |

## Declared members (20 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final NATURAL : Lnet/minecraft/world/entity/EntitySpawnReason;
public static final CHUNK_GENERATION : Lnet/minecraft/world/entity/EntitySpawnReason;
public static final SPAWNER : Lnet/minecraft/world/entity/EntitySpawnReason;
public static final STRUCTURE : Lnet/minecraft/world/entity/EntitySpawnReason;
public static final BREEDING : Lnet/minecraft/world/entity/EntitySpawnReason;
public static final MOB_SUMMONED : Lnet/minecraft/world/entity/EntitySpawnReason;
public static final JOCKEY : Lnet/minecraft/world/entity/EntitySpawnReason;
public static final EVENT : Lnet/minecraft/world/entity/EntitySpawnReason;
public static final CONVERSION : Lnet/minecraft/world/entity/EntitySpawnReason;
public static final REINFORCEMENT : Lnet/minecraft/world/entity/EntitySpawnReason;
public static final TRIGGERED : Lnet/minecraft/world/entity/EntitySpawnReason;
public static final BUCKET : Lnet/minecraft/world/entity/EntitySpawnReason;
public static final SPAWN_ITEM_USE : Lnet/minecraft/world/entity/EntitySpawnReason;
public static final COMMAND : Lnet/minecraft/world/entity/EntitySpawnReason;
public static final DISPENSER : Lnet/minecraft/world/entity/EntitySpawnReason;
public static final PATROL : Lnet/minecraft/world/entity/EntitySpawnReason;
public static final TRIAL_SPAWNER : Lnet/minecraft/world/entity/EntitySpawnReason;
public static final LOAD : Lnet/minecraft/world/entity/EntitySpawnReason;
public static final DIMENSION_TRAVEL : Lnet/minecraft/world/entity/EntitySpawnReason;
private static final synthetic $VALUES : [Lnet/minecraft/world/entity/EntitySpawnReason;
public static values()[Lnet/minecraft/world/entity/EntitySpawnReason;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/world/entity/EntitySpawnReason;
private <init>(Ljava/lang/String;I)V
public static isSpawner(Lnet/minecraft/world/entity/EntitySpawnReason;)Z
public static ignoresLightRequirements(Lnet/minecraft/world/entity/EntitySpawnReason;)Z
private static synthetic $values()[Lnet/minecraft/world/entity/EntitySpawnReason;
static <clinit>()V
```
