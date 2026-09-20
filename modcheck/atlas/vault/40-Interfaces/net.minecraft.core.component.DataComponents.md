---
type: "interface"
fqcn: "net.minecraft.core.component.DataComponents"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.component.DataComponents

System: [[20-Systems/net.minecraft.core.component|net.minecraft.core.component]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `ATTRIBUTE_MODIFIERS` | `Lnet/minecraft/core/component/DataComponentType;` | exact | getstatic@432 in `VanillaTooltipProviderOrder.scrapeVanillaOrder` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| reads | `ATTRIBUTE_MODIFIERS` | `Lnet/minecraft/core/component/DataComponentType;` | exact | getstatic@1 in `ItemStackMixin.preAttributeModifiers` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| reads | `BUNDLE_CONTENTS` | `Lnet/minecraft/core/component/DataComponentType;` | exact | getstatic@159 in `BundleContentsStorage$BundleSlotWrapper.extract` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `BUNDLE_CONTENTS` | `Lnet/minecraft/core/component/DataComponentType;` | exact | getstatic@74 in `BundleContentsStorage.insert` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `BUNDLE_CONTENTS` | `Lnet/minecraft/core/component/DataComponentType;` | exact | getstatic@14 in `BundleContentsStorage.bundleContents` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `COMPOSTABLE` | `Lnet/minecraft/core/component/DataComponentType;` | exact | getstatic@63 in `ComposterWrapper$TopStorage.insert` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `CONTAINER` | `Lnet/minecraft/core/component/DataComponentType;` | exact | getstatic@80 in `ItemContainerContentsStorage$ContainerSlotWrapper.setStack` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `CONTAINER` | `Lnet/minecraft/core/component/DataComponentType;` | exact | getstatic@14 in `ItemContainerContentsStorage.container` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `CUSTOM_DATA` | `Lnet/minecraft/core/component/DataComponentType;` | exact | getstatic@14 in `CustomDataIngredient.test` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| reads | `CUSTOM_DATA` | `Lnet/minecraft/core/component/DataComponentType;` | exact | getstatic@3 in `CustomDataIngredient.createEntryDisplay` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| reads | `MAX_STACK_SIZE` | `Lnet/minecraft/core/component/DataComponentType;` | exact | getstatic@6 in `ItemVariantImpl.getMaxStackSize` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `POTION_CONTENTS` | `Lnet/minecraft/core/component/DataComponentType;` | exact | getstatic@20 in `FabricItem.getCreatorNamespace` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| reads | `POTION_CONTENTS` | `Lnet/minecraft/core/component/DataComponentType;` | exact | getstatic@30 in `FabricItem.getCreatorNamespace` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| reads | `POTION_CONTENTS` | `Lnet/minecraft/core/component/DataComponentType;` | exact | getstatic@8 in `FluidStorage.lambda$static$4` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `POTION_CONTENTS` | `Lnet/minecraft/core/component/DataComponentType;` | exact | getstatic@13 in `WaterPotionStorage.isWaterPotion` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `POTION_CONTENTS` | `Lnet/minecraft/core/component/DataComponentType;` | exact | getstatic@16 in `WaterPotionStorage.mapToGlassBottle` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `STORED_ENCHANTMENTS` | `Lnet/minecraft/core/component/DataComponentType;` | exact | getstatic@72 in `FabricItem.getCreatorNamespace` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| reads | `STORED_ENCHANTMENTS` | `Lnet/minecraft/core/component/DataComponentType;` | exact | getstatic@82 in `FabricItem.getCreatorNamespace` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (124 fields, 126 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
static final ENCODER_CACHE : Lnet/minecraft/util/EncoderCache;
public static final CUSTOM_DATA : Lnet/minecraft/core/component/DataComponentType;
public static final MAX_STACK_SIZE : Lnet/minecraft/core/component/DataComponentType;
public static final MAX_DAMAGE : Lnet/minecraft/core/component/DataComponentType;
public static final DAMAGE : Lnet/minecraft/core/component/DataComponentType;
public static final UNBREAKABLE : Lnet/minecraft/core/component/DataComponentType;
public static final USE_EFFECTS : Lnet/minecraft/core/component/DataComponentType;
public static final CUSTOM_NAME : Lnet/minecraft/core/component/DataComponentType;
public static final MINIMUM_ATTACK_CHARGE : Lnet/minecraft/core/component/DataComponentType;
public static final DAMAGE_TYPE : Lnet/minecraft/core/component/DataComponentType;
public static final ITEM_NAME : Lnet/minecraft/core/component/DataComponentType;
public static final ITEM_MODEL : Lnet/minecraft/core/component/DataComponentType;
public static final LORE : Lnet/minecraft/core/component/DataComponentType;
public static final RARITY : Lnet/minecraft/core/component/DataComponentType;
public static final ENCHANTMENTS : Lnet/minecraft/core/component/DataComponentType;
public static final CAN_PLACE_ON : Lnet/minecraft/core/component/DataComponentType;
public static final CAN_BREAK : Lnet/minecraft/core/component/DataComponentType;
public static final ATTRIBUTE_MODIFIERS : Lnet/minecraft/core/component/DataComponentType;
public static final CUSTOM_MODEL_DATA : Lnet/minecraft/core/component/DataComponentType;
public static final TOOLTIP_DISPLAY : Lnet/minecraft/core/component/DataComponentType;
public static final REPAIR_COST : Lnet/minecraft/core/component/DataComponentType;
public static final CREATIVE_SLOT_LOCK : Lnet/minecraft/core/component/DataComponentType;
public static final ENCHANTMENT_GLINT_OVERRIDE : Lnet/minecraft/core/component/DataComponentType;
public static final INTANGIBLE_PROJECTILE : Lnet/minecraft/core/component/DataComponentType;
public static final FOOD : Lnet/minecraft/core/component/DataComponentType;
public static final CONSUMABLE : Lnet/minecraft/core/component/DataComponentType;
public static final USE_REMAINDER : Lnet/minecraft/core/component/DataComponentType;
public static final USE_COOLDOWN : Lnet/minecraft/core/component/DataComponentType;
public static final DAMAGE_RESISTANT : Lnet/minecraft/core/component/DataComponentType;
public static final TOOL : Lnet/minecraft/core/component/DataComponentType;
public static final WEAPON : Lnet/minecraft/core/component/DataComponentType;
public static final ATTACK_RANGE : Lnet/minecraft/core/component/DataComponentType;
public static final ENCHANTABLE : Lnet/minecraft/core/component/DataComponentType;
public static final EQUIPPABLE : Lnet/minecraft/core/component/DataComponentType;
public static final REPAIRABLE : Lnet/minecraft/core/component/DataComponentType;
public static final GLIDER : Lnet/minecraft/core/component/DataComponentType;
public static final TOOLTIP_STYLE : Lnet/minecraft/core/component/DataComponentType;
public static final DEATH_PROTECTION : Lnet/minecraft/core/component/DataComponentType;
public static final BLOCKS_ATTACKS : Lnet/minecraft/core/component/DataComponentType;
public static final PIERCING_WEAPON : Lnet/minecraft/core/component/DataComponentType;
public static final KINETIC_WEAPON : Lnet/minecraft/core/component/DataComponentType;
public static final ATTACK_ANIMATION : Lnet/minecraft/core/component/DataComponentType;
public static final INTERACT_ANIMATION : Lnet/minecraft/core/component/DataComponentType;
public static final ADDITIONAL_TRADE_COST : Lnet/minecraft/core/component/DataComponentType;
public static final BLOCK_TRANSFORMER : Lnet/minecraft/core/component/DataComponentType;
public static final VILLAGER_FOOD : Lnet/minecraft/core/component/DataComponentType;
public static final STORED_ENCHANTMENTS : Lnet/minecraft/core/component/DataComponentType;
public static final DYE : Lnet/minecraft/core/component/DataComponentType;
public static final DYED_COLOR : Lnet/minecraft/core/component/DataComponentType;
public static final MAP_ID : Lnet/minecraft/core/component/DataComponentType;
public static final MAP_DECORATIONS : Lnet/minecraft/core/component/DataComponentType;
public static final MAP_POST_PROCESSING : Lnet/minecraft/core/component/DataComponentType;
public static final CHARGED_PROJECTILES : Lnet/minecraft/core/component/DataComponentType;
public static final BUNDLE_CONTENTS : Lnet/minecraft/core/component/DataComponentType;
public static final POTION_CONTENTS : Lnet/minecraft/core/component/DataComponentType;
public static final POTION_DURATION_SCALE : Lnet/minecraft/core/component/DataComponentType;
public static final SUSPICIOUS_STEW_EFFECTS : Lnet/minecraft/core/component/DataComponentType;
public static final WRITABLE_BOOK_CONTENT : Lnet/minecraft/core/component/DataComponentType;
public static final WRITTEN_BOOK_CONTENT : Lnet/minecraft/core/component/DataComponentType;
public static final TRIM : Lnet/minecraft/core/component/DataComponentType;
public static final DEBUG_STICK_STATE : Lnet/minecraft/core/component/DataComponentType;
public static final ENTITY_DATA : Lnet/minecraft/core/component/DataComponentType;
public static final BUCKET_ENTITY_DATA : Lnet/minecraft/core/component/DataComponentType;
public static final BLOCK_ENTITY_DATA : Lnet/minecraft/core/component/DataComponentType;
public static final INSTRUMENT : Lnet/minecraft/core/component/DataComponentType;
public static final PROVIDES_TRIM_MATERIAL : Lnet/minecraft/core/component/DataComponentType;
public static final OMINOUS_BOTTLE_AMPLIFIER : Lnet/minecraft/core/component/DataComponentType;
public static final JUKEBOX_PLAYABLE : Lnet/minecraft/core/component/DataComponentType;
public static final PROVIDES_BANNER_PATTERNS : Lnet/minecraft/core/component/DataComponentType;
public static final RECIPES : Lnet/minecraft/core/component/DataComponentType;
public static final LODESTONE_TRACKER : Lnet/minecraft/core/component/DataComponentType;
public static final FIREWORK_EXPLOSION : Lnet/minecraft/core/component/DataComponentType;
public static final FIREWORKS : Lnet/minecraft/core/component/DataComponentType;
public static final PROFILE : Lnet/minecraft/core/component/DataComponentType;
public static final NOTE_BLOCK_SOUND : Lnet/minecraft/core/component/DataComponentType;
public static final BANNER_PATTERNS : Lnet/minecraft/core/component/DataComponentType;
public static final BASE_COLOR : Lnet/minecraft/core/component/DataComponentType;
public static final POT_DECORATIONS : Lnet/minecraft/core/component/DataComponentType;
public static final CONTAINER : Lnet/minecraft/core/component/DataComponentType;
public static final BLOCK_STATE : Lnet/minecraft/core/component/DataComponentType;
public static final BEES : Lnet/minecraft/core/component/DataComponentType;
public static final SULFUR_CUBE_CONTENT : Lnet/minecraft/core/component/DataComponentType;
public static final LOCK : Lnet/minecraft/core/component/DataComponentType;
public static final CONTAINER_LOOT : Lnet/minecraft/core/component/DataComponentType;
public static final BREAK_SOUND : Lnet/minecraft/core/component/DataComponentType;
public static final COMPOSTABLE : Lnet/minecraft/core/component/DataComponentType;
public static final COOKING_FUEL : Lnet/minecraft/core/component/DataComponentType;
public static final BREWING_FUEL : Lnet/minecraft/core/component/DataComponentType;
public static final MOB_VISIBILITY : Lnet/minecraft/core/component/DataComponentType;
public static final VILLAGER_VARIANT : Lnet/minecraft/core/component/DataComponentType;
public static final WOLF_VARIANT : Lnet/minecraft/core/component/DataComponentType;
public static final WOLF_SOUND_VARIANT : Lnet/minecraft/core/component/DataComponentType;
public static final WOLF_COLLAR : Lnet/minecraft/core/component/DataComponentType;
public static final FOX_VARIANT : Lnet/minecraft/core/component/DataComponentType;
public static final SALMON_SIZE : Lnet/minecraft/core/component/DataComponentType;
public static final PARROT_VARIANT : Lnet/minecraft/core/component/DataComponentType;
public static final TROPICAL_FISH_PATTERN : Lnet/minecraft/core/component/DataComponentType;
public static final TROPICAL_FISH_BASE_COLOR : Lnet/minecraft/core/component/DataComponentType;
public static final TROPICAL_FISH_PATTERN_COLOR : Lnet/minecraft/core/component/DataComponentType;
public static final MOOSHROOM_VARIANT : Lnet/minecraft/core/component/DataComponentType;
public static final RABBIT_VARIANT : Lnet/minecraft/core/component/DataComponentType;
public static final PIG_VARIANT : Lnet/minecraft/core/component/DataComponentType;
public static final PIG_SOUND_VARIANT : Lnet/minecraft/core/component/DataComponentType;
public static final COW_VARIANT : Lnet/minecraft/core/component/DataComponentType;
public static final COW_SOUND_VARIANT : Lnet/minecraft/core/component/DataComponentType;
public static final CHICKEN_VARIANT : Lnet/minecraft/core/component/DataComponentType;
public static final CHICKEN_SOUND_VARIANT : Lnet/minecraft/core/component/DataComponentType;
public static final ZOMBIE_NAUTILUS_VARIANT : Lnet/minecraft/core/component/DataComponentType;
public static final FROG_VARIANT : Lnet/minecraft/core/component/DataComponentType;
public static final HORSE_VARIANT : Lnet/minecraft/core/component/DataComponentType;
public static final PAINTING_VARIANT : Lnet/minecraft/core/component/DataComponentType;
public static final LLAMA_VARIANT : Lnet/minecraft/core/component/DataComponentType;
public static final AXOLOTL_VARIANT : Lnet/minecraft/core/component/DataComponentType;
public static final CAT_VARIANT : Lnet/minecraft/core/component/DataComponentType;
public static final CAT_SOUND_VARIANT : Lnet/minecraft/core/component/DataComponentType;
public static final CAT_COLLAR : Lnet/minecraft/core/component/DataComponentType;
public static final SHEEP_COLOR : Lnet/minecraft/core/component/DataComponentType;
public static final SHULKER_COLOR : Lnet/minecraft/core/component/DataComponentType;
public static final PROVIDES_POTTERY_PATTERN : Lnet/minecraft/core/component/DataComponentType;
public static final SIGN_TEXT_FRONT : Lnet/minecraft/core/component/DataComponentType;
public static final SIGN_TEXT_BACK : Lnet/minecraft/core/component/DataComponentType;
public static final WAXED : Lnet/minecraft/core/component/DataComponentType;
public static final CUSHION_COLOR : Lnet/minecraft/core/component/DataComponentType;
public static final COMMON_ITEM_COMPONENTS : Lnet/minecraft/core/component/DataComponentMap;
public <init>()V
public static bootstrap(Lnet/minecraft/core/Registry;)Lnet/minecraft/core/component/DataComponentType;
private static register(Ljava/lang/String;Ljava/util/function/UnaryOperator;)Lnet/minecraft/core/component/DataComponentType;
private static synthetic lambda$static$121(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$120(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$119(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$118(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$117(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$116(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$115(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$114(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$113(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$112(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$111(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$110(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$109(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$108(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$107(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$106(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$105(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$104(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$103(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$102(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$101(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$100(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$99(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$98(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$97(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$96(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$95(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$94(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$93(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$92(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$91(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$90(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$89(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$88(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$87(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$86(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$85(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$84(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$83(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$82(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$81(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$80(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$79(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$78(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$77(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$76(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$75(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$74(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$73(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$72(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$71(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$70(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$69(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$68(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$67(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$66(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$65(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$64(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$63(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$62(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$61(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$60(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$59(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$58(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$57(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$56(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$55(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$54(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$53(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$52(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$51(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$50(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$49(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$48(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$47(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$46(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$45(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$44(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$43(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$42(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$41(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$40(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$39(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$38(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$37(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$36(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$35(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$34(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$33(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$32(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$31(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$30(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$29(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$28(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$27(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$26(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$25(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$24(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$23(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$22(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$21(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$20(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$19(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$18(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$17(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$16(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$15(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$14(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$13(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$12(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$11(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$10(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$9(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$8(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$7(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$6(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$5(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$4(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$3(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$2(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$1(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
private static synthetic lambda$static$0(Lnet/minecraft/core/component/DataComponentType$Builder;)Lnet/minecraft/core/component/DataComponentType$Builder;
static <clinit>()V
```
