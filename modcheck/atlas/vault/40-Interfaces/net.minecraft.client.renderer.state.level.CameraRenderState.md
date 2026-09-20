---
type: "interface"
fqcn: "net.minecraft.client.renderer.state.level.CameraRenderState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.state.level.CameraRenderState

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `clearExtraData()V` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (20, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.state.level.CameraRenderState {
    public net.minecraft.core.BlockPos blockPos;
    public net.minecraft.world.phys.Vec3 pos;
    public float xRot;
    public float yRot;
    public boolean initialized;
    public boolean isPanoramicMode;
    public boolean isFrustumCaptured;
    public boolean isFirstPerson;
    public boolean smartCull;
    public org.joml.Quaternionf orientation;
    public float cameraEntityPartialTicks;
    public net.minecraft.client.renderer.culling.Frustum cullFrustum;
    public net.minecraft.world.level.material.FogType fogType;
    public net.minecraft.client.renderer.fog.FogData fogData;
    public float hudFov;
    public float depthFar;
    public org.joml.Matrix4f projectionMatrix;
    public org.joml.Matrix4f viewRotationMatrix;
    public net.minecraft.client.renderer.state.level.CameraEntityRenderState entityRenderState;
    public net.minecraft.client.renderer.state.level.CameraRenderState();
}
```
