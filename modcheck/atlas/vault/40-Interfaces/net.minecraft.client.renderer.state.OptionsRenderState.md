---
type: "interface"
fqcn: "net.minecraft.client.renderer.state.OptionsRenderState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.state.OptionsRenderState

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| reads | `ambientOcclusionZ` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (23, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.state.OptionsRenderState {
    public int cloudRange;
    public boolean cutoutLeaves;
    public boolean improvedTransparency;
    public boolean ambientOcclusion;
    public int menuBackgroundBlurriness;
    public double panoramaSpeed;
    public int maxAnisotropyValue;
    public net.minecraft.client.TextureFilteringMethod textureFiltering;
    public boolean bobView;
    public float screenEffectScale;
    public double glintSpeed;
    public double glintStrength;
    public double damageTiltStrength;
    public boolean backgroundForChatOnly;
    public float textBackgroundOpacity;
    public net.minecraft.client.CloudStatus cloudStatus;
    public net.minecraft.client.CameraType cameraType;
    public int renderDistance;
    public double chunkSectionFadeInTime;
    public net.minecraft.client.PrioritizeChunkUpdates prioritizeChunkUpdates;
    public int fov;
    public net.minecraft.client.renderer.state.OptionsRenderState();
    public float getBackgroundOpacity(float);
}
```
