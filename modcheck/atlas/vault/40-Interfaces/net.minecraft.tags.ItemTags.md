---
type: "interface"
fqcn: "net.minecraft.tags.ItemTags"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.tags.ItemTags

System: [[20-Systems/net.minecraft.tags|net.minecraft.tags]]

`class` public final; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `SKULLS` | `Lnet/minecraft/tags/TagKey;` | exact | getstatic@1084 in `ConventionalBlockItemTags.<clinit>` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |

## Declared members (219 fields, 3 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final WOOL : Lnet/minecraft/tags/TagKey;
public static final PLANKS : Lnet/minecraft/tags/TagKey;
public static final STONE_BRICKS : Lnet/minecraft/tags/TagKey;
public static final WOODEN_BUTTONS : Lnet/minecraft/tags/TagKey;
public static final WOOL_CARPETS : Lnet/minecraft/tags/TagKey;
public static final WOOL_STAIRS : Lnet/minecraft/tags/TagKey;
public static final WOOL_SLABS : Lnet/minecraft/tags/TagKey;
public static final WOODEN_DOORS : Lnet/minecraft/tags/TagKey;
public static final WOODEN_STAIRS : Lnet/minecraft/tags/TagKey;
public static final WOODEN_SLABS : Lnet/minecraft/tags/TagKey;
public static final WOODEN_FENCES : Lnet/minecraft/tags/TagKey;
public static final FENCE_GATES : Lnet/minecraft/tags/TagKey;
public static final WOODEN_PRESSURE_PLATES : Lnet/minecraft/tags/TagKey;
public static final WOODEN_SHELVES : Lnet/minecraft/tags/TagKey;
public static final SAPLINGS : Lnet/minecraft/tags/TagKey;
public static final BAMBOO_BLOCKS : Lnet/minecraft/tags/TagKey;
public static final OAK_LOGS : Lnet/minecraft/tags/TagKey;
public static final DARK_OAK_LOGS : Lnet/minecraft/tags/TagKey;
public static final PALE_OAK_LOGS : Lnet/minecraft/tags/TagKey;
public static final BIRCH_LOGS : Lnet/minecraft/tags/TagKey;
public static final ACACIA_LOGS : Lnet/minecraft/tags/TagKey;
public static final SPRUCE_LOGS : Lnet/minecraft/tags/TagKey;
public static final MANGROVE_LOGS : Lnet/minecraft/tags/TagKey;
public static final POPLAR_LOGS : Lnet/minecraft/tags/TagKey;
public static final JUNGLE_LOGS : Lnet/minecraft/tags/TagKey;
public static final CHERRY_LOGS : Lnet/minecraft/tags/TagKey;
public static final CRIMSON_STEMS : Lnet/minecraft/tags/TagKey;
public static final WARPED_STEMS : Lnet/minecraft/tags/TagKey;
public static final WART_BLOCKS : Lnet/minecraft/tags/TagKey;
public static final LOGS_THAT_BURN : Lnet/minecraft/tags/TagKey;
public static final LOGS : Lnet/minecraft/tags/TagKey;
public static final SAND : Lnet/minecraft/tags/TagKey;
public static final SMELTS_TO_GLASS : Lnet/minecraft/tags/TagKey;
public static final WALLS : Lnet/minecraft/tags/TagKey;
public static final ANVIL : Lnet/minecraft/tags/TagKey;
public static final RAILS : Lnet/minecraft/tags/TagKey;
public static final LEAVES : Lnet/minecraft/tags/TagKey;
public static final WOODEN_TRAPDOORS : Lnet/minecraft/tags/TagKey;
public static final BEDS : Lnet/minecraft/tags/TagKey;
public static final SOUL_FIRE_BASE_BLOCKS : Lnet/minecraft/tags/TagKey;
public static final CANDLES : Lnet/minecraft/tags/TagKey;
public static final DAMPENS_VIBRATIONS : Lnet/minecraft/tags/TagKey;
public static final GOLD_ORES : Lnet/minecraft/tags/TagKey;
public static final IRON_ORES : Lnet/minecraft/tags/TagKey;
public static final DIAMOND_ORES : Lnet/minecraft/tags/TagKey;
public static final REDSTONE_ORES : Lnet/minecraft/tags/TagKey;
public static final LAPIS_ORES : Lnet/minecraft/tags/TagKey;
public static final COAL_ORES : Lnet/minecraft/tags/TagKey;
public static final EMERALD_ORES : Lnet/minecraft/tags/TagKey;
public static final COPPER_ORES : Lnet/minecraft/tags/TagKey;
public static final DIRT : Lnet/minecraft/tags/TagKey;
public static final MUD : Lnet/minecraft/tags/TagKey;
public static final MOSS_BLOCKS : Lnet/minecraft/tags/TagKey;
public static final TERRACOTTA : Lnet/minecraft/tags/TagKey;
public static final GLAZED_TERRACOTTA : Lnet/minecraft/tags/TagKey;
public static final CONCRETE : Lnet/minecraft/tags/TagKey;
public static final CONCRETE_STAIRS : Lnet/minecraft/tags/TagKey;
public static final CONCRETE_SLABS : Lnet/minecraft/tags/TagKey;
public static final CONCRETE_POWDERS : Lnet/minecraft/tags/TagKey;
public static final COMPLETES_FIND_TREE_TUTORIAL : Lnet/minecraft/tags/TagKey;
public static final SHULKER_BOXES : Lnet/minecraft/tags/TagKey;
public static final COPPER : Lnet/minecraft/tags/TagKey;
public static final SKULLS : Lnet/minecraft/tags/TagKey;
public static final SIGNS : Lnet/minecraft/tags/TagKey;
public static final HANGING_SIGNS : Lnet/minecraft/tags/TagKey;
public static final BEE_FOOD : Lnet/minecraft/tags/TagKey;
public static final SULFUR_CUBE_ARCHETYPE_BOUNCY : Lnet/minecraft/tags/TagKey;
public static final SULFUR_CUBE_ARCHETYPE_SLOW_BOUNCY : Lnet/minecraft/tags/TagKey;
public static final SULFUR_CUBE_ARCHETYPE_REGULAR : Lnet/minecraft/tags/TagKey;
public static final SULFUR_CUBE_ARCHETYPE_SLOW_FLAT : Lnet/minecraft/tags/TagKey;
public static final SULFUR_CUBE_ARCHETYPE_FAST_FLAT : Lnet/minecraft/tags/TagKey;
public static final SULFUR_CUBE_ARCHETYPE_LIGHT : Lnet/minecraft/tags/TagKey;
public static final SULFUR_CUBE_ARCHETYPE_FAST_SLIDING : Lnet/minecraft/tags/TagKey;
public static final SULFUR_CUBE_ARCHETYPE_SLOW_SLIDING : Lnet/minecraft/tags/TagKey;
public static final SULFUR_CUBE_ARCHETYPE_STICKY : Lnet/minecraft/tags/TagKey;
public static final SULFUR_CUBE_ARCHETYPE_HIGH_RESISTANCE : Lnet/minecraft/tags/TagKey;
public static final SULFUR_CUBE_ARCHETYPE_EXPLOSIVE : Lnet/minecraft/tags/TagKey;
public static final SULFUR_CUBE_ARCHETYPE_HOT : Lnet/minecraft/tags/TagKey;
public static final SULFUR_CUBE_SWALLOWABLE : Lnet/minecraft/tags/TagKey;
public static final BANNERS : Lnet/minecraft/tags/TagKey;
public static final PIGLIN_REPELLENTS : Lnet/minecraft/tags/TagKey;
public static final PIGLIN_LOVED : Lnet/minecraft/tags/TagKey;
public static final IGNORED_BY_PIGLIN_BABIES : Lnet/minecraft/tags/TagKey;
public static final PIGLIN_SAFE_ARMOR : Lnet/minecraft/tags/TagKey;
public static final DUPLICATES_ALLAYS : Lnet/minecraft/tags/TagKey;
public static final EGGS : Lnet/minecraft/tags/TagKey;
public static final MEAT : Lnet/minecraft/tags/TagKey;
public static final SNIFFER_FOOD : Lnet/minecraft/tags/TagKey;
public static final PIGLIN_FOOD : Lnet/minecraft/tags/TagKey;
public static final FOX_FOOD : Lnet/minecraft/tags/TagKey;
public static final COW_FOOD : Lnet/minecraft/tags/TagKey;
public static final GOAT_FOOD : Lnet/minecraft/tags/TagKey;
public static final SHEEP_FOOD : Lnet/minecraft/tags/TagKey;
public static final WOLF_FOOD : Lnet/minecraft/tags/TagKey;
public static final CAT_FOOD : Lnet/minecraft/tags/TagKey;
public static final HORSE_FOOD : Lnet/minecraft/tags/TagKey;
public static final ZOMBIE_HORSE_FOOD : Lnet/minecraft/tags/TagKey;
public static final HORSE_TEMPT_ITEMS : Lnet/minecraft/tags/TagKey;
public static final HARNESSES : Lnet/minecraft/tags/TagKey;
public static final HAPPY_GHAST_FOOD : Lnet/minecraft/tags/TagKey;
public static final HAPPY_GHAST_TEMPT_ITEMS : Lnet/minecraft/tags/TagKey;
public static final CAMEL_FOOD : Lnet/minecraft/tags/TagKey;
public static final CAMEL_HUSK_FOOD : Lnet/minecraft/tags/TagKey;
public static final ARMADILLO_FOOD : Lnet/minecraft/tags/TagKey;
public static final CHICKEN_FOOD : Lnet/minecraft/tags/TagKey;
public static final FROG_FOOD : Lnet/minecraft/tags/TagKey;
public static final HOGLIN_FOOD : Lnet/minecraft/tags/TagKey;
public static final LLAMA_FOOD : Lnet/minecraft/tags/TagKey;
public static final LLAMA_TEMPT_ITEMS : Lnet/minecraft/tags/TagKey;
public static final OCELOT_FOOD : Lnet/minecraft/tags/TagKey;
public static final PANDA_FOOD : Lnet/minecraft/tags/TagKey;
public static final PANDA_EATS_FROM_GROUND : Lnet/minecraft/tags/TagKey;
public static final PIG_FOOD : Lnet/minecraft/tags/TagKey;
public static final RABBIT_FOOD : Lnet/minecraft/tags/TagKey;
public static final STRIDER_FOOD : Lnet/minecraft/tags/TagKey;
public static final STRIDER_TEMPT_ITEMS : Lnet/minecraft/tags/TagKey;
public static final TURTLE_FOOD : Lnet/minecraft/tags/TagKey;
public static final PARROT_FOOD : Lnet/minecraft/tags/TagKey;
public static final PARROT_POISONOUS_FOOD : Lnet/minecraft/tags/TagKey;
public static final AXOLOTL_FOOD : Lnet/minecraft/tags/TagKey;
public static final NAUTILUS_BUCKET_FOOD : Lnet/minecraft/tags/TagKey;
public static final NAUTILUS_FOOD : Lnet/minecraft/tags/TagKey;
public static final NAUTILUS_TAMING_ITEMS : Lnet/minecraft/tags/TagKey;
public static final SULFUR_CUBE_FOOD : Lnet/minecraft/tags/TagKey;
public static final MUSHROOMS : Lnet/minecraft/tags/TagKey;
public static final NON_FLAMMABLE_WOOD : Lnet/minecraft/tags/TagKey;
public static final BOATS : Lnet/minecraft/tags/TagKey;
public static final CHEST_BOATS : Lnet/minecraft/tags/TagKey;
public static final FISHES : Lnet/minecraft/tags/TagKey;
public static final CREEPER_DROP_MUSIC_DISCS : Lnet/minecraft/tags/TagKey;
public static final COALS : Lnet/minecraft/tags/TagKey;
public static final ARROWS : Lnet/minecraft/tags/TagKey;
public static final LECTERN_BOOKS : Lnet/minecraft/tags/TagKey;
public static final BOOKSHELF_BOOKS : Lnet/minecraft/tags/TagKey;
public static final BEACON_PAYMENT_ITEMS : Lnet/minecraft/tags/TagKey;
public static final WOODEN_TOOL_MATERIALS : Lnet/minecraft/tags/TagKey;
public static final STONE_TOOL_MATERIALS : Lnet/minecraft/tags/TagKey;
public static final COPPER_TOOL_MATERIALS : Lnet/minecraft/tags/TagKey;
public static final IRON_TOOL_MATERIALS : Lnet/minecraft/tags/TagKey;
public static final GOLD_TOOL_MATERIALS : Lnet/minecraft/tags/TagKey;
public static final DIAMOND_TOOL_MATERIALS : Lnet/minecraft/tags/TagKey;
public static final NETHERITE_TOOL_MATERIALS : Lnet/minecraft/tags/TagKey;
public static final REPAIRS_LEATHER_ARMOR : Lnet/minecraft/tags/TagKey;
public static final REPAIRS_COPPER_ARMOR : Lnet/minecraft/tags/TagKey;
public static final REPAIRS_CHAIN_ARMOR : Lnet/minecraft/tags/TagKey;
public static final REPAIRS_IRON_ARMOR : Lnet/minecraft/tags/TagKey;
public static final REPAIRS_GOLD_ARMOR : Lnet/minecraft/tags/TagKey;
public static final REPAIRS_DIAMOND_ARMOR : Lnet/minecraft/tags/TagKey;
public static final REPAIRS_NETHERITE_ARMOR : Lnet/minecraft/tags/TagKey;
public static final REPAIRS_TURTLE_HELMET : Lnet/minecraft/tags/TagKey;
public static final REPAIRS_WOLF_ARMOR : Lnet/minecraft/tags/TagKey;
public static final STONE_CRAFTING_MATERIALS : Lnet/minecraft/tags/TagKey;
public static final FREEZE_IMMUNE_WEARABLES : Lnet/minecraft/tags/TagKey;
public static final CLUSTER_MAX_HARVESTABLES : Lnet/minecraft/tags/TagKey;
public static final COMPASSES : Lnet/minecraft/tags/TagKey;
public static final CLONABLE_MAPS : Lnet/minecraft/tags/TagKey;
public static final EXTENDABLE_MAPS : Lnet/minecraft/tags/TagKey;
public static final CREEPER_IGNITERS : Lnet/minecraft/tags/TagKey;
public static final NOTE_BLOCK_TOP_INSTRUMENTS : Lnet/minecraft/tags/TagKey;
public static final FOOT_ARMOR : Lnet/minecraft/tags/TagKey;
public static final LEG_ARMOR : Lnet/minecraft/tags/TagKey;
public static final CHEST_ARMOR : Lnet/minecraft/tags/TagKey;
public static final HEAD_ARMOR : Lnet/minecraft/tags/TagKey;
public static final TRIMMABLE_ARMOR : Lnet/minecraft/tags/TagKey;
public static final TRIM_MATERIALS : Lnet/minecraft/tags/TagKey;
public static final DECORATED_POT_SHERDS : Lnet/minecraft/tags/TagKey;
public static final DECORATED_POT_INGREDIENTS : Lnet/minecraft/tags/TagKey;
public static final SWORDS : Lnet/minecraft/tags/TagKey;
public static final AXES : Lnet/minecraft/tags/TagKey;
public static final HOES : Lnet/minecraft/tags/TagKey;
public static final PICKAXES : Lnet/minecraft/tags/TagKey;
public static final SHOVELS : Lnet/minecraft/tags/TagKey;
public static final SPEARS : Lnet/minecraft/tags/TagKey;
public static final BREAKS_DECORATED_POTS : Lnet/minecraft/tags/TagKey;
public static final VILLAGER_PLANTABLE_SEEDS : Lnet/minecraft/tags/TagKey;
public static final VILLAGER_PICKS_UP : Lnet/minecraft/tags/TagKey;
public static final FURNACE_MINECART_FUEL : Lnet/minecraft/tags/TagKey;
public static final BUNDLES : Lnet/minecraft/tags/TagKey;
public static final BOOK_CLONING_TARGET : Lnet/minecraft/tags/TagKey;
public static final DYES : Lnet/minecraft/tags/TagKey;
public static final LOOM_DYES : Lnet/minecraft/tags/TagKey;
public static final LOOM_PATTERNS : Lnet/minecraft/tags/TagKey;
public static final CAULDRON_CAN_REMOVE_DYE : Lnet/minecraft/tags/TagKey;
public static final CAT_COLLAR_DYES : Lnet/minecraft/tags/TagKey;
public static final WOLF_COLLAR_DYES : Lnet/minecraft/tags/TagKey;
public static final CUSHIONS : Lnet/minecraft/tags/TagKey;
public static final SKELETON_PREFERRED_WEAPONS : Lnet/minecraft/tags/TagKey;
public static final DROWNED_PREFERRED_WEAPONS : Lnet/minecraft/tags/TagKey;
public static final PIGLIN_PREFERRED_WEAPONS : Lnet/minecraft/tags/TagKey;
public static final PILLAGER_PREFERRED_WEAPONS : Lnet/minecraft/tags/TagKey;
public static final WITHER_SKELETON_DISLIKED_WEAPONS : Lnet/minecraft/tags/TagKey;
public static final SHEARABLE_FROM_COPPER_GOLEM : Lnet/minecraft/tags/TagKey;
public static final METAL_NUGGETS : Lnet/minecraft/tags/TagKey;
public static final DOUSES_CAMPFIRES : Lnet/minecraft/tags/TagKey;
public static final BREWING_POTION_INPUTS : Lnet/minecraft/tags/TagKey;
public static final FURNACE_FUEL_BOTTOM_TAKEABLE : Lnet/minecraft/tags/TagKey;
public static final FOOT_ARMOR_ENCHANTABLE : Lnet/minecraft/tags/TagKey;
public static final LEG_ARMOR_ENCHANTABLE : Lnet/minecraft/tags/TagKey;
public static final CHEST_ARMOR_ENCHANTABLE : Lnet/minecraft/tags/TagKey;
public static final HEAD_ARMOR_ENCHANTABLE : Lnet/minecraft/tags/TagKey;
public static final ARMOR_ENCHANTABLE : Lnet/minecraft/tags/TagKey;
public static final MELEE_WEAPON_ENCHANTABLE : Lnet/minecraft/tags/TagKey;
public static final SWEEPING_ENCHANTABLE : Lnet/minecraft/tags/TagKey;
public static final FIRE_ASPECT_ENCHANTABLE : Lnet/minecraft/tags/TagKey;
public static final SHARP_WEAPON_ENCHANTABLE : Lnet/minecraft/tags/TagKey;
public static final WEAPON_ENCHANTABLE : Lnet/minecraft/tags/TagKey;
public static final MINING_ENCHANTABLE : Lnet/minecraft/tags/TagKey;
public static final MINING_LOOT_ENCHANTABLE : Lnet/minecraft/tags/TagKey;
public static final FISHING_ENCHANTABLE : Lnet/minecraft/tags/TagKey;
public static final TRIDENT_ENCHANTABLE : Lnet/minecraft/tags/TagKey;
public static final LUNGE_ENCHANTABLE : Lnet/minecraft/tags/TagKey;
public static final DURABILITY_ENCHANTABLE : Lnet/minecraft/tags/TagKey;
public static final BOW_ENCHANTABLE : Lnet/minecraft/tags/TagKey;
public static final EQUIPPABLE_ENCHANTABLE : Lnet/minecraft/tags/TagKey;
public static final CROSSBOW_ENCHANTABLE : Lnet/minecraft/tags/TagKey;
public static final VANISHING_ENCHANTABLE : Lnet/minecraft/tags/TagKey;
public static final MACE_ENCHANTABLE : Lnet/minecraft/tags/TagKey;
public static final MAP_INVISIBILITY_EQUIPMENT : Lnet/minecraft/tags/TagKey;
public static final GAZE_DISGUISE_EQUIPMENT : Lnet/minecraft/tags/TagKey;
private <init>()V
private static bind(Ljava/lang/String;)Lnet/minecraft/tags/TagKey;
static <clinit>()V
```
