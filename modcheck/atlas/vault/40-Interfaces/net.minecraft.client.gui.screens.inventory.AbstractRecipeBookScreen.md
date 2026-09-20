---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.inventory.AbstractRecipeBookScreen"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.inventory.AbstractRecipeBookScreen

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`abstract_class` public abstract; extends `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen`; implements `net/minecraft/client/gui/screens/recipebook/RecipeUpdateListener`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `extractRenderState` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |

## Declared members (2 fields, 20 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final recipeBookComponent : Lnet/minecraft/client/gui/screens/recipebook/RecipeBookComponent;
private widthTooNarrow : Z
public <init>(Lnet/minecraft/world/inventory/RecipeBookMenu;Lnet/minecraft/client/gui/screens/recipebook/RecipeBookComponent;Lnet/minecraft/world/entity/player/Inventory;Lnet/minecraft/network/chat/Component;)V
protected init()V
protected abstract getRecipeBookButtonPosition()Lnet/minecraft/client/gui/navigation/ScreenPosition;
private initButton()V
protected onRecipeBookButtonClick()V
public extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V
protected extractSlots(Lnet/minecraft/client/gui/GuiGraphicsExtractor;II)V
protected isBiggerResultSlot()Z
public charTyped(Lnet/minecraft/client/input/CharacterEvent;)Z
public keyPressed(Lnet/minecraft/client/input/KeyEvent;)Z
public isInputCaptured()Z
public mouseClicked(Lnet/minecraft/client/input/MouseButtonEvent;Z)Z
public mouseDragged(Lnet/minecraft/client/input/MouseButtonEvent;DD)Z
protected isHovering(IIIIDD)Z
protected hasClickedOutside(DDII)Z
protected slotClicked(Lnet/minecraft/world/inventory/Slot;IILnet/minecraft/world/inventory/ContainerInput;)V
public containerTick()V
public recipesUpdated()V
public fillGhostRecipe(Lnet/minecraft/world/item/crafting/display/RecipeDisplay;)V
private synthetic lambda$initButton$0(Lnet/minecraft/client/gui/components/Button;)V
```
