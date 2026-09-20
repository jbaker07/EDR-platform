---
type: "interface"
fqcn: "net.minecraft.world.level.block.SoundType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.SoundType

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `HANGING_SIGN` | `Lnet/minecraft/world/level/block/SoundType;` | exact | getstatic@12 in `WoodTypeBuilder.<init>` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| reads | `WOOD` | `Lnet/minecraft/world/level/block/SoundType;` | exact | getstatic@27 in `BlockSetTypeBuilder.<init>` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| reads | `WOOD` | `Lnet/minecraft/world/level/block/SoundType;` | exact | getstatic@5 in `WoodTypeBuilder.<init>` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |

## Declared members (138 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final EMPTY : Lnet/minecraft/world/level/block/SoundType;
public static final WOOD : Lnet/minecraft/world/level/block/SoundType;
public static final GRAVEL : Lnet/minecraft/world/level/block/SoundType;
public static final GRASS : Lnet/minecraft/world/level/block/SoundType;
public static final LILY_PAD : Lnet/minecraft/world/level/block/SoundType;
public static final STONE : Lnet/minecraft/world/level/block/SoundType;
public static final METAL : Lnet/minecraft/world/level/block/SoundType;
public static final GLASS : Lnet/minecraft/world/level/block/SoundType;
public static final WOOL : Lnet/minecraft/world/level/block/SoundType;
public static final SAND : Lnet/minecraft/world/level/block/SoundType;
public static final SNOW : Lnet/minecraft/world/level/block/SoundType;
public static final POWDER_SNOW : Lnet/minecraft/world/level/block/SoundType;
public static final LADDER : Lnet/minecraft/world/level/block/SoundType;
public static final ANVIL : Lnet/minecraft/world/level/block/SoundType;
public static final SLIME_BLOCK : Lnet/minecraft/world/level/block/SoundType;
public static final HONEY_BLOCK : Lnet/minecraft/world/level/block/SoundType;
public static final WET_GRASS : Lnet/minecraft/world/level/block/SoundType;
public static final CORAL_BLOCK : Lnet/minecraft/world/level/block/SoundType;
public static final BAMBOO : Lnet/minecraft/world/level/block/SoundType;
public static final BAMBOO_SAPLING : Lnet/minecraft/world/level/block/SoundType;
public static final SCAFFOLDING : Lnet/minecraft/world/level/block/SoundType;
public static final SWEET_BERRY_BUSH : Lnet/minecraft/world/level/block/SoundType;
public static final CROP : Lnet/minecraft/world/level/block/SoundType;
public static final HARD_CROP : Lnet/minecraft/world/level/block/SoundType;
public static final VINE : Lnet/minecraft/world/level/block/SoundType;
public static final NETHER_WART : Lnet/minecraft/world/level/block/SoundType;
public static final LANTERN : Lnet/minecraft/world/level/block/SoundType;
public static final STEM : Lnet/minecraft/world/level/block/SoundType;
public static final NYLIUM : Lnet/minecraft/world/level/block/SoundType;
public static final FUNGUS : Lnet/minecraft/world/level/block/SoundType;
public static final ROOTS : Lnet/minecraft/world/level/block/SoundType;
public static final SHROOMLIGHT : Lnet/minecraft/world/level/block/SoundType;
public static final WEEPING_VINES : Lnet/minecraft/world/level/block/SoundType;
public static final TWISTING_VINES : Lnet/minecraft/world/level/block/SoundType;
public static final SOUL_SAND : Lnet/minecraft/world/level/block/SoundType;
public static final SOUL_SOIL : Lnet/minecraft/world/level/block/SoundType;
public static final BASALT : Lnet/minecraft/world/level/block/SoundType;
public static final WART_BLOCK : Lnet/minecraft/world/level/block/SoundType;
public static final NETHERRACK : Lnet/minecraft/world/level/block/SoundType;
public static final NETHER_BRICKS : Lnet/minecraft/world/level/block/SoundType;
public static final NETHER_SPROUTS : Lnet/minecraft/world/level/block/SoundType;
public static final NETHER_ORE : Lnet/minecraft/world/level/block/SoundType;
public static final BONE_BLOCK : Lnet/minecraft/world/level/block/SoundType;
public static final NETHERITE_BLOCK : Lnet/minecraft/world/level/block/SoundType;
public static final ANCIENT_DEBRIS : Lnet/minecraft/world/level/block/SoundType;
public static final LODESTONE : Lnet/minecraft/world/level/block/SoundType;
public static final CHAIN : Lnet/minecraft/world/level/block/SoundType;
public static final NETHER_GOLD_ORE : Lnet/minecraft/world/level/block/SoundType;
public static final GILDED_BLACKSTONE : Lnet/minecraft/world/level/block/SoundType;
public static final CANDLE : Lnet/minecraft/world/level/block/SoundType;
public static final AMETHYST : Lnet/minecraft/world/level/block/SoundType;
public static final AMETHYST_CLUSTER : Lnet/minecraft/world/level/block/SoundType;
public static final SMALL_AMETHYST_BUD : Lnet/minecraft/world/level/block/SoundType;
public static final MEDIUM_AMETHYST_BUD : Lnet/minecraft/world/level/block/SoundType;
public static final LARGE_AMETHYST_BUD : Lnet/minecraft/world/level/block/SoundType;
public static final TUFF : Lnet/minecraft/world/level/block/SoundType;
public static final TUFF_BRICKS : Lnet/minecraft/world/level/block/SoundType;
public static final POLISHED_TUFF : Lnet/minecraft/world/level/block/SoundType;
public static final CALCITE : Lnet/minecraft/world/level/block/SoundType;
public static final DRIPSTONE_BLOCK : Lnet/minecraft/world/level/block/SoundType;
public static final POINTED_DRIPSTONE : Lnet/minecraft/world/level/block/SoundType;
public static final COPPER : Lnet/minecraft/world/level/block/SoundType;
public static final COPPER_BULB : Lnet/minecraft/world/level/block/SoundType;
public static final COPPER_GRATE : Lnet/minecraft/world/level/block/SoundType;
public static final COPPER_GOLEM_STATUE : Lnet/minecraft/world/level/block/SoundType;
public static final CAVE_VINES : Lnet/minecraft/world/level/block/SoundType;
public static final SPORE_BLOSSOM : Lnet/minecraft/world/level/block/SoundType;
public static final CACTUS_FLOWER : Lnet/minecraft/world/level/block/SoundType;
public static final AZALEA : Lnet/minecraft/world/level/block/SoundType;
public static final FLOWERING_AZALEA : Lnet/minecraft/world/level/block/SoundType;
public static final MOSS_CARPET : Lnet/minecraft/world/level/block/SoundType;
public static final PINK_PETALS : Lnet/minecraft/world/level/block/SoundType;
public static final LEAF_LITTER : Lnet/minecraft/world/level/block/SoundType;
public static final MOSS : Lnet/minecraft/world/level/block/SoundType;
public static final BIG_DRIPLEAF : Lnet/minecraft/world/level/block/SoundType;
public static final SMALL_DRIPLEAF : Lnet/minecraft/world/level/block/SoundType;
public static final ROOTED_DIRT : Lnet/minecraft/world/level/block/SoundType;
public static final HANGING_ROOTS : Lnet/minecraft/world/level/block/SoundType;
public static final AZALEA_LEAVES : Lnet/minecraft/world/level/block/SoundType;
public static final SCULK_SENSOR : Lnet/minecraft/world/level/block/SoundType;
public static final SCULK_CATALYST : Lnet/minecraft/world/level/block/SoundType;
public static final SCULK : Lnet/minecraft/world/level/block/SoundType;
public static final SCULK_VEIN : Lnet/minecraft/world/level/block/SoundType;
public static final SCULK_SHRIEKER : Lnet/minecraft/world/level/block/SoundType;
public static final GLOW_LICHEN : Lnet/minecraft/world/level/block/SoundType;
public static final DEEPSLATE : Lnet/minecraft/world/level/block/SoundType;
public static final DEEPSLATE_BRICKS : Lnet/minecraft/world/level/block/SoundType;
public static final DEEPSLATE_TILES : Lnet/minecraft/world/level/block/SoundType;
public static final POLISHED_DEEPSLATE : Lnet/minecraft/world/level/block/SoundType;
public static final FROGLIGHT : Lnet/minecraft/world/level/block/SoundType;
public static final FROGSPAWN : Lnet/minecraft/world/level/block/SoundType;
public static final MANGROVE_ROOTS : Lnet/minecraft/world/level/block/SoundType;
public static final MUDDY_MANGROVE_ROOTS : Lnet/minecraft/world/level/block/SoundType;
public static final MUD : Lnet/minecraft/world/level/block/SoundType;
public static final MUD_BRICKS : Lnet/minecraft/world/level/block/SoundType;
public static final PACKED_MUD : Lnet/minecraft/world/level/block/SoundType;
public static final HANGING_SIGN : Lnet/minecraft/world/level/block/SoundType;
public static final NETHER_WOOD_HANGING_SIGN : Lnet/minecraft/world/level/block/SoundType;
public static final BAMBOO_WOOD_HANGING_SIGN : Lnet/minecraft/world/level/block/SoundType;
public static final BAMBOO_WOOD : Lnet/minecraft/world/level/block/SoundType;
public static final NETHER_WOOD : Lnet/minecraft/world/level/block/SoundType;
public static final CHERRY_WOOD : Lnet/minecraft/world/level/block/SoundType;
public static final CHERRY_SAPLING : Lnet/minecraft/world/level/block/SoundType;
public static final CHERRY_LEAVES : Lnet/minecraft/world/level/block/SoundType;
public static final CHERRY_WOOD_HANGING_SIGN : Lnet/minecraft/world/level/block/SoundType;
public static final CHISELED_BOOKSHELF : Lnet/minecraft/world/level/block/SoundType;
public static final SHELF : Lnet/minecraft/world/level/block/SoundType;
public static final SUSPICIOUS_SAND : Lnet/minecraft/world/level/block/SoundType;
public static final SUSPICIOUS_GRAVEL : Lnet/minecraft/world/level/block/SoundType;
public static final DECORATED_POT : Lnet/minecraft/world/level/block/SoundType;
public static final DECORATED_POT_CRACKED : Lnet/minecraft/world/level/block/SoundType;
public static final TRIAL_SPAWNER : Lnet/minecraft/world/level/block/SoundType;
public static final SPONGE : Lnet/minecraft/world/level/block/SoundType;
public static final WET_SPONGE : Lnet/minecraft/world/level/block/SoundType;
public static final VAULT : Lnet/minecraft/world/level/block/SoundType;
public static final CREAKING_HEART : Lnet/minecraft/world/level/block/SoundType;
public static final HEAVY_CORE : Lnet/minecraft/world/level/block/SoundType;
public static final COBWEB : Lnet/minecraft/world/level/block/SoundType;
public static final SPAWNER : Lnet/minecraft/world/level/block/SoundType;
public static final RESIN : Lnet/minecraft/world/level/block/SoundType;
public static final RESIN_BRICKS : Lnet/minecraft/world/level/block/SoundType;
public static final IRON : Lnet/minecraft/world/level/block/SoundType;
public static final DRIED_GHAST : Lnet/minecraft/world/level/block/SoundType;
public static final SULFUR : Lnet/minecraft/world/level/block/SoundType;
public static final POTENT_SULFUR : Lnet/minecraft/world/level/block/SoundType;
public static final SULFUR_SPIKE : Lnet/minecraft/world/level/block/SoundType;
public static final CINNABAR : Lnet/minecraft/world/level/block/SoundType;
public static final SHELF_MUSHROOM : Lnet/minecraft/world/level/block/SoundType;
public static final POPLAR_LEAVES : Lnet/minecraft/world/level/block/SoundType;
public static final STRAW_BED : Lnet/minecraft/world/level/block/SoundType;
public static final RED_SHRUB : Lnet/minecraft/world/level/block/SoundType;
public final volume : F
public final pitch : F
private final breakSound : Lnet/minecraft/sounds/SoundEvent;
private final stepSound : Lnet/minecraft/sounds/SoundEvent;
private final placeSound : Lnet/minecraft/sounds/SoundEvent;
private final hitSound : Lnet/minecraft/sounds/SoundEvent;
private final fallSound : Lnet/minecraft/sounds/SoundEvent;
public <init>(FFLnet/minecraft/sounds/SoundEvent;Lnet/minecraft/sounds/SoundEvent;Lnet/minecraft/sounds/SoundEvent;Lnet/minecraft/sounds/SoundEvent;Lnet/minecraft/sounds/SoundEvent;)V
public getVolume()F
public getPitch()F
public getBreakSound()Lnet/minecraft/sounds/SoundEvent;
public getStepSound()Lnet/minecraft/sounds/SoundEvent;
public getPlaceSound()Lnet/minecraft/sounds/SoundEvent;
public getHitSound()Lnet/minecraft/sounds/SoundEvent;
public getFallSound()Lnet/minecraft/sounds/SoundEvent;
static <clinit>()V
```
