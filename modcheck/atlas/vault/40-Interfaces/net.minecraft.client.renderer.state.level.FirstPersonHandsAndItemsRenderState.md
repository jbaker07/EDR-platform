---
type: "interface"
fqcn: "net.minecraft.client.renderer.state.level.FirstPersonHandsAndItemsRenderState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.state.level.FirstPersonHandsAndItemsRenderState

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `clearExtraData` | `()V` | inherited_exact | invokevirtual@13 in `PlayerRenderStateMixin.clearExtraRenderData` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (26 fields, 1 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public attackHand : Lnet/minecraft/world/InteractionHand;
public viewXRot : F
public viewYRot : F
public xBob : F
public yBob : F
public isScoping : Z
public useItemRemainingTicks : I
public mainHandUseDuration : I
public offHandUseDuration : I
public mainHandChargeDuration : I
public offHandChargeDuration : I
public mainHandSwapScale : F
public offHandSwapScale : F
public handRenderSelection : Lnet/minecraft/client/renderer/state/level/FirstPersonHandsAndItemsRenderState$HandRenderSelection;
public mainHandItem : Lnet/minecraft/world/item/ItemStack;
public offHandItem : Lnet/minecraft/world/item/ItemStack;
public mainHandHeight : F
public oldMainHandHeight : F
public offHandHeight : F
public oldOffHandHeight : F
public final mainHandRenderState : Lnet/minecraft/client/renderer/item/ItemStackRenderState;
public final offHandRenderState : Lnet/minecraft/client/renderer/item/ItemStackRenderState;
public final mainHandMapRenderState : Lnet/minecraft/client/renderer/state/MapRenderState;
public final offHandMapRenderState : Lnet/minecraft/client/renderer/state/MapRenderState;
public hasMainHandMapData : Z
public hasOffHandMapData : Z
public <init>()V
```
