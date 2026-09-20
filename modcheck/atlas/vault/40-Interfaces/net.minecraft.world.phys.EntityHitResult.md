---
type: "interface"
fqcn: "net.minecraft.world.phys.EntityHitResult"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.phys.EntityHitResult

System: [[20-Systems/net.minecraft.world.phys|net.minecraft.world.phys]]

`class` public; extends `net/minecraft/world/phys/HitResult`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/Vec3;)V` | exact | invokespecial@33 in `ServerGamePacketListenerImplMixin.handleInteract` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `getLocation` | `()Lnet/minecraft/world/phys/Vec3;` | inherited_exact | invokevirtual@52 in `MinecraftMixin.injectUseEntityCallback` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |

## Declared members (1 fields, 4 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final entity : Lnet/minecraft/world/entity/Entity;
public <init>(Lnet/minecraft/world/entity/Entity;)V
public <init>(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/Vec3;)V
public getEntity()Lnet/minecraft/world/entity/Entity;
public getType()Lnet/minecraft/world/phys/HitResult$Type;
```
