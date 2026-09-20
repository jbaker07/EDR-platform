---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.BlockModelRenderState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.BlockModelRenderState

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `scratchRandomSource(J)Lnet/minecraft/util/RandomSource;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `setupMesh(Lorg/joml/Matrix4fc;Z)Lnet/fabricmc/fabric/api/client/rende` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| injects_into | `clear` | `@Inject at TAIL` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `clear()V` | `@Inject at RETURN` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| injects_into | `setupModel` | `@Inject at RETURN` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| injects_into | `submitModel` | `@Inject at HEAD` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `EMPTY_TINTS[I` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `EMPTY_TINTS[I` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (23, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.block.BlockModelRenderState {
    public static final int[] EMPTY_TINTS;
    private java.util.List<net.minecraft.client.renderer.block.dispatch.BlockStateModelPart> modelParts;
    private org.joml.Matrix4fc transformation;
    private net.minecraft.client.renderer.rendertype.RenderType renderType;
    private net.minecraft.client.renderer.special.SpecialModelRenderer<?> specialRenderer;
    private org.joml.Matrix4fc specialRendererTransformation;
    private it.unimi.dsi.fastutil.ints.IntList tintLayers;
    public int blockLightCoords;
    private net.minecraft.util.RandomSource randomSource;
    public net.minecraft.client.renderer.block.BlockModelRenderState();
    public void clear();
    public it.unimi.dsi.fastutil.ints.IntList tintLayers();
    public <T> void setupSpecialModel(net.minecraft.client.renderer.special.SpecialModelRenderer<T>, org.joml.Matrix4fc);
    public java.util.List<net.minecraft.client.renderer.block.dispatch.BlockStateModelPart> setupModel(org.joml.Matrix4fc, boolean);
    public void submit(com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.SubmitNodeCollector, int, int, int);
    private static org.joml.Matrix4fc identityToNull(org.joml.Matrix4fc);
    private void submitModel(net.minecraft.client.renderer.rendertype.RenderType, com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.SubmitNodeCollector, int, int, int);
    private static void submitSpecialRenderer(net.minecraft.client.renderer.special.SpecialModelRenderer<?>, com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.SubmitNodeCollector, int, int, int);
    public void submitOnlyOutline(com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.SubmitNodeCollector, int, int, int);
    public void submitWithZOffset(com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.SubmitNodeCollector, int, int, int);
    public boolean isEmpty();
    public net.minecraft.util.RandomSource scratchRandomSource(long);
    static {};
}
```
