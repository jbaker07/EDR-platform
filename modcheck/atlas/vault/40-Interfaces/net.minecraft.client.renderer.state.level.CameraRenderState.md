---
type: "interface"
fqcn: "net.minecraft.client.renderer.state.level.CameraRenderState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.state.level.CameraRenderState

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `clearExtraData` | `()V` | inherited_exact | invokevirtual@13 in `LevelRenderStateMixin.clearExtraRenderData` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (19 fields, 1 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public blockPos : Lnet/minecraft/core/BlockPos;
public pos : Lnet/minecraft/world/phys/Vec3;
public xRot : F
public yRot : F
public initialized : Z
public isPanoramicMode : Z
public isFrustumCaptured : Z
public isFirstPerson : Z
public smartCull : Z
public orientation : Lorg/joml/Quaternionf;
public cameraEntityPartialTicks : F
public cullFrustum : Lnet/minecraft/client/renderer/culling/Frustum;
public fogType : Lnet/minecraft/world/level/material/FogType;
public fogData : Lnet/minecraft/client/renderer/fog/FogData;
public hudFov : F
public depthFar : F
public projectionMatrix : Lorg/joml/Matrix4f;
public viewRotationMatrix : Lorg/joml/Matrix4f;
public entityRenderState : Lnet/minecraft/client/renderer/state/level/CameraEntityRenderState;
public <init>()V
```
