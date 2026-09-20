---
type: "interface"
fqcn: "net.minecraft.world.item.CreativeModeTab"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.CreativeModeTab

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`class` public; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `column` | `()I` | exact | invokevirtual@382 in `CreativeModeTabsMixin.paginateTabs` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `getDisplayName` | `()Lnet/minecraft/network/chat/Component;` | exact | invokevirtual@361 in `CreativeModeTabsMixin.paginateTabs` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `getDisplayName` | `()Lnet/minecraft/network/chat/Component;` | exact | invokevirtual@14 in `FabricLanguageProvider$TranslationBuilder.addCreativeModeTab` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getDisplayName` | `()Lnet/minecraft/network/chat/Component;` | exact | invokevirtual@65 in `FabricLanguageProvider$TranslationBuilder.addCreativeModeTab` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `isAlignedRight` | `()Z` | exact | invokevirtual@29 in `CreativeModeTabMixin.getStacks` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `isAlignedRight` | `()Z` | exact | invokevirtual@1 in `CreativeModeInventoryScreenMixin.lambda$getTabsOnPage$1` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `isAlignedRight` | `()Z` | exact | invokevirtual@5 in `CreativeModeInventoryScreenMixin.lambda$getTabsOnPage$1` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `isAlignedRight` | `()Z` | exact | invokevirtual@1 in `CreativeModeInventoryScreenMixin.lambda$updateSelection$0` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `isAlignedRight` | `()Z` | exact | invokevirtual@5 in `CreativeModeInventoryScreenMixin.lambda$updateSelection$0` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `row` | `()Lnet/minecraft/world/item/CreativeModeTab$Row;` | exact | invokevirtual@377 in `CreativeModeTabsMixin.paginateTabs` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `shouldDisplay` | `()Z` | exact | invokevirtual@7 in `CreativeModeTabsMixin.lambda$paginateTabs$0` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `shouldDisplay` | `()Z` | exact | invokevirtual@17 in `CreativeModeTabsMixin.lambda$paginateTabs$0` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `shouldDisplay` | `()Z` | exact | invokevirtual@1 in `CreativeModeInventoryScreenMixin.isTabVisible` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| injects_into | `buildContents` | `(Lnet/minecraft/world/item/CreativeModeTab$ItemDisplayParameters;)V` | name_only | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `displayItems` | `Ljava/util/Collection;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | declared |
| reads | `displayItemsSearchTab` | `Ljava/util/Set;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | declared |

## Declared members (14 fields, 19 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final DEFAULT_BACKGROUND : Lnet/minecraft/resources/Identifier;
private final displayName : Lnet/minecraft/network/chat/Component;
private backgroundTexture : Lnet/minecraft/resources/Identifier;
private canScroll : Z
private showTitle : Z
private alignedRight : Z
private final row : Lnet/minecraft/world/item/CreativeModeTab$Row;
private final column : I
private final type : Lnet/minecraft/world/item/CreativeModeTab$Type;
private iconItemStack : Lnet/minecraft/world/item/ItemStack;
private displayItems : Ljava/util/Collection;
private displayItemsSearchTab : Ljava/util/Set;
private final iconGenerator : Ljava/util/function/Supplier;
private final displayItemsGenerator : Lnet/minecraft/world/item/CreativeModeTab$DisplayItemsGenerator;
private <init>(Lnet/minecraft/world/item/CreativeModeTab$Row;ILnet/minecraft/world/item/CreativeModeTab$Type;Lnet/minecraft/network/chat/Component;Ljava/util/function/Supplier;Lnet/minecraft/world/item/CreativeModeTab$DisplayItemsGenerator;)V
public static createTextureLocation(Ljava/lang/String;)Lnet/minecraft/resources/Identifier;
public static builder(Lnet/minecraft/world/item/CreativeModeTab$Row;I)Lnet/minecraft/world/item/CreativeModeTab$Builder;
public getDisplayName()Lnet/minecraft/network/chat/Component;
public getIconItem()Lnet/minecraft/world/item/ItemStack;
public getBackgroundTexture()Lnet/minecraft/resources/Identifier;
public showTitle()Z
public canScroll()Z
public column()I
public row()Lnet/minecraft/world/item/CreativeModeTab$Row;
public hasAnyItems()Z
public shouldDisplay()Z
public isAlignedRight()Z
public getType()Lnet/minecraft/world/item/CreativeModeTab$Type;
public buildContents(Lnet/minecraft/world/item/CreativeModeTab$ItemDisplayParameters;)V
public getDisplayItems()Ljava/util/Collection;
public getSearchTabDisplayItems()Ljava/util/Collection;
public contains(Lnet/minecraft/world/item/ItemStack;)Z
static <clinit>()V
```
