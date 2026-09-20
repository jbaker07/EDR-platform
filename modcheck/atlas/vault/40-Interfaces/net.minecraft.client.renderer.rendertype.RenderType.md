---
type: "interface"
fqcn: "net.minecraft.client.renderer.rendertype.RenderType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.rendertype.RenderType

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `hasBlending()Z` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `hasBlending()Z` | `` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `isOutline()Z` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `outline()Ljava/util/Optional;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (25, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.rendertype.RenderType {
    private static final int MEGABYTE;
    public static final int BIG_BUFFER_SIZE;
    public static final int SMALL_BUFFER_SIZE;
    public static final int TRANSIENT_BUFFER_SIZE;
    private final net.minecraft.client.renderer.rendertype.RenderSetup state;
    private final boolean hasBlending;
    private final java.util.Optional<net.minecraft.client.renderer.rendertype.RenderType> outline;
    protected final java.lang.String name;
    private net.minecraft.client.renderer.rendertype.RenderType(java.lang.String, net.minecraft.client.renderer.rendertype.RenderSetup);
    static net.minecraft.client.renderer.rendertype.RenderType create(java.lang.String, net.minecraft.client.renderer.rendertype.RenderSetup);
    public java.lang.String toString();
    public boolean hasBlending();
    private boolean calculateHasBlending();
    public net.minecraft.client.renderer.rendertype.PreparedRenderType prepare();
    private com.mojang.renderpearl.api.buffers.GpuBufferSlice writeDynamicTransforms(org.joml.Matrix4f);
    public com.mojang.renderpearl.api.vertex.VertexFormat format();
    public com.mojang.renderpearl.api.pipeline.PrimitiveTopology primitiveTopology();
    public java.util.Optional<net.minecraft.client.renderer.rendertype.RenderType> outline();
    public boolean isOutline();
    public com.mojang.renderpearl.api.pipeline.RenderPipeline pipeline();
    public boolean affectsCrumbling();
    public boolean canConsolidateConsecutiveGeometry();
    public boolean sortOnUpload();
    public boolean forceSolidModelPhase();
    private static net.minecraft.client.renderer.rendertype.RenderType lambda$new$0(net.minecraft.client.renderer.rendertype.RenderSetup, net.minecraft.client.renderer.rendertype.RenderSetup$TextureBinding);
}
```
