---
type: "interface"
fqcn: "net.minecraft.client.gui.components.toasts.AdvancementToast"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.components.toasts.AdvancementToast

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/client/gui/components/toasts/Toast`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `advancement` | `Lnet/minecraft/advancements/AdvancementHolder;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | declared |
| wraps | `extractRenderState` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (5 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final BACKGROUND_SPRITE : Lnet/minecraft/resources/Identifier;
public static final DISPLAY_TIME : I
private final advancement : Lnet/minecraft/advancements/AdvancementHolder;
private wantedVisibility : Lnet/minecraft/client/gui/components/toasts/Toast$Visibility;
private final iconItem : Lnet/minecraft/world/item/ItemStack;
public <init>(Lnet/minecraft/advancements/AdvancementHolder;)V
public getWantedVisibility()Lnet/minecraft/client/gui/components/toasts/Toast$Visibility;
public update(Lnet/minecraft/client/gui/components/toasts/ToastManager;J)V
public getSoundEvent()Lnet/minecraft/sounds/SoundEvent;
private isChallengeAdvancement()Z
public extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/gui/Font;J)V
private static synthetic lambda$new$0(Lnet/minecraft/advancements/DisplayInfo;)Lnet/minecraft/world/item/ItemStack;
static <clinit>()V
```
