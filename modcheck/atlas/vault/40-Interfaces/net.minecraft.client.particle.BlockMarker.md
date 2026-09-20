---
type: "interface"
fqcn: "net.minecraft.client.particle.BlockMarker"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.particle.BlockMarker

System: [[20-Systems/net.minecraft.client.particle|net.minecraft.client.particle]]

`class` public; extends `net/minecraft/client/particle/SingleQuadParticle`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| wraps | `<init>` | `(Lnet/minecraft/client/multiplayer/ClientLevel;DDDLnet/minecraft/world` | exact | @Redirect at ['INVOKE'] | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (1 fields, 3 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final layer : Lnet/minecraft/client/particle/SingleQuadParticle$Layer;
private <init>(Lnet/minecraft/client/multiplayer/ClientLevel;DDDLnet/minecraft/world/level/block/state/BlockState;)V
public getLayer()Lnet/minecraft/client/particle/SingleQuadParticle$Layer;
public getQuadSize(F)F
```
