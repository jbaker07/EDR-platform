---
type: "interface"
fqcn: "net.fabricmc.fabric.api.client.renderer.v1.mesh.QuadView"
module: "fabric-renderer-api-v1"
sha256: "2e4aaeb20f8615e8176f30bc82616cee25f8727323bfef7a70d5a6555ae8e9ee"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.renderer.v1.mesh.QuadView

Module: [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] -- kind: interface

```java
public abstract float x(int)
public abstract float y(int)
public abstract float z(int)
public abstract float posByIndex(int, int)
public abstract org.joml.Vector3f copyPos(int, org.joml.Vector3f)
public abstract int color(int)
public abstract float u(int)
public abstract float v(int)
public abstract org.joml.Vector2f copyUv(int, org.joml.Vector2f)
public abstract int lightmap(int)
public abstract boolean hasNormal(int)
public abstract float normalX(int)
public abstract float normalY(int)
public abstract float normalZ(int)
public abstract org.joml.Vector3f copyNormal(int, org.joml.Vector3f)
public abstract org.joml.Vector3fc faceNormal()
public abstract net.minecraft.core.Direction lightFace()
public abstract net.minecraft.core.Direction nominalFace()
public abstract net.minecraft.core.Direction cullFace()
public abstract net.fabricmc.fabric.api.client.renderer.v1.mesh.QuadAtlas atlas()
public abstract net.minecraft.client.renderer.chunk.ChunkSectionLayer chunkLayer()
public abstract net.minecraft.client.renderer.rendertype.RenderType itemRenderType()
public abstract net.minecraft.client.renderer.rendertype.RenderType itemGlintRenderType()
public abstract net.minecraft.client.renderer.rendertype.RenderType itemGlintSpecialRenderType()
public abstract boolean emissive()
public abstract net.minecraft.core.Direction shadeDirectionOverride()
public abstract net.fabricmc.fabric.api.util.TriState ambientOcclusion()
public abstract net.minecraft.client.renderer.item.ItemStackRenderState$FoilType foilType()
public abstract net.fabricmc.fabric.api.client.renderer.v1.mesh.ShadeMode shadeMode()
public abstract boolean animated()
public abstract int tintIndex()
public abstract int tag()
public net.minecraft.client.resources.model.geometry.BakedQuad toBakedQuad(net.minecraft.client.renderer.texture.TextureAtlasSprite)
public abstract void buffer(int, com.mojang.blaze3d.vertex.VertexConsumer)
public abstract void buffer(int, com.mojang.blaze3d.vertex.PoseStack$Pose, com.mojang.blaze3d.vertex.VertexConsumer)
```
