---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.BlockStateModelSet"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.BlockStateModelSet

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getParticleMaterial(Lnet/minecraft/world/level/block/state/BlockState;Lnet/mine` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getParticleMaterial(Lnet/minecraft/world/level/block/state/BlockState;Lnet/mine` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getParticleMaterial(Lnet/minecraft/world/level/block/state/BlockState;Lnet/mine` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (6, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.block.BlockStateModelSet {
    private final java.util.Map<net.minecraft.world.level.block.state.BlockState, net.minecraft.client.renderer.block.dispatch.BlockStateModel> modelByState;
    private final net.minecraft.client.renderer.block.dispatch.BlockStateModel missingModel;
    public net.minecraft.client.renderer.block.BlockStateModelSet(java.util.Map<net.minecraft.world.level.block.state.BlockState, net.minecraft.client.renderer.block.dispatch.BlockStateModel>, net.minecraft.client.renderer.block.dispatch.BlockStateModel);
    public net.minecraft.client.renderer.block.dispatch.BlockStateModel get(net.minecraft.world.level.block.state.BlockState);
    public net.minecraft.client.renderer.block.dispatch.BlockStateModel missingModel();
    public net.minecraft.client.resources.model.sprite.Material$Baked getParticleMaterial(net.minecraft.world.level.block.state.BlockState);
}
```
