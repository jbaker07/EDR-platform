---
type: "interface"
fqcn: "net.minecraft.client.renderer.chunk.ChunkSectionLayer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.chunk.ChunkSectionLayer

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `ordinal()I` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `values()[Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `SOLIDLnet/minecraft/client/renderer/chunk/ChunkSectionLayer;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (20, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.client.renderer.chunk.ChunkSectionLayer extends java.lang.Enum<net.minecraft.client.renderer.chunk.ChunkSectionLayer> {
    public static final net.minecraft.client.renderer.chunk.ChunkSectionLayer SOLID;
    public static final net.minecraft.client.renderer.chunk.ChunkSectionLayer CUTOUT;
    public static final net.minecraft.client.renderer.chunk.ChunkSectionLayer TRANSLUCENT;
    private final com.mojang.renderpearl.api.pipeline.RenderPipeline pipeline;
    private final com.mojang.renderpearl.api.pipeline.RenderPipeline multiDrawPipeline;
    private final int bufferSize;
    private final boolean translucent;
    private final java.lang.String label;
    private static final net.minecraft.client.renderer.chunk.ChunkSectionLayer[] $VALUES;
    public static net.minecraft.client.renderer.chunk.ChunkSectionLayer[] values();
    public static net.minecraft.client.renderer.chunk.ChunkSectionLayer valueOf(java.lang.String);
    private net.minecraft.client.renderer.chunk.ChunkSectionLayer(com.mojang.renderpearl.api.pipeline.RenderPipeline, com.mojang.renderpearl.api.pipeline.RenderPipeline, int, boolean);
    public static net.minecraft.client.renderer.chunk.ChunkSectionLayer byTransparency(com.mojang.blaze3d.platform.Transparency);
    public com.mojang.renderpearl.api.pipeline.RenderPipeline pipeline(boolean);
    public int bufferSize();
    public java.lang.String label();
    public boolean translucent();
    public com.mojang.renderpearl.api.vertex.VertexFormat vertexFormat();
    private static net.minecraft.client.renderer.chunk.ChunkSectionLayer[] $values();
    static {};
}
```
