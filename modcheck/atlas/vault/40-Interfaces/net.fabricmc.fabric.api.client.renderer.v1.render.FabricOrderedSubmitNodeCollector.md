---
type: "interface"
fqcn: "net.fabricmc.fabric.api.client.renderer.v1.render.FabricOrderedSubmitNodeCollector"
module: "fabric-renderer-api-v1"
sha256: "2e4aaeb20f8615e8176f30bc82616cee25f8727323bfef7a70d5a6555ae8e9ee"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.renderer.v1.render.FabricOrderedSubmitNodeCollector

Module: [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] -- kind: interface

```java
public void submitBlockModel(com.mojang.blaze3d.vertex.PoseStack, java.util.function.Function, boolean, java.util.List, net.fabricmc.fabric.api.client.renderer.v1.mesh.Mesh, int[], int, int, int)
public void submitBreakingBlockModel(com.mojang.blaze3d.vertex.PoseStack, java.util.List, net.fabricmc.fabric.api.client.renderer.v1.mesh.Mesh, int, boolean)
public void submitItem(com.mojang.blaze3d.vertex.PoseStack, net.minecraft.world.item.ItemDisplayContext, int, int, int, int[], net.minecraft.client.resources.model.geometry.ItemQuads, net.fabricmc.fabric.api.client.renderer.v1.mesh.MeshView, net.minecraft.client.renderer.item.ItemStackRenderState$FoilType)
```
