---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.inventory.AbstractContainerScreen"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.inventory.AbstractContainerScreen

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`abstract_class` public abstract; extends `net/minecraft/client/gui/screens/Screen`; implements `net/minecraft/client/gui/screens/inventory/MenuAccess`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/world/inventory/AbstractContainerMenu;Lnet/minecraft/w` | exact | invokespecial@4 in `CreativeModeInventoryScreenMixin.<init>` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/world/inventory/AbstractContainerMenu;Lnet/minecraft/w` | exact | invokespecial@4 in `AbstractRecipeBookScreenMixin.<init>` | unknown | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| injects_into | `extractRenderState` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| injects_into | `mouseDragged` | `(Lnet/minecraft/client/input/MouseButtonEvent;DD)Z` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| injects_into | `mouseReleased` | `(Lnet/minecraft/client/input/MouseButtonEvent;)Z` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |

## Declared members (28 fields, 44 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final INVENTORY_LOCATION : Lnet/minecraft/resources/Identifier;
private static final SLOT_HIGHLIGHT_BACK_SPRITE : Lnet/minecraft/resources/Identifier;
private static final SLOT_HIGHLIGHT_FRONT_SPRITE : Lnet/minecraft/resources/Identifier;
protected static final BACKGROUND_TEXTURE_WIDTH : I
protected static final BACKGROUND_TEXTURE_HEIGHT : I
protected static final DEFAULT_IMAGE_WIDTH : I
protected static final DEFAULT_IMAGE_HEIGHT : I
protected final imageWidth : I
protected final imageHeight : I
protected titleLabelX : I
protected titleLabelY : I
protected inventoryLabelX : I
protected inventoryLabelY : I
private final itemSlotMouseActions : Ljava/util/List;
protected final menu : Lnet/minecraft/world/inventory/AbstractContainerMenu;
protected final playerInventoryTitle : Lnet/minecraft/network/chat/Component;
protected hoveredSlot : Lnet/minecraft/world/inventory/Slot;
private lastClickSlot : Lnet/minecraft/world/inventory/Slot;
protected leftPos : I
protected topPos : I
protected final quickCraftSlots : Ljava/util/Set;
protected isQuickCrafting : Z
private quickCraftingType : I
private quickCraftingButton : I
private skipNextRelease : Z
private quickCraftingRemainder : I
private doubleclick : Z
private lastQuickMoved : Lnet/minecraft/world/item/ItemStack;
public <init>(Lnet/minecraft/world/inventory/AbstractContainerMenu;Lnet/minecraft/world/entity/player/Inventory;Lnet/minecraft/network/chat/Component;)V
public <init>(Lnet/minecraft/world/inventory/AbstractContainerMenu;Lnet/minecraft/world/entity/player/Inventory;Lnet/minecraft/network/chat/Component;II)V
protected init()V
protected addItemSlotMouseAction(Lnet/minecraft/client/gui/ItemSlotMouseAction;)V
public extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V
public extractContents(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V
public extractCarriedItem(Lnet/minecraft/client/gui/GuiGraphicsExtractor;II)V
protected extractSlots(Lnet/minecraft/client/gui/GuiGraphicsExtractor;II)V
public mouseScrolled(DDDD)Z
private extractSlotHighlightBack(Lnet/minecraft/client/gui/GuiGraphicsExtractor;)V
private extractSlotHighlightFront(Lnet/minecraft/client/gui/GuiGraphicsExtractor;)V
protected extractTooltip(Lnet/minecraft/client/gui/GuiGraphicsExtractor;II)V
private showTooltipWithItemInHand(Lnet/minecraft/world/item/ItemStack;)Z
protected getTooltipFromContainerItem(Lnet/minecraft/world/item/ItemStack;)Ljava/util/List;
private extractFloatingItem(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/world/item/ItemStack;IILjava/lang/String;)V
protected extractLabels(Lnet/minecraft/client/gui/GuiGraphicsExtractor;II)V
protected extractSlot(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/world/inventory/Slot;II)V
private recalculateQuickCraftRemaining()V
private getHoveredSlot(DD)Lnet/minecraft/world/inventory/Slot;
private slotClicked(Lnet/minecraft/world/inventory/Slot;ILnet/minecraft/client/input/MouseButtonEvent;Lnet/minecraft/world/inventory/ContainerInput;)V
private static getContainerClickButton(Lnet/minecraft/client/input/MouseButtonEvent;)I
public mouseClicked(Lnet/minecraft/client/input/MouseButtonEvent;Z)Z
private checkHotbarMouseClicked(Lnet/minecraft/client/input/MouseButtonEvent;)V
protected hasClickedOutside(DDII)Z
public mouseDragged(Lnet/minecraft/client/input/MouseButtonEvent;DD)Z
public mouseReleased(Lnet/minecraft/client/input/MouseButtonEvent;)Z
private isHovering(Lnet/minecraft/world/inventory/Slot;DD)Z
protected isHovering(IIIIDD)Z
private onStopHovering(Lnet/minecraft/world/inventory/Slot;)V
protected slotClicked(Lnet/minecraft/world/inventory/Slot;IILnet/minecraft/world/inventory/ContainerInput;)V
protected onMouseClickAction(Lnet/minecraft/world/inventory/Slot;Lnet/minecraft/world/inventory/ContainerInput;)V
protected handleSlotStateChanged(IIZ)V
public keyPressed(Lnet/minecraft/client/input/KeyEvent;)Z
protected checkHotbarKeyPressed(Lnet/minecraft/client/input/KeyEvent;)Z
public removed()V
public isPauseScreen()Z
public isInGameUi()Z
public final tick()V
protected containerTick()V
private shouldAddSlotToQuickCraft(Lnet/minecraft/world/inventory/Slot;Lnet/minecraft/world/item/ItemStack;)Z
private quickCraftToSlots()V
public getMenu()Lnet/minecraft/world/inventory/AbstractContainerMenu;
public onClose()V
static <clinit>()V
```
