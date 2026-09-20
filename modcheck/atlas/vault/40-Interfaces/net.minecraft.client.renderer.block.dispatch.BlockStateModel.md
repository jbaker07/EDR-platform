---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.dispatch.BlockStateModel"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.dispatch.BlockStateModel

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `collectParts(Lnet/minecraft/util/RandomSource;Ljava/util/List;)V` | `` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `createGeometryKey(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lne` | `` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `createGeometryKey(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lne` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `createGeometryKey(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lne` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `emitQuads(Lnet/fabricmc/fabric/api/client/renderer/v1/mesh/QuadEmitte` | `` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `emitQuads(Lnet/fabricmc/fabric/api/client/renderer/v1/mesh/QuadEmitte` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `emitQuads(Lnet/fabricmc/fabric/api/client/renderer/v1/mesh/QuadEmitte` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `emitQuads(Lnet/fabricmc/fabric/api/client/renderer/v1/mesh/QuadEmitte` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `emitQuads(Lnet/fabricmc/fabric/api/client/renderer/v1/mesh/QuadEmitte` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `emitQuads(Lnet/fabricmc/fabric/api/client/renderer/v1/mesh/QuadEmitte` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `hasMaterialFlag(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lne` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `hasMaterialFlag(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lne` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `hasMaterialFlag(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lne` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `materialFlags()I` | `` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `materialFlags(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lne` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `materialFlags(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lne` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `particleMaterial()Lnet/minecraft/client/resources/model/sprite/Material$Bake` | `` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `particleMaterial(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lne` | `` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `particleMaterial(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lne` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `particleMaterial(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lne` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (4, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.client.renderer.block.dispatch.BlockStateModel {
    public abstract void collectParts(net.minecraft.util.RandomSource, java.util.List<net.minecraft.client.renderer.block.dispatch.BlockStateModelPart>);
    public abstract net.minecraft.client.resources.model.sprite.Material$Baked particleMaterial();
    public abstract int materialFlags();
    public default boolean hasMaterialFlag(int);
}
```
