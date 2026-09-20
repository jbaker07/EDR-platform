---
type: "interface"
fqcn: "net.minecraft.world.level.storage.loot.parameters.LootContextParams"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.loot.parameters.LootContextParams

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `BLOCK_STATE` | `Lnet/minecraft/util/context/ContextKey;` | exact | getstatic@12 in `ComposterWrapper.getLayersToAdd` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `ORIGIN` | `Lnet/minecraft/util/context/ContextKey;` | exact | getstatic@19 in `ComposterWrapper.getLayersToAdd` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (16 fields, 2 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final THIS_ENTITY : Lnet/minecraft/util/context/ContextKey;
public static final INTERACTING_ENTITY : Lnet/minecraft/util/context/ContextKey;
public static final TARGET_ENTITY : Lnet/minecraft/util/context/ContextKey;
public static final LAST_DAMAGE_PLAYER : Lnet/minecraft/util/context/ContextKey;
public static final DAMAGE_SOURCE : Lnet/minecraft/util/context/ContextKey;
public static final ATTACKING_ENTITY : Lnet/minecraft/util/context/ContextKey;
public static final DIRECT_ATTACKING_ENTITY : Lnet/minecraft/util/context/ContextKey;
public static final ORIGIN : Lnet/minecraft/util/context/ContextKey;
public static final BLOCK_STATE : Lnet/minecraft/util/context/ContextKey;
public static final BLOCK_ENTITY : Lnet/minecraft/util/context/ContextKey;
public static final TOOL : Lnet/minecraft/util/context/ContextKey;
public static final EXPLOSION_RADIUS : Lnet/minecraft/util/context/ContextKey;
public static final ENCHANTMENT_LEVEL : Lnet/minecraft/util/context/ContextKey;
public static final ENCHANTMENT_ACTIVE : Lnet/minecraft/util/context/ContextKey;
public static final ADDITIONAL_COST_COMPONENT_ALLOWED : Lnet/minecraft/util/context/ContextKey;
public static final CONTAINER : Lnet/minecraft/util/context/ContextKey;
public <init>()V
static <clinit>()V
```
