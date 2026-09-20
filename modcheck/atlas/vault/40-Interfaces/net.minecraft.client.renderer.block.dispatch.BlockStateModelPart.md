---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.dispatch.BlockStateModelPart"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.dispatch.BlockStateModelPart

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `emitQuads(Lnet/fabricmc/fabric/api/client/renderer/v1/mesh/QuadEmitte` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `emitQuads(Lnet/fabricmc/fabric/api/client/renderer/v1/mesh/QuadEmitte` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getQuads(Lnet/minecraft/core/Direction;)Ljava/util/List;` | `` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getQuads(Lnet/minecraft/core/Direction;)Ljava/util/List;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `useAmbientOcclusion()Z` | `` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (4, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.client.renderer.block.dispatch.BlockStateModelPart {
    public abstract java.util.List<net.minecraft.client.resources.model.geometry.BakedQuad> getQuads(net.minecraft.core.Direction);
    public abstract boolean useAmbientOcclusion();
    public abstract net.minecraft.client.resources.model.sprite.Material$Baked particleMaterial();
    public abstract int materialFlags();
}
```
