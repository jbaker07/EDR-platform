---
type: "interface"
fqcn: "net.minecraft.client.renderer.state.level.PlayerRenderState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.state.level.PlayerRenderState

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `reset` | `@Inject at TAIL` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (15, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.state.level.PlayerRenderState {
    public boolean hasPlayer;
    public net.minecraft.client.renderer.entity.state.AvatarRenderState avatarRenderState;
    public final net.minecraft.client.renderer.state.level.FirstPersonHandsAndItemsRenderState firstPersonHandsAndItems;
    public float portalEffectIntensity;
    public float nauseaEffectIntensity;
    public float spinningEffectAngle;
    public boolean isEyeInWater;
    public boolean isOnFire;
    public boolean isUnderWater;
    public double eyePositionY;
    public net.minecraft.client.renderer.state.level.PlayerRenderState$BlockOverlay blockOverlay;
    public net.minecraft.client.renderer.state.level.PlayerRenderState$WaterOverlay waterOverlay;
    public net.minecraft.client.renderer.state.level.PlayerRenderState$ItemActivationRenderState itemActivation;
    public net.minecraft.client.renderer.state.level.PlayerRenderState();
    public void reset();
}
```
