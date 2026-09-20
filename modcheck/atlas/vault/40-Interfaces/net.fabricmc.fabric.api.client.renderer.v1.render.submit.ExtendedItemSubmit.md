---
type: "interface"
fqcn: "net.fabricmc.fabric.api.client.renderer.v1.render.submit.ExtendedItemSubmit"
module: "fabric-renderer-api-v1"
sha256: "2e4aaeb20f8615e8176f30bc82616cee25f8727323bfef7a70d5a6555ae8e9ee"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.renderer.v1.render.submit.ExtendedItemSubmit

Module: [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] -- kind: record

```java
public static final net.minecraft.client.renderer.feature.FeatureRendererType TYPE
public <init>(com.mojang.blaze3d.vertex.PoseStack$Pose, net.minecraft.world.item.ItemDisplayContext, int, int, int, int[], java.util.List, net.fabricmc.fabric.api.client.renderer.v1.mesh.MeshView, net.minecraft.client.renderer.item.ItemStackRenderState$FoilType)
public float distanceToCameraSq()
public net.minecraft.client.renderer.feature.FeatureRendererType featureType()
public final java.lang.String toString()
public final int hashCode()
public final boolean equals(java.lang.Object)
public com.mojang.blaze3d.vertex.PoseStack$Pose pose()
public net.minecraft.world.item.ItemDisplayContext displayContext()
public int lightCoords()
public int overlayCoords()
public int outlineColor()
public int[] tintLayers()
public java.util.List quads()
public net.fabricmc.fabric.api.client.renderer.v1.mesh.MeshView mesh()
public net.minecraft.client.renderer.item.ItemStackRenderState$FoilType foilType()
```
