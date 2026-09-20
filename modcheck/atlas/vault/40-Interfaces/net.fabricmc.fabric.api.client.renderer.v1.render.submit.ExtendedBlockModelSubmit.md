---
type: "interface"
fqcn: "net.fabricmc.fabric.api.client.renderer.v1.render.submit.ExtendedBlockModelSubmit"
module: "fabric-renderer-api-v1"
sha256: "2e4aaeb20f8615e8176f30bc82616cee25f8727323bfef7a70d5a6555ae8e9ee"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.renderer.v1.render.submit.ExtendedBlockModelSubmit

Module: [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] -- kind: record

```java
public static final net.minecraft.client.renderer.feature.FeatureRendererType TYPE
public <init>(com.mojang.blaze3d.vertex.PoseStack$Pose, java.util.function.Function, java.util.List, net.fabricmc.fabric.api.client.renderer.v1.mesh.Mesh, int[], int, int, int, com.mojang.blaze3d.vertex.PoseStack$Pose)
public float distanceToCameraSq()
public net.minecraft.client.renderer.feature.FeatureRendererType featureType()
public final java.lang.String toString()
public final int hashCode()
public final boolean equals(java.lang.Object)
public com.mojang.blaze3d.vertex.PoseStack$Pose pose()
public java.util.function.Function renderTypeFunction()
public java.util.List modelParts()
public net.fabricmc.fabric.api.client.renderer.v1.mesh.Mesh mesh()
public int[] tintLayers()
public int lightCoords()
public int overlayCoords()
public int tintColor()
public com.mojang.blaze3d.vertex.PoseStack$Pose sheetedDecalPose()
```
