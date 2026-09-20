---
type: "interface"
fqcn: "net.minecraft.client.renderer.texture.OverlayTexture"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.texture.OverlayTexture

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| reads | `NO_OVERLAYI` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `NO_OVERLAYI` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (14, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.texture.OverlayTexture implements java.lang.AutoCloseable {
    private static final int SIZE;
    public static final int NO_WHITE_U;
    public static final int RED_OVERLAY_V;
    public static final int WHITE_OVERLAY_V;
    public static final int NO_OVERLAY;
    private final net.minecraft.client.renderer.texture.DynamicTexture texture;
    public net.minecraft.client.renderer.texture.OverlayTexture();
    public void close();
    public static int u(float);
    public static int v(boolean);
    public static int pack(int, int);
    public static int pack(float, boolean);
    public com.mojang.renderpearl.api.textures.GpuTextureView getTextureView();
    static {};
}
```
