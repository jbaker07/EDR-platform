---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.dispatch.multipart.MultiPartModel$SharedBakedState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.dispatch.multipart.MultiPartModel$SharedBakedState

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `selectModels(Lnet/minecraft/world/level/block/state/BlockState;)Ljava/ut` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (9, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
final class net.minecraft.client.renderer.block.dispatch.multipart.MultiPartModel$SharedBakedState {
    private final java.util.List<net.minecraft.client.renderer.block.dispatch.multipart.MultiPartModel$Selector<net.minecraft.client.renderer.block.dispatch.BlockStateModel>> selectors;
    private final net.minecraft.client.resources.model.sprite.Material$Baked particleMaterial;
    private final int materialFlags;
    private final java.util.Map<java.util.BitSet, java.util.List<net.minecraft.client.renderer.block.dispatch.BlockStateModel>> subsets;
    private static net.minecraft.client.renderer.block.dispatch.BlockStateModel getFirstModel(java.util.List<net.minecraft.client.renderer.block.dispatch.multipart.MultiPartModel$Selector<net.minecraft.client.renderer.block.dispatch.BlockStateModel>>);
    private static int computeMaterialFlags(java.util.List<net.minecraft.client.renderer.block.dispatch.multipart.MultiPartModel$Selector<net.minecraft.client.renderer.block.dispatch.BlockStateModel>>);
    public net.minecraft.client.renderer.block.dispatch.multipart.MultiPartModel$SharedBakedState(java.util.List<net.minecraft.client.renderer.block.dispatch.multipart.MultiPartModel$Selector<net.minecraft.client.renderer.block.dispatch.BlockStateModel>>);
    public java.util.List<net.minecraft.client.renderer.block.dispatch.BlockStateModel> selectModels(net.minecraft.world.level.block.state.BlockState);
    private java.util.List lambda$selectModels$0(java.util.BitSet);
}
```
