---
type: "interface"
fqcn: "net.minecraft.client.player.FirstPersonHandsAndItems"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.player.FirstPersonHandsAndItems

System: [[20-Systems/net.minecraft.client.player|net.minecraft.client.player]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `tick` | `(Lnet/minecraft/client/player/LocalPlayer;)V` | name_only | @Inject at ['HEAD'] | client | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| reads | `mainHandItem` | `Lnet/minecraft/world/item/ItemStack;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | declared |
| reads | `offHandItem` | `Lnet/minecraft/world/item/ItemStack;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | declared |

## Declared members (6 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private mainHandItem : Lnet/minecraft/world/item/ItemStack;
private offHandItem : Lnet/minecraft/world/item/ItemStack;
private mainHandHeight : F
private oMainHandHeight : F
private offHandHeight : F
private oOffHandHeight : F
public <init>()V
public tick(Lnet/minecraft/client/player/LocalPlayer;)V
public itemUsed(Lnet/minecraft/world/InteractionHand;)V
public extractRenderState(Lnet/minecraft/client/player/LocalPlayer;FLnet/minecraft/client/renderer/state/level/FirstPersonHandsAndItemsRenderState;)V
private shouldInstantlyReplaceVisibleItem(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/client/player/LocalPlayer;)Z
private extractMapRenderState(Lnet/minecraft/client/player/LocalPlayer;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/client/renderer/state/MapRenderState;)Z
public static evaluateWhichHandsToRender(Lnet/minecraft/client/player/LocalPlayer;)Lnet/minecraft/client/renderer/state/level/FirstPersonHandsAndItemsRenderState$HandRenderSelection;
private static isChargedCrossbow(Lnet/minecraft/world/item/ItemStack;)Z
```
