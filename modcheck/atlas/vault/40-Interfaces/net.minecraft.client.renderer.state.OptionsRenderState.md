---
type: "interface"
fqcn: "net.minecraft.client.renderer.state.OptionsRenderState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.state.OptionsRenderState

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `ambientOcclusion` | `Z` | exact | getfield@9 in `MovingBlockFeatureRendererMixin.beforeInitBlockRenderer` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (21 fields, 2 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public cloudRange : I
public cutoutLeaves : Z
public improvedTransparency : Z
public ambientOcclusion : Z
public menuBackgroundBlurriness : I
public panoramaSpeed : D
public maxAnisotropyValue : I
public textureFiltering : Lnet/minecraft/client/TextureFilteringMethod;
public bobView : Z
public screenEffectScale : F
public glintSpeed : D
public glintStrength : D
public damageTiltStrength : D
public backgroundForChatOnly : Z
public textBackgroundOpacity : F
public cloudStatus : Lnet/minecraft/client/CloudStatus;
public cameraType : Lnet/minecraft/client/CameraType;
public renderDistance : I
public chunkSectionFadeInTime : D
public prioritizeChunkUpdates : Lnet/minecraft/client/PrioritizeChunkUpdates;
public fov : I
public <init>()V
public getBackgroundOpacity(F)F
```
