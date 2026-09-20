---
type: "interface"
fqcn: "net.minecraft.client.renderer.item.ItemStackRenderState$LayerRenderState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.item.ItemStackRenderState$LayerRenderState

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `emitter()Lnet/fabricmc/fabric/api/client/renderer/v1/mesh/QuadEmitt` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| injects_into | `clear` | `@Inject at TAIL` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `clear()V` | `@Inject at RETURN` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `EMPTY_TINTS[I` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| wraps | `submit` | `@Redirect at INVOKE Lnet/minecraft/client/renderer/SubmitNodeCollector;submitIte` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (30, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.item.ItemStackRenderState$LayerRenderState {
    private static final org.joml.Vector3fc[] NO_EXTENTS;
    public static final java.util.function.Supplier<org.joml.Vector3fc[]> NO_EXTENTS_SUPPLIER;
    public static final int[] EMPTY_TINTS;
    private net.minecraft.client.resources.model.geometry.ItemQuads quads;
    private boolean usesBlockLight;
    private net.minecraft.client.resources.model.sprite.Material$Baked particleMaterial;
    private net.minecraft.client.resources.model.cuboid.ItemTransform itemTransform;
    private final org.joml.Matrix4f localTransform;
    private net.minecraft.client.renderer.item.ItemStackRenderState$FoilType foilType;
    private it.unimi.dsi.fastutil.ints.IntList tintLayers;
    private net.minecraft.client.renderer.special.SpecialModelRenderer<java.lang.Object> specialRenderer;
    private java.lang.Object argumentForSpecialRendering;
    private java.util.function.Supplier<org.joml.Vector3fc[]> extents;
    final net.minecraft.client.renderer.item.ItemStackRenderState this$0;
    public net.minecraft.client.renderer.item.ItemStackRenderState$LayerRenderState(net.minecraft.client.renderer.item.ItemStackRenderState);
    public void clear();
    public void setQuads(net.minecraft.client.resources.model.geometry.ItemQuads);
    public void setUsesBlockLight(boolean);
    public void setExtents(java.util.function.Supplier<org.joml.Vector3fc[]>);
    public void setParticleMaterial(net.minecraft.client.resources.model.sprite.Material$Baked);
    public void setItemTransform(net.minecraft.client.resources.model.cuboid.ItemTransform);
    public void setLocalTransform(org.joml.Matrix4fc);
    public <T> void setupSpecialModel(net.minecraft.client.renderer.special.SpecialModelRenderer<T>, T);
    private static net.minecraft.client.renderer.special.SpecialModelRenderer<java.lang.Object> eraseSpecialRenderer(net.minecraft.client.renderer.special.SpecialModelRenderer<?>);
    public void setFoilType(net.minecraft.client.renderer.item.ItemStackRenderState$FoilType);
    public it.unimi.dsi.fastutil.ints.IntList tintLayers();
    private void submit(com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.SubmitNodeCollector, int, int, int);
    private void applyTransform(com.mojang.blaze3d.vertex.PoseStack$Pose);
    private static org.joml.Vector3fc[] lambda$static$0();
    static {};
}
```
