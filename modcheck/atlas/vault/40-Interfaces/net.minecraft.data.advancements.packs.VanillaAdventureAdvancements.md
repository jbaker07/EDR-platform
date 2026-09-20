---
type: "interface"
fqcn: "net.minecraft.data.advancements.packs.VanillaAdventureAdvancements"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.advancements.packs.VanillaAdventureAdvancements

System: [[20-Systems/net.minecraft.data.advancements|net.minecraft.data.advancements]]

`class` public; extends `net/minecraft/data/advancements/AdvancementSubProvider`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `validateMobsToKill` | `(Ljava/util/List;)Ljava/util/List;` | name_only | @ModifyExpressionValue at ['INVOKE'] | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (15 fields, 29 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final DISTANCE_FROM_BOTTOM_TO_TOP : I
private static final Y_COORDINATE_AT_TOP : I
private static final Y_COORDINATE_AT_BOTTOM : I
private static final BEDROCK_THICKNESS : I
private static final EXCEPTIONS_BY_EXPECTED_CATEGORIES : Ljava/util/Map;
private static final MOBS_TO_KILL : Ljava/util/List;
private final entityTypes : Lnet/minecraft/core/HolderGetter;
private final items : Lnet/minecraft/core/HolderGetter;
private final blocks : Lnet/minecraft/core/HolderGetter;
private final biomes : Lnet/minecraft/core/HolderGetter;
private final structures : Lnet/minecraft/core/HolderGetter;
private final bannerPatterns : Lnet/minecraft/core/HolderGetter;
private final lootTables : Lnet/minecraft/core/HolderGetter;
private final recipes : Lnet/minecraft/core/HolderGetter;
public <init>(Lnet/minecraft/data/worldgen/BootstrapContext;)V
private static fireCountAndBystander(Lnet/minecraft/advancements/predicates/MinMaxBounds$Ints;Ljava/util/Optional;)Lnet/minecraft/advancements/triggers/Criterion;
private static lookAtThroughItem(Lnet/minecraft/advancements/predicates/entity/EntityPredicate$Builder;Lnet/minecraft/advancements/predicates/ItemPredicate$Builder;)Lnet/minecraft/advancements/triggers/Criterion;
public generate()V
private lookupRecipe(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/core/HolderSet;
public createMonsterHunterAdvancement(Lnet/minecraft/advancements/AdvancementHolder;Ljava/util/List;)Lnet/minecraft/advancements/AdvancementHolder;
private placedBlockReadByComparator(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/advancements/triggers/Criterion;
private placedComparatorReadingBlock(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/advancements/triggers/Criterion;
private placedBlockActivatesCreakingHeart(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/advancements/triggers/Criterion;
private smithingWithStyle(Lnet/minecraft/advancements/Advancement$Builder;)Lnet/minecraft/advancements/Advancement$Builder;
private craftingANewLook(Lnet/minecraft/advancements/Advancement$Builder;)Lnet/minecraft/advancements/Advancement$Builder;
private respectingTheRemnantsCriterions(Lnet/minecraft/advancements/Advancement$Builder;)Lnet/minecraft/advancements/Advancement$Builder;
protected createAdventuringTime(Lnet/minecraft/advancements/AdvancementHolder;Lnet/minecraft/world/level/biome/MultiNoiseBiomeSourceParameterList$Preset;)V
private addMobsToKill(Lnet/minecraft/advancements/Advancement$Builder;Ljava/util/List;)Lnet/minecraft/advancements/Advancement$Builder;
public static addBiomes(Lnet/minecraft/advancements/Advancement$Builder;Lnet/minecraft/core/HolderGetter;Ljava/util/List;)Lnet/minecraft/advancements/Advancement$Builder;
private validateMobsToKill(Ljava/util/List;)Ljava/util/List;
private static synthetic lambda$validateMobsToKill$0(Ljava/util/Map;Ljava/util/List;Lnet/minecraft/world/entity/MobCategory;Ljava/util/Set;)V
private synthetic lambda$addMobsToKill$0(Lnet/minecraft/advancements/Advancement$Builder;Lnet/minecraft/world/entity/EntityType;)V
private static synthetic lambda$respectingTheRemnantsCriterions$0(Lnet/minecraft/advancements/Advancement$Builder;Lcom/mojang/datafixers/util/Pair;)V
private synthetic lambda$craftingANewLook$0(Lnet/minecraft/advancements/Advancement$Builder;Lnet/minecraft/resources/ResourceKey;)V
private synthetic lambda$smithingWithStyle$1(Lnet/minecraft/advancements/Advancement$Builder;Lnet/minecraft/data/recipes/packs/VanillaRecipeProvider$TrimTemplate;)V
private static synthetic lambda$smithingWithStyle$0(Ljava/util/Set;Lnet/minecraft/data/recipes/packs/VanillaRecipeProvider$TrimTemplate;)Z
private static synthetic lambda$placedBlockActivatesCreakingHeart$1(I)[Lnet/minecraft/world/level/storage/loot/predicates/LootItemCondition$Builder;
private synthetic lambda$placedBlockActivatesCreakingHeart$0(Lnet/minecraft/tags/TagKey;Lnet/minecraft/core/Direction;)Lnet/minecraft/world/level/storage/loot/predicates/AllOfCondition$Builder;
private static synthetic lambda$placedComparatorReadingBlock$1(I)[Lnet/minecraft/world/level/storage/loot/predicates/LootItemCondition$Builder;
private synthetic lambda$placedComparatorReadingBlock$0(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/core/Direction;)Lnet/minecraft/world/level/storage/loot/predicates/AllOfCondition$Builder;
private static synthetic lambda$placedBlockReadByComparator$1(I)[Lnet/minecraft/world/level/storage/loot/predicates/LootItemCondition$Builder;
private synthetic lambda$placedBlockReadByComparator$0(Lnet/minecraft/core/Direction;)Lnet/minecraft/world/level/storage/loot/predicates/LootItemCondition$Builder;
static <clinit>()V
```
