---
type: "interface"
fqcn: "net.minecraft.client.renderer.item.ItemStackRenderState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.item.ItemStackRenderState

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `clear` | `@Inject at TAIL` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `visitExtents(Ljava/util/function/Consumer;)V` | `@Inject at NEW com/mojang/blaze3d/vertex/PoseStack$Pose` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| injects_into | `visitExtents(Ljava/util/function/Consumer;)V` | `@Inject at INVOKE Lcom/mojang/blaze3d/vertex/PoseStack$Pose;setIdentity()V` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (22, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.item.ItemStackRenderState {
    net.minecraft.world.item.ItemDisplayContext displayContext;
    private int activeLayerCount;
    private boolean animated;
    private boolean oversizedInGui;
    private net.minecraft.world.phys.AABB cachedModelBoundingBox;
    private net.minecraft.client.renderer.item.ItemStackRenderState$LayerRenderState[] layers;
    public net.minecraft.client.renderer.item.ItemStackRenderState();
    public void ensureCapacity(int);
    public net.minecraft.client.renderer.item.ItemStackRenderState$LayerRenderState newLayer();
    public void clear();
    public void setAnimated();
    public boolean isAnimated();
    public void appendModelIdentityElement(java.lang.Object);
    private net.minecraft.client.renderer.item.ItemStackRenderState$LayerRenderState firstLayer();
    public boolean isEmpty();
    public boolean usesBlockLight();
    public net.minecraft.client.resources.model.sprite.Material$Baked pickParticleMaterial(net.minecraft.util.RandomSource);
    public void visitExtents(java.util.function.Consumer<org.joml.Vector3fc>);
    public void submit(com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.SubmitNodeCollector, int, int, int);
    public net.minecraft.world.phys.AABB getModelBoundingBox();
    public void setOversizedInGui(boolean);
    public boolean isOversizedInGui();
}
```
