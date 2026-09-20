---
type: "interface"
fqcn: "net.minecraft.client.renderer.feature.MovingBlockFeatureRenderer$Submit"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.feature.MovingBlockFeatureRenderer$Submit

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `forceTranslucent()Z` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `outlineColor()I` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (14, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.client.renderer.feature.MovingBlockFeatureRenderer$Submit extends java.lang.Record implements net.minecraft.client.renderer.feature.submit.TranslucentSubmit {
    private final org.joml.Matrix4fc pose;
    private final net.minecraft.client.renderer.block.MovingBlockRenderState movingBlockRenderState;
    private final int outlineColor;
    private final boolean forceTranslucent;
    public net.minecraft.client.renderer.feature.MovingBlockFeatureRenderer$Submit(org.joml.Matrix4fc, net.minecraft.client.renderer.block.MovingBlockRenderState, int, boolean);
    public float distanceToCameraSq();
    public net.minecraft.client.renderer.feature.FeatureRendererType<net.minecraft.client.renderer.feature.MovingBlockFeatureRenderer$Submit> featureType();
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public org.joml.Matrix4fc pose();
    public net.minecraft.client.renderer.block.MovingBlockRenderState movingBlockRenderState();
    public int outlineColor();
    public boolean forceTranslucent();
}
```
