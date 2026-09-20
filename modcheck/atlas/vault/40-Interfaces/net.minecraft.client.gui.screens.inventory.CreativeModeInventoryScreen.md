---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.inventory.CreativeModeInventoryScreen"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.inventory.CreativeModeInventoryScreen

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen`; implements `net/fabricmc/fabric/api/client/creativetab/v1/FabricCreativeModeInventoryScreen`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getCurrentPage` | `()I` | inherited_exact | invokevirtual@153 in `FabricCreativeGuiComponents$CreativeModeTabButton.extractContents` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `getCurrentPage` | `()I` | inherited_exact | invokevirtual@1 in `FabricCreativeGuiComponents$Type.lambda$static$1` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `getCurrentPage` | `()I` | inherited_exact | invokevirtual@1 in `FabricCreativeGuiComponents$Type.lambda$static$0` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `getPageCount` | `()I` | inherited_exact | invokevirtual@7 in `FabricCreativeGuiComponents$Type.lambda$static$0` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `hasAdditionalPages` | `()Z` | inherited_exact | invokevirtual@25 in `FabricCreativeGuiComponents$CreativeModeTabButton.extractContents` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `selectTab` | `(Lnet/minecraft/world/item/CreativeModeTab;)V` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | declared |
| injects_into | `checkTabClicked` | `(Lnet/minecraft/world/item/CreativeModeTab;DD)Z` | name_only | @Inject at ['HEAD'] | client | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| injects_into | `checkTabHovering` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/world/i` | name_only | @Inject at ['HEAD'] | client | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| injects_into | `extractTabButton` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IILnet/minecraft/world` | name_only | @Inject at ['HEAD'] | client | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| injects_into | `init` | `?` | ambiguous | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| injects_into | `keyPressed` | `(Lnet/minecraft/client/input/KeyEvent;)Z` | name_only | @Inject at ['HEAD'] | client | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| injects_into | `selectTab` | `(Lnet/minecraft/world/item/CreativeModeTab;)V` | name_only | @Inject at ['HEAD'] | client | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `selectedTab` | `Lnet/minecraft/world/item/CreativeModeTab;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | declared |

## Declared members (26 fields, 43 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final SCROLLER_SPRITE : Lnet/minecraft/resources/Identifier;
private static final SCROLLER_DISABLED_SPRITE : Lnet/minecraft/resources/Identifier;
private static final UNSELECTED_TOP_TABS : [Lnet/minecraft/resources/Identifier;
private static final SELECTED_TOP_TABS : [Lnet/minecraft/resources/Identifier;
private static final UNSELECTED_BOTTOM_TABS : [Lnet/minecraft/resources/Identifier;
private static final SELECTED_BOTTOM_TABS : [Lnet/minecraft/resources/Identifier;
private static final NUM_ROWS : I
private static final NUM_COLS : I
private static final TAB_WIDTH : I
private static final TAB_HEIGHT : I
private static final SCROLLER_WIDTH : I
private static final SCROLLER_HEIGHT : I
private static final CONTAINER : Lnet/minecraft/world/SimpleContainer;
private static final TRASH_SLOT_TOOLTIP : Lnet/minecraft/network/chat/Component;
private static selectedTab : Lnet/minecraft/world/item/CreativeModeTab;
private scrollOffs : F
private scrolling : Z
private searchBox : Lnet/minecraft/client/gui/components/EditBox;
private originalSlots : Ljava/util/List;
private destroyItemSlot : Lnet/minecraft/world/inventory/Slot;
private listener : Lnet/minecraft/client/gui/screens/inventory/CreativeInventoryListener;
private ignoreTextInput : Z
private hasClickedOutside : Z
private final visibleTags : Ljava/util/Set;
private final displayOperatorCreativeTab : Z
private final effects : Lnet/minecraft/client/gui/screens/inventory/EffectsInInventory;
public <init>(Lnet/minecraft/client/player/LocalPlayer;Lnet/minecraft/world/flag/FeatureFlagSet;Z)V
private hasPermissions(Lnet/minecraft/world/entity/player/Player;)Z
private tryRefreshInvalidatedTabs(Lnet/minecraft/world/flag/FeatureFlagSet;ZLnet/minecraft/core/HolderLookup$Provider;)V
private tryRebuildTabContents(Lnet/minecraft/client/multiplayer/SessionSearchTrees;Lnet/minecraft/world/flag/FeatureFlagSet;ZLnet/minecraft/core/HolderLookup$Provider;)Z
private refreshCurrentTabContents(Ljava/util/Collection;)V
public containerTick()V
protected slotClicked(Lnet/minecraft/world/inventory/Slot;IILnet/minecraft/world/inventory/ContainerInput;)V
private isCreativeSlot(Lnet/minecraft/world/inventory/Slot;)Z
protected init()V
public resize(II)V
public removed()V
public charTyped(Lnet/minecraft/client/input/CharacterEvent;)Z
public preeditUpdated(Lnet/minecraft/client/input/PreeditEvent;)Z
public keyPressed(Lnet/minecraft/client/input/KeyEvent;)Z
public keyReleased(Lnet/minecraft/client/input/KeyEvent;)Z
public isInputCaptured()Z
private refreshSearchResults()V
private updateVisibleTags(Ljava/lang/String;)V
protected extractLabels(Lnet/minecraft/client/gui/GuiGraphicsExtractor;II)V
public mouseClicked(Lnet/minecraft/client/input/MouseButtonEvent;Z)Z
public mouseReleased(Lnet/minecraft/client/input/MouseButtonEvent;)Z
private canScroll()Z
private selectTab(Lnet/minecraft/world/item/CreativeModeTab;)V
public mouseScrolled(DDDD)Z
protected hasClickedOutside(DDII)Z
protected insideScrollbar(DD)Z
public mouseDragged(Lnet/minecraft/client/input/MouseButtonEvent;DD)Z
public extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V
public showsActiveEffects()Z
public getTooltipFromContainerItem(Lnet/minecraft/world/item/ItemStack;)Ljava/util/List;
public extractBackground(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V
private getTabX(Lnet/minecraft/world/item/CreativeModeTab;)I
private getTabY(Lnet/minecraft/world/item/CreativeModeTab;)I
protected checkTabClicked(Lnet/minecraft/world/item/CreativeModeTab;DD)Z
protected checkTabHovering(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/world/item/CreativeModeTab;II)Z
protected extractTabButton(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IILnet/minecraft/world/item/CreativeModeTab;)V
public isInventoryOpen()Z
public static handleHotbarLoadOrSave(Lnet/minecraft/client/Minecraft;IZZ)V
private static synthetic lambda$getTooltipFromContainerItem$0(Lnet/minecraft/world/item/ItemStack;Ljava/util/List;Lnet/minecraft/tags/TagKey;)V
private static synthetic lambda$updateVisibleTags$2(Ljava/util/function/Predicate;Lnet/minecraft/tags/TagKey;)Z
private static synthetic lambda$updateVisibleTags$1(Ljava/lang/String;Ljava/lang/String;Lnet/minecraft/resources/Identifier;)Z
private static synthetic lambda$updateVisibleTags$0(Ljava/lang/String;Lnet/minecraft/resources/Identifier;)Z
static <clinit>()V
```
