---
type: "interface"
fqcn: "net.minecraft.world.item.CreativeModeTabs"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.CreativeModeTabs

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `allTabs()Ljava/util/List;` | `` | client | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `tabs()Ljava/util/List;` | `` | client | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `tabs()Ljava/util/List;` | `` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `validate()V` | `` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `validate()V` | `` | client | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `buildAllTabContents` | `@Inject at TAIL` | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| injects_into | `validate` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `BUILDING_BLOCKSLnet/minecraft/resources/ResourceKey;` | `` | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `CACHED_PARAMETERSLnet/minecraft/world/item/CreativeModeTab$ItemDisplayParamet` | `` | client | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `COLORED_BLOCKSLnet/minecraft/resources/ResourceKey;` | `` | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `COMBATLnet/minecraft/resources/ResourceKey;` | `` | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `FOOD_AND_DRINKSLnet/minecraft/resources/ResourceKey;` | `` | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `FUNCTIONAL_BLOCKSLnet/minecraft/resources/ResourceKey;` | `` | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `HOTBARLnet/minecraft/resources/ResourceKey;` | `` | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `INGREDIENTSLnet/minecraft/resources/ResourceKey;` | `` | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `INVENTORYLnet/minecraft/resources/ResourceKey;` | `` | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `NATURAL_BLOCKSLnet/minecraft/resources/ResourceKey;` | `` | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `OP_BLOCKSLnet/minecraft/resources/ResourceKey;` | `` | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `OP_BLOCKSLnet/minecraft/resources/ResourceKey;` | `` | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `REDSTONE_BLOCKSLnet/minecraft/resources/ResourceKey;` | `` | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `SEARCHLnet/minecraft/resources/ResourceKey;` | `` | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `SPAWN_EGGSLnet/minecraft/resources/ResourceKey;` | `` | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `TOOLS_AND_UTILITIESLnet/minecraft/resources/ResourceKey;` | `` | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |

## Declared members (94, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.item.CreativeModeTabs {
    private static final net.minecraft.resources.Identifier INVENTORY_BACKGROUND;
    private static final net.minecraft.resources.Identifier SEARCH_BACKGROUND;
    private static final net.minecraft.resources.ResourceKey<net.minecraft.world.item.CreativeModeTab> BUILDING_BLOCKS;
    private static final net.minecraft.resources.ResourceKey<net.minecraft.world.item.CreativeModeTab> COLORED_BLOCKS;
    private static final net.minecraft.resources.ResourceKey<net.minecraft.world.item.CreativeModeTab> NATURAL_BLOCKS;
    private static final net.minecraft.resources.ResourceKey<net.minecraft.world.item.CreativeModeTab> FUNCTIONAL_BLOCKS;
    private static final net.minecraft.resources.ResourceKey<net.minecraft.world.item.CreativeModeTab> REDSTONE_BLOCKS;
    private static final net.minecraft.resources.ResourceKey<net.minecraft.world.item.CreativeModeTab> HOTBAR;
    private static final net.minecraft.resources.ResourceKey<net.minecraft.world.item.CreativeModeTab> SEARCH;
    private static final net.minecraft.resources.ResourceKey<net.minecraft.world.item.CreativeModeTab> TOOLS_AND_UTILITIES;
    private static final net.minecraft.resources.ResourceKey<net.minecraft.world.item.CreativeModeTab> COMBAT;
    private static final net.minecraft.resources.ResourceKey<net.minecraft.world.item.CreativeModeTab> FOOD_AND_DRINKS;
    private static final net.minecraft.resources.ResourceKey<net.minecraft.world.item.CreativeModeTab> INGREDIENTS;
    private static final net.minecraft.resources.ResourceKey<net.minecraft.world.item.CreativeModeTab> SPAWN_EGGS;
    private static final net.minecraft.resources.ResourceKey<net.minecraft.world.item.CreativeModeTab> OP_BLOCKS;
    private static final net.minecraft.resources.ResourceKey<net.minecraft.world.item.CreativeModeTab> INVENTORY;
    private static final java.util.Comparator<net.minecraft.core.Holder<net.minecraft.world.entity.decoration.painting.PaintingVariant>> PAINTING_COMPARATOR;
    private static net.minecraft.world.item.CreativeModeTab$ItemDisplayParameters CACHED_PARAMETERS;
    public net.minecraft.world.item.CreativeModeTabs();
    private static net.minecraft.resources.ResourceKey<net.minecraft.world.item.CreativeModeTab> createKey(java.lang.String);
    public static net.minecraft.world.item.CreativeModeTab bootstrap(net.minecraft.core.Registry<net.minecraft.world.item.CreativeModeTab>);
    private static void registerColoredItems(net.minecraft.world.item.CreativeModeTab$Output, java.util.List<net.minecraft.world.item.DyeColor>, net.minecraft.world.level.block.ColorCollection<net.minecraft.world.item.Item>);
    public static void validate();
    public static net.minecraft.world.item.CreativeModeTab getDefaultTab();
    private static void generatePotionEffectTypes(net.minecraft.world.item.CreativeModeTab$Output, net.minecraft.core.HolderLookup<net.minecraft.world.item.alchemy.Potion>, net.minecraft.world.item.Item, net.minecraft.world.item.CreativeModeTab$TabVisibility, net.minecraft.world.flag.FeatureFlagSet);
    private static void generateEnchantmentBookTypesOnlyMaxLevel(net.minecraft.world.item.CreativeModeTab$Output, net.minecraft.core.HolderLookup<net.minecraft.world.item.enchantment.Enchantment>, net.minecraft.world.item.CreativeModeTab$TabVisibility);
    private static void generateEnchantmentBookTypesAllLevels(net.minecraft.world.item.CreativeModeTab$Output, net.minecraft.core.HolderLookup<net.minecraft.world.item.enchantment.Enchantment>, net.minecraft.world.item.CreativeModeTab$TabVisibility);
    private static void generateInstrumentTypes(net.minecraft.world.item.CreativeModeTab$Output, net.minecraft.core.HolderLookup<net.minecraft.world.item.Instrument>, net.minecraft.world.item.Item, net.minecraft.tags.TagKey<net.minecraft.world.item.Instrument>, net.minecraft.world.item.CreativeModeTab$TabVisibility);
    private static void generateSuspiciousStews(net.minecraft.world.item.CreativeModeTab$Output, net.minecraft.world.item.CreativeModeTab$TabVisibility);
    private static void generateOminousBottles(net.minecraft.world.item.CreativeModeTab$Output, net.minecraft.world.item.CreativeModeTab$TabVisibility);
    private static void generateFireworksAllDurations(net.minecraft.world.item.CreativeModeTab$Output, net.minecraft.world.item.CreativeModeTab$TabVisibility);
    private static void generatePresetPaintings(net.minecraft.world.item.CreativeModeTab$Output, net.minecraft.core.HolderLookup$Provider, net.minecraft.core.HolderLookup$RegistryLookup<net.minecraft.world.entity.decoration.painting.PaintingVariant>, java.util.function.Predicate<net.minecraft.core.Holder<net.minecraft.world.entity.decoration.painting.PaintingVariant>>, net.minecraft.world.item.CreativeModeTab$TabVisibility);
    private static void copperBlockFamilies(java.util.function.Consumer<net.minecraft.world.level.block.WeatheringCopperCollection<net.minecraft.world.item.Item>>);
    public static java.util.List<net.minecraft.world.item.CreativeModeTab> tabs();
    public static java.util.List<net.minecraft.world.item.CreativeModeTab> allTabs();
    private static java.util.stream.Stream<net.minecraft.world.item.CreativeModeTab> streamAllTabs();
    public static net.minecraft.world.item.CreativeModeTab searchTab();
    private static void buildAllTabContents(net.minecraft.world.item.CreativeModeTab$ItemDisplayParameters);
    public static boolean tryRebuildTabContents(net.minecraft.world.flag.FeatureFlagSet, boolean, net.minecraft.core.HolderLookup$Provider);
    private static void lambda$buildAllTabContents$3(net.minecraft.world.item.CreativeModeTab$ItemDisplayParameters, net.minecraft.world.item.CreativeModeTab);
    private static boolean lambda$buildAllTabContents$2(net.minecraft.world.item.CreativeModeTab);
    private static void lambda$buildAllTabContents$1(net.minecraft.world.item.CreativeModeTab$ItemDisplayParameters, net.minecraft.world.item.CreativeModeTab);
    private static boolean lambda$buildAllTabContents$0(net.minecraft.world.item.CreativeModeTab);
    private static void lambda$generatePresetPaintings$0(net.minecraft.world.item.CreativeModeTab$Output, net.minecraft.world.item.CreativeModeTab$TabVisibility, net.minecraft.core.Holder$Reference);
    private static void lambda$generateInstrumentTypes$0(net.minecraft.world.item.Item, net.minecraft.world.item.CreativeModeTab$Output, net.minecraft.world.item.CreativeModeTab$TabVisibility, net.minecraft.core.HolderSet$Named);
    private static void lambda$generateInstrumentTypes$2(net.minecraft.world.item.CreativeModeTab$Output, net.minecraft.world.item.CreativeModeTab$TabVisibility, net.minecraft.world.item.ItemStack);
    private static net.minecraft.world.item.ItemStack lambda$generateInstrumentTypes$1(net.minecraft.world.item.Item, net.minecraft.core.Holder);
    private static void lambda$generateEnchantmentBookTypesAllLevels$2(net.minecraft.world.item.CreativeModeTab$Output, net.minecraft.world.item.CreativeModeTab$TabVisibility, net.minecraft.world.item.ItemStack);
    private static java.util.stream.Stream lambda$generateEnchantmentBookTypesAllLevels$0(net.minecraft.core.Holder$Reference);
    private static net.minecraft.world.item.ItemStack lambda$generateEnchantmentBookTypesAllLevels$1(net.minecraft.core.Holder$Reference, int);
    private static void lambda$generateEnchantmentBookTypesOnlyMaxLevel$1(net.minecraft.world.item.CreativeModeTab$Output, net.minecraft.world.item.CreativeModeTab$TabVisibility, net.minecraft.world.item.ItemStack);
    private static net.minecraft.world.item.ItemStack lambda$generateEnchantmentBookTypesOnlyMaxLevel$0(net.minecraft.core.Holder$Reference);
    private static void lambda$generatePotionEffectTypes$2(net.minecraft.world.item.CreativeModeTab$Output, net.minecraft.world.item.CreativeModeTab$TabVisibility, net.minecraft.world.item.ItemStack);
    private static net.minecraft.world.item.ItemStack lambda$generatePotionEffectTypes$1(net.minecraft.world.item.Item, net.minecraft.core.Holder$Reference);
    private static boolean lambda$generatePotionEffectTypes$0(net.minecraft.world.flag.FeatureFlagSet, net.minecraft.core.Holder$Reference);
    private static void lambda$registerColoredItems$0(net.minecraft.world.item.CreativeModeTab$Output, net.minecraft.world.level.block.ColorCollection, net.minecraft.world.item.DyeColor);
    private static net.minecraft.world.item.ItemStack lambda$bootstrap$36();
    private static void lambda$bootstrap$33(net.minecraft.world.item.CreativeModeTab$ItemDisplayParameters, net.minecraft.world.item.CreativeModeTab$Output);
    private static void lambda$bootstrap$34(net.minecraft.world.item.CreativeModeTab$Output, net.minecraft.world.item.CreativeModeTab$ItemDisplayParameters, net.minecraft.core.HolderLookup$RegistryLookup);
    private static boolean lambda$bootstrap$35(net.minecraft.core.Holder);
    private static net.minecraft.world.item.ItemStack lambda$bootstrap$32();
    private static void lambda$bootstrap$31(net.minecraft.world.item.CreativeModeTab$ItemDisplayParameters, net.minecraft.world.item.CreativeModeTab$Output);
    private static net.minecraft.world.item.ItemStack lambda$bootstrap$30();
    private static void lambda$bootstrap$27(net.minecraft.world.item.CreativeModeTab$ItemDisplayParameters, net.minecraft.world.item.CreativeModeTab$Output);
    private static void lambda$bootstrap$29(net.minecraft.world.item.CreativeModeTab$Output, net.minecraft.core.HolderLookup$RegistryLookup);
    private static void lambda$bootstrap$28(net.minecraft.world.item.CreativeModeTab$Output, net.minecraft.world.item.Item);
    private static net.minecraft.world.item.ItemStack lambda$bootstrap$26();
    private static void lambda$bootstrap$24(net.minecraft.world.item.CreativeModeTab$ItemDisplayParameters, net.minecraft.world.item.CreativeModeTab$Output);
    private static void lambda$bootstrap$25(net.minecraft.world.item.CreativeModeTab$Output, net.minecraft.world.item.CreativeModeTab$ItemDisplayParameters, net.minecraft.core.HolderLookup$RegistryLookup);
    private static net.minecraft.world.item.ItemStack lambda$bootstrap$23();
    private static void lambda$bootstrap$21(net.minecraft.world.item.CreativeModeTab$ItemDisplayParameters, net.minecraft.world.item.CreativeModeTab$Output);
    private static void lambda$bootstrap$22(net.minecraft.world.item.CreativeModeTab$Output, net.minecraft.world.item.CreativeModeTab$ItemDisplayParameters, net.minecraft.core.HolderLookup$RegistryLookup);
    private static net.minecraft.world.item.ItemStack lambda$bootstrap$20();
    private static void lambda$bootstrap$18(net.minecraft.world.item.CreativeModeTab$ItemDisplayParameters, net.minecraft.world.item.CreativeModeTab$Output);
    private static void lambda$bootstrap$19(net.minecraft.world.item.CreativeModeTab$Output, net.minecraft.core.HolderLookup$RegistryLookup);
    private static net.minecraft.world.item.ItemStack lambda$bootstrap$17();
    private static void lambda$bootstrap$16(net.minecraft.core.Registry, net.minecraft.world.item.CreativeModeTab$ItemDisplayParameters, net.minecraft.world.item.CreativeModeTab$Output);
    private static net.minecraft.world.item.ItemStack lambda$bootstrap$15();
    private static net.minecraft.world.item.ItemStack lambda$bootstrap$14();
    private static void lambda$bootstrap$13(net.minecraft.world.item.CreativeModeTab$ItemDisplayParameters, net.minecraft.world.item.CreativeModeTab$Output);
    private static net.minecraft.world.item.ItemStack lambda$bootstrap$12();
    private static void lambda$bootstrap$9(java.util.List, net.minecraft.world.item.CreativeModeTab$ItemDisplayParameters, net.minecraft.world.item.CreativeModeTab$Output);
    private static void lambda$bootstrap$10(net.minecraft.world.item.CreativeModeTab$Output, net.minecraft.world.item.CreativeModeTab$ItemDisplayParameters, net.minecraft.core.HolderLookup$RegistryLookup);
    private static boolean lambda$bootstrap$11(net.minecraft.core.Holder);
    private static net.minecraft.world.item.ItemStack lambda$bootstrap$8();
    private static void lambda$bootstrap$7(net.minecraft.world.item.CreativeModeTab$ItemDisplayParameters, net.minecraft.world.item.CreativeModeTab$Output);
    private static net.minecraft.world.item.ItemStack lambda$bootstrap$6();
    private static void lambda$bootstrap$5(java.util.List, net.minecraft.world.item.CreativeModeTab$ItemDisplayParameters, net.minecraft.world.item.CreativeModeTab$Output);
    private static net.minecraft.world.item.ItemStack lambda$bootstrap$4();
    private static void lambda$bootstrap$1(net.minecraft.world.item.CreativeModeTab$ItemDisplayParameters, net.minecraft.world.item.CreativeModeTab$Output);
    private static void lambda$bootstrap$3(net.minecraft.world.item.CreativeModeTab$Output, net.minecraft.world.level.block.WeatheringCopperCollection);
    private static void lambda$bootstrap$2(net.minecraft.world.item.CreativeModeTab$Output, net.minecraft.world.level.block.WeatheringCopperCollection);
    private static net.minecraft.world.item.ItemStack lambda$bootstrap$0();
    static {};
}
```
