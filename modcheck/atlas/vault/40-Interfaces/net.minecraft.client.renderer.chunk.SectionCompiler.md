---
type: "interface"
fqcn: "net.minecraft.client.renderer.chunk.SectionCompiler"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.chunk.SectionCompiler

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `compile` | `@Inject at INVOKE Lnet/minecraft/core/BlockPos;betweenClosed(Lnet/minecraft/core` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| wraps | `compile` | `@Redirect at INVOKE net/minecraft/client/renderer/block/ModelBlockRenderer.tesse` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (12, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.chunk.SectionCompiler {
    private final boolean ambientOcclusion;
    private final boolean cutoutLeaves;
    private final net.minecraft.client.renderer.block.BlockStateModelSet blockModelSet;
    private final net.minecraft.client.renderer.block.FluidStateModelSet fluidModelSet;
    private final net.minecraft.client.color.block.BlockColors blockColors;
    public net.minecraft.client.renderer.chunk.SectionCompiler(boolean, boolean, net.minecraft.client.renderer.block.BlockStateModelSet, net.minecraft.client.renderer.block.FluidStateModelSet, net.minecraft.client.color.block.BlockColors);
    public net.minecraft.client.renderer.chunk.SectionCompiler$Results compile(net.minecraft.core.SectionPos, net.minecraft.client.renderer.chunk.RenderSectionRegion, com.mojang.blaze3d.vertex.VertexSorting, net.minecraft.client.renderer.SectionBufferBuilderPack);
    private com.mojang.blaze3d.vertex.BufferBuilder getOrBeginLayer(java.util.Map<net.minecraft.client.renderer.chunk.ChunkSectionLayer, com.mojang.blaze3d.vertex.BufferBuilder>, net.minecraft.client.renderer.SectionBufferBuilderPack, net.minecraft.client.renderer.chunk.ChunkSectionLayer);
    private <E extends net.minecraft.world.level.block.entity.BlockEntity> void handleBlockEntity(net.minecraft.client.renderer.chunk.SectionCompiler$Results, E);
    private com.mojang.blaze3d.vertex.VertexConsumer lambda$compile$2(java.util.Map, net.minecraft.client.renderer.SectionBufferBuilderPack, net.minecraft.client.renderer.chunk.ChunkSectionLayer);
    private void lambda$compile$1(java.util.Map, net.minecraft.client.renderer.SectionBufferBuilderPack, float, float, float, net.minecraft.client.resources.model.geometry.BakedQuad, com.mojang.blaze3d.vertex.QuadInstance);
    private void lambda$compile$0(java.util.Map, net.minecraft.client.renderer.SectionBufferBuilderPack, float, float, float, net.minecraft.client.resources.model.geometry.BakedQuad, com.mojang.blaze3d.vertex.QuadInstance);
}
```
