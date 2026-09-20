---
type: "interface"
fqcn: "net.minecraft.client.renderer.Panorama"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.Panorama

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `holdSpin()V` | `` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (8, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.Panorama {
    public static final net.minecraft.resources.Identifier PANORAMA_OVERLAY;
    private float spin;
    private boolean shouldSpin;
    public net.minecraft.client.renderer.Panorama();
    public void startSpin();
    public void holdSpin();
    public void extractRenderState(net.minecraft.client.gui.GuiGraphicsExtractor, int, int);
    static {};
}
```
