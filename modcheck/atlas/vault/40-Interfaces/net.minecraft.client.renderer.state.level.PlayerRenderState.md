---
type: "interface"
fqcn: "net.minecraft.client.renderer.state.level.PlayerRenderState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.state.level.PlayerRenderState

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `reset` | `()V` | name_only | @Inject at ['TAIL'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `firstPersonHandsAndItems` | `Lnet/minecraft/client/renderer/state/level/FirstPersonHandsAndItemsRen` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | declared |

## Declared members (13 fields, 2 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public hasPlayer : Z
public avatarRenderState : Lnet/minecraft/client/renderer/entity/state/AvatarRenderState;
public final firstPersonHandsAndItems : Lnet/minecraft/client/renderer/state/level/FirstPersonHandsAndItemsRenderState;
public portalEffectIntensity : F
public nauseaEffectIntensity : F
public spinningEffectAngle : F
public isEyeInWater : Z
public isOnFire : Z
public isUnderWater : Z
public eyePositionY : D
public blockOverlay : Lnet/minecraft/client/renderer/state/level/PlayerRenderState$BlockOverlay;
public waterOverlay : Lnet/minecraft/client/renderer/state/level/PlayerRenderState$WaterOverlay;
public itemActivation : Lnet/minecraft/client/renderer/state/level/PlayerRenderState$ItemActivationRenderState;
public <init>()V
public reset()V
```
