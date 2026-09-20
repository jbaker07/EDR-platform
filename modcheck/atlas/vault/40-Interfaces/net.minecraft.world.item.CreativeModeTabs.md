---
type: "interface"
fqcn: "net.minecraft.world.item.CreativeModeTabs"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.CreativeModeTabs

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`class` public; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `allTabs` | `()Ljava/util/List;` | exact | invokestatic@10 in `CreativeModeInventoryScreenMixin.updateSelection` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `tabs` | `()Ljava/util/List;` | exact | invokestatic@0 in `FabricCreativeGuiComponents.getPageCount` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `tabs` | `()Ljava/util/List;` | exact | invokestatic@0 in `CreativeModeInventoryScreenMixin.hasGroupForPage` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `tabs` | `()Ljava/util/List;` | exact | invokestatic@0 in `CreativeModeInventoryScreenMixin.getTabsOnPage` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `tabs` | `()Ljava/util/List;` | exact | invokestatic@8 in `CreativeModeInventoryScreenMixin.hasAdditionalPages` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `validate` | `()V` | exact | invokestatic@30 in `MainMixin.afterModInit` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `validate` | `()V` | exact | invokestatic@16 in `MinecraftMixin.afterModInit` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `buildAllTabContents` | `(Lnet/minecraft/world/item/CreativeModeTab$ItemDisplayParameters;)V` | name_only | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| injects_into | `validate` | `()V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `BUILDING_BLOCKS` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@7 in `CreativeModeTabsMixin.paginateTabs` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `CACHED_PARAMETERS` | `Lnet/minecraft/world/item/CreativeModeTab$ItemDisplayParameters;` | exact | getstatic@0 in `CreativeModeInventoryScreenMixin.hasAdditionalPages` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `COLORED_BLOCKS` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@13 in `CreativeModeTabsMixin.paginateTabs` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `COMBAT` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@58 in `CreativeModeTabsMixin.paginateTabs` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `FOOD_AND_DRINKS` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@65 in `CreativeModeTabsMixin.paginateTabs` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `FUNCTIONAL_BLOCKS` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@25 in `CreativeModeTabsMixin.paginateTabs` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `HOTBAR` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@16 in `FabricCreativeGuiComponents.<clinit>` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `HOTBAR` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@37 in `CreativeModeTabsMixin.paginateTabs` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `INGREDIENTS` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@72 in `CreativeModeTabsMixin.paginateTabs` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `INVENTORY` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@13 in `FabricCreativeGuiComponents.<clinit>` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `INVENTORY` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@93 in `CreativeModeTabsMixin.paginateTabs` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `NATURAL_BLOCKS` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@19 in `CreativeModeTabsMixin.paginateTabs` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `OP_BLOCKS` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@19 in `FabricCreativeGuiComponents.<clinit>` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `OP_BLOCKS` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@37 in `CreativeModeTabMixin.getStacks` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `OP_BLOCKS` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@106 in `CreativeModeTabMixin.getStacks` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `OP_BLOCKS` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@86 in `CreativeModeTabsMixin.paginateTabs` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `REDSTONE_BLOCKS` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@31 in `CreativeModeTabsMixin.paginateTabs` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `SEARCH` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@10 in `FabricCreativeGuiComponents.<clinit>` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `SEARCH` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@44 in `CreativeModeTabsMixin.paginateTabs` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `SPAWN_EGGS` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@79 in `CreativeModeTabsMixin.paginateTabs` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `TOOLS_AND_UTILITIES` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@51 in `CreativeModeTabsMixin.paginateTabs` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |

## Declared members (18 fields, 76 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final INVENTORY_BACKGROUND : Lnet/minecraft/resources/Identifier;
private static final SEARCH_BACKGROUND : Lnet/minecraft/resources/Identifier;
public static final BUILDING_BLOCKS : Lnet/minecraft/resources/ResourceKey;
public static final COLORED_BLOCKS : Lnet/minecraft/resources/ResourceKey;
public static final NATURAL_BLOCKS : Lnet/minecraft/resources/ResourceKey;
public static final FUNCTIONAL_BLOCKS : Lnet/minecraft/resources/ResourceKey;
public static final REDSTONE_BLOCKS : Lnet/minecraft/resources/ResourceKey;
public static final HOTBAR : Lnet/minecraft/resources/ResourceKey;
public static final SEARCH : Lnet/minecraft/resources/ResourceKey;
public static final TOOLS_AND_UTILITIES : Lnet/minecraft/resources/ResourceKey;
public static final COMBAT : Lnet/minecraft/resources/ResourceKey;
public static final FOOD_AND_DRINKS : Lnet/minecraft/resources/ResourceKey;
public static final INGREDIENTS : Lnet/minecraft/resources/ResourceKey;
public static final SPAWN_EGGS : Lnet/minecraft/resources/ResourceKey;
public static final OP_BLOCKS : Lnet/minecraft/resources/ResourceKey;
public static final INVENTORY : Lnet/minecraft/resources/ResourceKey;
private static final PAINTING_COMPARATOR : Ljava/util/Comparator;
private static CACHED_PARAMETERS : Lnet/minecraft/world/item/CreativeModeTab$ItemDisplayParameters;
public <init>()V
private static createKey(Ljava/lang/String;)Lnet/minecraft/resources/ResourceKey;
public static bootstrap(Lnet/minecraft/core/Registry;)Lnet/minecraft/world/item/CreativeModeTab;
private static registerColoredItems(Lnet/minecraft/world/item/CreativeModeTab$Output;Ljava/util/List;Lnet/minecraft/world/level/block/ColorCollection;)V
public static validate()V
public static getDefaultTab()Lnet/minecraft/world/item/CreativeModeTab;
private static generatePotionEffectTypes(Lnet/minecraft/world/item/CreativeModeTab$Output;Lnet/minecraft/core/HolderLookup;Lnet/minecraft/world/item/Item;Lnet/minecraft/world/item/CreativeModeTab$TabVisibility;Lnet/minecraft/world/flag/FeatureFlagSet;)V
private static generateEnchantmentBookTypesOnlyMaxLevel(Lnet/minecraft/world/item/CreativeModeTab$Output;Lnet/minecraft/core/HolderLookup;Lnet/minecraft/world/item/CreativeModeTab$TabVisibility;)V
private static generateEnchantmentBookTypesAllLevels(Lnet/minecraft/world/item/CreativeModeTab$Output;Lnet/minecraft/core/HolderLookup;Lnet/minecraft/world/item/CreativeModeTab$TabVisibility;)V
private static generateInstrumentTypes(Lnet/minecraft/world/item/CreativeModeTab$Output;Lnet/minecraft/core/HolderLookup;Lnet/minecraft/world/item/Item;Lnet/minecraft/tags/TagKey;Lnet/minecraft/world/item/CreativeModeTab$TabVisibility;)V
private static generateSuspiciousStews(Lnet/minecraft/world/item/CreativeModeTab$Output;Lnet/minecraft/world/item/CreativeModeTab$TabVisibility;)V
private static generateOminousBottles(Lnet/minecraft/world/item/CreativeModeTab$Output;Lnet/minecraft/world/item/CreativeModeTab$TabVisibility;)V
private static generateFireworksAllDurations(Lnet/minecraft/world/item/CreativeModeTab$Output;Lnet/minecraft/world/item/CreativeModeTab$TabVisibility;)V
private static generatePresetPaintings(Lnet/minecraft/world/item/CreativeModeTab$Output;Lnet/minecraft/core/HolderLookup$Provider;Lnet/minecraft/core/HolderLookup$RegistryLookup;Ljava/util/function/Predicate;Lnet/minecraft/world/item/CreativeModeTab$TabVisibility;)V
private static copperBlockFamilies(Ljava/util/function/Consumer;)V
public static tabs()Ljava/util/List;
public static allTabs()Ljava/util/List;
private static streamAllTabs()Ljava/util/stream/Stream;
public static searchTab()Lnet/minecraft/world/item/CreativeModeTab;
private static buildAllTabContents(Lnet/minecraft/world/item/CreativeModeTab$ItemDisplayParameters;)V
public static tryRebuildTabContents(Lnet/minecraft/world/flag/FeatureFlagSet;ZLnet/minecraft/core/HolderLookup$Provider;)Z
private static synthetic lambda$buildAllTabContents$3(Lnet/minecraft/world/item/CreativeModeTab$ItemDisplayParameters;Lnet/minecraft/world/item/CreativeModeTab;)V
private static synthetic lambda$buildAllTabContents$2(Lnet/minecraft/world/item/CreativeModeTab;)Z
private static synthetic lambda$buildAllTabContents$1(Lnet/minecraft/world/item/CreativeModeTab$ItemDisplayParameters;Lnet/minecraft/world/item/CreativeModeTab;)V
private static synthetic lambda$buildAllTabContents$0(Lnet/minecraft/world/item/CreativeModeTab;)Z
private static synthetic lambda$generatePresetPaintings$0(Lnet/minecraft/world/item/CreativeModeTab$Output;Lnet/minecraft/world/item/CreativeModeTab$TabVisibility;Lnet/minecraft/core/Holder$Reference;)V
private static synthetic lambda$generateInstrumentTypes$0(Lnet/minecraft/world/item/Item;Lnet/minecraft/world/item/CreativeModeTab$Output;Lnet/minecraft/world/item/CreativeModeTab$TabVisibility;Lnet/minecraft/core/HolderSet$Named;)V
private static synthetic lambda$generateInstrumentTypes$2(Lnet/minecraft/world/item/CreativeModeTab$Output;Lnet/minecraft/world/item/CreativeModeTab$TabVisibility;Lnet/minecraft/world/item/ItemStack;)V
private static synthetic lambda$generateInstrumentTypes$1(Lnet/minecraft/world/item/Item;Lnet/minecraft/core/Holder;)Lnet/minecraft/world/item/ItemStack;
private static synthetic lambda$generateEnchantmentBookTypesAllLevels$2(Lnet/minecraft/world/item/CreativeModeTab$Output;Lnet/minecraft/world/item/CreativeModeTab$TabVisibility;Lnet/minecraft/world/item/ItemStack;)V
private static synthetic lambda$generateEnchantmentBookTypesAllLevels$0(Lnet/minecraft/core/Holder$Reference;)Ljava/util/stream/Stream;
private static synthetic lambda$generateEnchantmentBookTypesAllLevels$1(Lnet/minecraft/core/Holder$Reference;I)Lnet/minecraft/world/item/ItemStack;
private static synthetic lambda$generateEnchantmentBookTypesOnlyMaxLevel$1(Lnet/minecraft/world/item/CreativeModeTab$Output;Lnet/minecraft/world/item/CreativeModeTab$TabVisibility;Lnet/minecraft/world/item/ItemStack;)V
private static synthetic lambda$generateEnchantmentBookTypesOnlyMaxLevel$0(Lnet/minecraft/core/Holder$Reference;)Lnet/minecraft/world/item/ItemStack;
private static synthetic lambda$generatePotionEffectTypes$2(Lnet/minecraft/world/item/CreativeModeTab$Output;Lnet/minecraft/world/item/CreativeModeTab$TabVisibility;Lnet/minecraft/world/item/ItemStack;)V
private static synthetic lambda$generatePotionEffectTypes$1(Lnet/minecraft/world/item/Item;Lnet/minecraft/core/Holder$Reference;)Lnet/minecraft/world/item/ItemStack;
private static synthetic lambda$generatePotionEffectTypes$0(Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/core/Holder$Reference;)Z
private static synthetic lambda$registerColoredItems$0(Lnet/minecraft/world/item/CreativeModeTab$Output;Lnet/minecraft/world/level/block/ColorCollection;Lnet/minecraft/world/item/DyeColor;)V
private static synthetic lambda$bootstrap$36()Lnet/minecraft/world/item/ItemStack;
private static synthetic lambda$bootstrap$33(Lnet/minecraft/world/item/CreativeModeTab$ItemDisplayParameters;Lnet/minecraft/world/item/CreativeModeTab$Output;)V
private static synthetic lambda$bootstrap$34(Lnet/minecraft/world/item/CreativeModeTab$Output;Lnet/minecraft/world/item/CreativeModeTab$ItemDisplayParameters;Lnet/minecraft/core/HolderLookup$RegistryLookup;)V
private static synthetic lambda$bootstrap$35(Lnet/minecraft/core/Holder;)Z
private static synthetic lambda$bootstrap$32()Lnet/minecraft/world/item/ItemStack;
private static synthetic lambda$bootstrap$31(Lnet/minecraft/world/item/CreativeModeTab$ItemDisplayParameters;Lnet/minecraft/world/item/CreativeModeTab$Output;)V
private static synthetic lambda$bootstrap$30()Lnet/minecraft/world/item/ItemStack;
private static synthetic lambda$bootstrap$27(Lnet/minecraft/world/item/CreativeModeTab$ItemDisplayParameters;Lnet/minecraft/world/item/CreativeModeTab$Output;)V
private static synthetic lambda$bootstrap$29(Lnet/minecraft/world/item/CreativeModeTab$Output;Lnet/minecraft/core/HolderLookup$RegistryLookup;)V
private static synthetic lambda$bootstrap$28(Lnet/minecraft/world/item/CreativeModeTab$Output;Lnet/minecraft/world/item/Item;)V
private static synthetic lambda$bootstrap$26()Lnet/minecraft/world/item/ItemStack;
private static synthetic lambda$bootstrap$24(Lnet/minecraft/world/item/CreativeModeTab$ItemDisplayParameters;Lnet/minecraft/world/item/CreativeModeTab$Output;)V
private static synthetic lambda$bootstrap$25(Lnet/minecraft/world/item/CreativeModeTab$Output;Lnet/minecraft/world/item/CreativeModeTab$ItemDisplayParameters;Lnet/minecraft/core/HolderLookup$RegistryLookup;)V
private static synthetic lambda$bootstrap$23()Lnet/minecraft/world/item/ItemStack;
private static synthetic lambda$bootstrap$21(Lnet/minecraft/world/item/CreativeModeTab$ItemDisplayParameters;Lnet/minecraft/world/item/CreativeModeTab$Output;)V
private static synthetic lambda$bootstrap$22(Lnet/minecraft/world/item/CreativeModeTab$Output;Lnet/minecraft/world/item/CreativeModeTab$ItemDisplayParameters;Lnet/minecraft/core/HolderLookup$RegistryLookup;)V
private static synthetic lambda$bootstrap$20()Lnet/minecraft/world/item/ItemStack;
private static synthetic lambda$bootstrap$18(Lnet/minecraft/world/item/CreativeModeTab$ItemDisplayParameters;Lnet/minecraft/world/item/CreativeModeTab$Output;)V
private static synthetic lambda$bootstrap$19(Lnet/minecraft/world/item/CreativeModeTab$Output;Lnet/minecraft/core/HolderLookup$RegistryLookup;)V
private static synthetic lambda$bootstrap$17()Lnet/minecraft/world/item/ItemStack;
private static synthetic lambda$bootstrap$16(Lnet/minecraft/core/Registry;Lnet/minecraft/world/item/CreativeModeTab$ItemDisplayParameters;Lnet/minecraft/world/item/CreativeModeTab$Output;)V
private static synthetic lambda$bootstrap$15()Lnet/minecraft/world/item/ItemStack;
private static synthetic lambda$bootstrap$14()Lnet/minecraft/world/item/ItemStack;
private static synthetic lambda$bootstrap$13(Lnet/minecraft/world/item/CreativeModeTab$ItemDisplayParameters;Lnet/minecraft/world/item/CreativeModeTab$Output;)V
private static synthetic lambda$bootstrap$12()Lnet/minecraft/world/item/ItemStack;
private static synthetic lambda$bootstrap$9(Ljava/util/List;Lnet/minecraft/world/item/CreativeModeTab$ItemDisplayParameters;Lnet/minecraft/world/item/CreativeModeTab$Output;)V
private static synthetic lambda$bootstrap$10(Lnet/minecraft/world/item/CreativeModeTab$Output;Lnet/minecraft/world/item/CreativeModeTab$ItemDisplayParameters;Lnet/minecraft/core/HolderLookup$RegistryLookup;)V
private static synthetic lambda$bootstrap$11(Lnet/minecraft/core/Holder;)Z
private static synthetic lambda$bootstrap$8()Lnet/minecraft/world/item/ItemStack;
private static synthetic lambda$bootstrap$7(Lnet/minecraft/world/item/CreativeModeTab$ItemDisplayParameters;Lnet/minecraft/world/item/CreativeModeTab$Output;)V
private static synthetic lambda$bootstrap$6()Lnet/minecraft/world/item/ItemStack;
private static synthetic lambda$bootstrap$5(Ljava/util/List;Lnet/minecraft/world/item/CreativeModeTab$ItemDisplayParameters;Lnet/minecraft/world/item/CreativeModeTab$Output;)V
private static synthetic lambda$bootstrap$4()Lnet/minecraft/world/item/ItemStack;
private static synthetic lambda$bootstrap$1(Lnet/minecraft/world/item/CreativeModeTab$ItemDisplayParameters;Lnet/minecraft/world/item/CreativeModeTab$Output;)V
private static synthetic lambda$bootstrap$3(Lnet/minecraft/world/item/CreativeModeTab$Output;Lnet/minecraft/world/level/block/WeatheringCopperCollection;)V
private static synthetic lambda$bootstrap$2(Lnet/minecraft/world/item/CreativeModeTab$Output;Lnet/minecraft/world/level/block/WeatheringCopperCollection;)V
private static synthetic lambda$bootstrap$0()Lnet/minecraft/world/item/ItemStack;
static <clinit>()V
```
