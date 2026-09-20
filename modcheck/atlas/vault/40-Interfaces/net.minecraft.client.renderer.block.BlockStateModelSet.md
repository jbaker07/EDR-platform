---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.BlockStateModelSet"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.BlockStateModelSet

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements `net/fabricmc/fabric/api/client/renderer/v1/model/FabricBlockStateModelSet`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `get` | `(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/cli` | exact | invokevirtual@5 in `FabricBlockStateModelSet.getParticleMaterial` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getParticleMaterial` | `(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/clie` | inherited_exact | invokevirtual@11 in `BlockMarkerMixin.getParticleMaterialProxy` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getParticleMaterial` | `(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/clie` | inherited_exact | invokevirtual@34 in `LevelExtractorMixin.getParticleMaterialProxy` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getParticleMaterial` | `(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/clie` | inherited_exact | invokevirtual@4 in `TerrainParticleMixin.getParticleIconProxy` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (2 fields, 4 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final modelByState : Ljava/util/Map;
private final missingModel : Lnet/minecraft/client/renderer/block/dispatch/BlockStateModel;
public <init>(Ljava/util/Map;Lnet/minecraft/client/renderer/block/dispatch/BlockStateModel;)V
public get(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/client/renderer/block/dispatch/BlockStateModel;
public missingModel()Lnet/minecraft/client/renderer/block/dispatch/BlockStateModel;
public getParticleMaterial(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/client/resources/model/sprite/Material$Baked;
```
