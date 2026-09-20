---
type: "interface"
fqcn: "net.minecraft.client.renderer.state.level.SkyRenderState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.state.level.SkyRenderState

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `reset` | `@Inject at TAIL` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (15, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.state.level.SkyRenderState {
    public net.minecraft.world.level.dimension.DimensionType$Skybox skybox;
    public boolean shouldRenderDarkDisc;
    public float sunAngle;
    public float moonAngle;
    public float starAngle;
    public float rainBrightness;
    public float starBrightness;
    public org.joml.Vector4fc sunriseAndSunsetColor;
    public net.minecraft.world.level.MoonPhase moonPhase;
    public org.joml.Vector3fc skyColor;
    public float endFlashIntensity;
    public float endFlashXAngle;
    public float endFlashYAngle;
    public net.minecraft.client.renderer.state.level.SkyRenderState();
    public void reset();
}
```
