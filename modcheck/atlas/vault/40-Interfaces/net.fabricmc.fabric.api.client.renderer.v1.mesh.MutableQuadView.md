---
type: "interface"
fqcn: "net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView"
module: "fabric-renderer-api-v1"
sha256: "2e4aaeb20f8615e8176f30bc82616cee25f8727323bfef7a70d5a6555ae8e9ee"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView

Module: [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] -- kind: interface

```java
public static final int BAKE_ROTATE_NONE
public static final int BAKE_ROTATE_90
public static final int BAKE_ROTATE_180
public static final int BAKE_ROTATE_270
public static final int BAKE_LOCK_UV
public static final int BAKE_FLIP_U
public static final int BAKE_FLIP_V
public static final int BAKE_NORMALIZED
public static final float CULL_FACE_EPSILON
public abstract net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView pos(int, float, float, float)
public default net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView pos(int, org.joml.Vector3f)
public default net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView pos(int, org.joml.Vector3fc)
public default net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView translate(float, float, float)
public abstract net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView color(int, int)
public default net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView color(int, int, int, int)
public default net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView multiplyColor(int)
public abstract net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView uv(int, float, float)
public default net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView uv(int, org.joml.Vector2f)
public default net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView uv(int, org.joml.Vector2fc)
public default net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView uvUnitSquare()
public default net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView materialBake(net.minecraft.client.resources.model.sprite.Material$Baked, int)
public default net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView postMaterialBake(net.minecraft.client.resources.model.sprite.Material$Baked)
public abstract net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView lightmap(int, int)
public default net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView lightmap(int, int, int, int)
public default net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView minLightmap(int)
public abstract net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView normal(int, float, float, float)
public default net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView normal(int, org.joml.Vector3f)
public default net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView normal(int, org.joml.Vector3fc)
public abstract net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView nominalFace(net.minecraft.core.Direction)
public abstract net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView cullFace(net.minecraft.core.Direction)
public abstract net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView atlas(net.fabricmc.fabric.api.client.renderer.v1.mesh.QuadAtlas)
public abstract net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView chunkLayer(net.minecraft.client.renderer.chunk.ChunkSectionLayer)
public abstract net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView itemRenderType(net.minecraft.client.renderer.rendertype.RenderType)
public abstract net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView itemGlintRenderType(net.minecraft.client.renderer.rendertype.RenderType)
public abstract net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView itemGlintSpecialRenderType(net.minecraft.client.renderer.rendertype.RenderType)
public abstract net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView emissive(boolean)
public abstract net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView shadeDirectionOverride(net.minecraft.core.Direction)
public abstract net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView ambientOcclusion(net.fabricmc.fabric.api.util.TriState)
public abstract net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView foilType(net.minecraft.client.renderer.item.ItemStackRenderState$FoilType)
public abstract net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView shadeMode(net.fabricmc.fabric.api.client.renderer.v1.mesh.ShadeMode)
public abstract net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView animated(boolean)
public abstract net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView tintIndex(int)
public abstract net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView tag(int)
public abstract net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView copyFrom(net.fabricmc.fabric.api.client.renderer.v1.mesh.QuadView)
public abstract net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView fromBakedQuad(net.minecraft.client.resources.model.geometry.BakedQuad)
public abstract net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView clear()
public default net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView square(net.minecraft.core.Direction, float, float, float, float, float)
```
