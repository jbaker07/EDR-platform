---
type: "interface"
fqcn: "net.minecraft.client.particle.TerrainParticle"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.particle.TerrainParticle

System: [[20-Systems/net.minecraft.client.particle|net.minecraft.client.particle]]

`class` public; extends `net/minecraft/client/particle/SingleQuadParticle`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/client/multiplayer/ClientLevel;DDDDDDLnet/minecraft/wo` | exact | invokespecial@52 in `TerrainParticleMixin.constructTerrainParticle` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/client/multiplayer/ClientLevel;DDDDDDLnet/minecraft/wo` | exact | invokespecial@31 in `TerrainParticleMixin.constructTerrainParticle` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| reads | `pos` | `Lnet/minecraft/core/BlockPos;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | declared |
| wraps | `<init>` | `(Lnet/minecraft/client/multiplayer/ClientLevel;DDDDDDLnet/minecraft/wo` | exact | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| wraps | `<init>` | `(Lnet/minecraft/client/multiplayer/ClientLevel;DDDDDDLnet/minecraft/wo` | exact | @Redirect at ['INVOKE'] | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| wraps | `createTerrainParticle` | `(Lnet/minecraft/core/particles/BlockParticleOption;Lnet/minecraft/clie` | name_only | @Redirect at ['NEW'] | client | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |

## Declared members (4 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final layer : Lnet/minecraft/client/particle/SingleQuadParticle$Layer;
private final pos : Lnet/minecraft/core/BlockPos;
private final uo : F
private final vo : F
public <init>(Lnet/minecraft/client/multiplayer/ClientLevel;DDDDDDLnet/minecraft/world/level/block/state/BlockState;)V
public <init>(Lnet/minecraft/client/multiplayer/ClientLevel;DDDDDDLnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;)V
public getLayer()Lnet/minecraft/client/particle/SingleQuadParticle$Layer;
protected getU0()F
protected getU1()F
protected getV0()F
protected getV1()F
private static createTerrainParticle(Lnet/minecraft/core/particles/BlockParticleOption;Lnet/minecraft/client/multiplayer/ClientLevel;DDDDDD)Lnet/minecraft/client/particle/TerrainParticle;
```
