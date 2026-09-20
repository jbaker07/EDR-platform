---
type: "interface"
fqcn: "net.minecraft.world.entity.EntityTypes"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.EntityTypes

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `PLAYER` | `Lnet/minecraft/world/entity/EntityType;` | exact | getstatic@40 in `EntityRenderersMixin.createAvatarRenderer` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (164 fields, 33 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final MAGIC_HORSE_WIDTH : F
private static final DISPLAY_TRACKING_RANGE : I
public static final ACACIA_BOAT : Lnet/minecraft/world/entity/EntityType;
public static final ACACIA_CHEST_BOAT : Lnet/minecraft/world/entity/EntityType;
public static final ALLAY : Lnet/minecraft/world/entity/EntityType;
public static final AREA_EFFECT_CLOUD : Lnet/minecraft/world/entity/EntityType;
public static final ARMADILLO : Lnet/minecraft/world/entity/EntityType;
public static final ARMOR_STAND : Lnet/minecraft/world/entity/EntityType;
public static final ARROW : Lnet/minecraft/world/entity/EntityType;
public static final AXOLOTL : Lnet/minecraft/world/entity/EntityType;
public static final BAMBOO_CHEST_RAFT : Lnet/minecraft/world/entity/EntityType;
public static final BAMBOO_RAFT : Lnet/minecraft/world/entity/EntityType;
public static final BAT : Lnet/minecraft/world/entity/EntityType;
public static final BEE : Lnet/minecraft/world/entity/EntityType;
public static final BIRCH_BOAT : Lnet/minecraft/world/entity/EntityType;
public static final BIRCH_CHEST_BOAT : Lnet/minecraft/world/entity/EntityType;
public static final BLAZE : Lnet/minecraft/world/entity/EntityType;
public static final BLOCK_DISPLAY : Lnet/minecraft/world/entity/EntityType;
public static final BOGGED : Lnet/minecraft/world/entity/EntityType;
public static final BREEZE : Lnet/minecraft/world/entity/EntityType;
public static final BREEZE_WIND_CHARGE : Lnet/minecraft/world/entity/EntityType;
public static final CAMEL : Lnet/minecraft/world/entity/EntityType;
public static final CAMEL_HUSK : Lnet/minecraft/world/entity/EntityType;
public static final CAT : Lnet/minecraft/world/entity/EntityType;
public static final CAVE_SPIDER : Lnet/minecraft/world/entity/EntityType;
public static final CHERRY_BOAT : Lnet/minecraft/world/entity/EntityType;
public static final CHERRY_CHEST_BOAT : Lnet/minecraft/world/entity/EntityType;
public static final CHEST_MINECART : Lnet/minecraft/world/entity/EntityType;
public static final CHICKEN : Lnet/minecraft/world/entity/EntityType;
public static final COD : Lnet/minecraft/world/entity/EntityType;
public static final COPPER_GOLEM : Lnet/minecraft/world/entity/EntityType;
public static final COMMAND_BLOCK_MINECART : Lnet/minecraft/world/entity/EntityType;
public static final COW : Lnet/minecraft/world/entity/EntityType;
public static final CREAKING : Lnet/minecraft/world/entity/EntityType;
public static final CREEPER : Lnet/minecraft/world/entity/EntityType;
public static final CUSHION : Lnet/minecraft/world/entity/EntityType;
public static final DARK_OAK_BOAT : Lnet/minecraft/world/entity/EntityType;
public static final DARK_OAK_CHEST_BOAT : Lnet/minecraft/world/entity/EntityType;
public static final DOLPHIN : Lnet/minecraft/world/entity/EntityType;
public static final DONKEY : Lnet/minecraft/world/entity/EntityType;
public static final DRAGON_FIREBALL : Lnet/minecraft/world/entity/EntityType;
public static final DROWNED : Lnet/minecraft/world/entity/EntityType;
public static final EGG : Lnet/minecraft/world/entity/EntityType;
public static final ELDER_GUARDIAN : Lnet/minecraft/world/entity/EntityType;
public static final ENDERMAN : Lnet/minecraft/world/entity/EntityType;
public static final ENDERMITE : Lnet/minecraft/world/entity/EntityType;
public static final ENDER_DRAGON : Lnet/minecraft/world/entity/EntityType;
public static final ENDER_PEARL : Lnet/minecraft/world/entity/EntityType;
public static final END_CRYSTAL : Lnet/minecraft/world/entity/EntityType;
public static final EVOKER : Lnet/minecraft/world/entity/EntityType;
public static final EVOKER_FANGS : Lnet/minecraft/world/entity/EntityType;
public static final EXPERIENCE_BOTTLE : Lnet/minecraft/world/entity/EntityType;
public static final EXPERIENCE_ORB : Lnet/minecraft/world/entity/EntityType;
public static final EYE_OF_ENDER : Lnet/minecraft/world/entity/EntityType;
public static final FALLING_BLOCK : Lnet/minecraft/world/entity/EntityType;
public static final FIREBALL : Lnet/minecraft/world/entity/EntityType;
public static final FIREWORK_ROCKET : Lnet/minecraft/world/entity/EntityType;
public static final FOX : Lnet/minecraft/world/entity/EntityType;
public static final FROG : Lnet/minecraft/world/entity/EntityType;
public static final FURNACE_MINECART : Lnet/minecraft/world/entity/EntityType;
public static final GHAST : Lnet/minecraft/world/entity/EntityType;
public static final HAPPY_GHAST : Lnet/minecraft/world/entity/EntityType;
public static final GIANT : Lnet/minecraft/world/entity/EntityType;
public static final GLOW_ITEM_FRAME : Lnet/minecraft/world/entity/EntityType;
public static final GLOW_SQUID : Lnet/minecraft/world/entity/EntityType;
public static final GOAT : Lnet/minecraft/world/entity/EntityType;
public static final GUARDIAN : Lnet/minecraft/world/entity/EntityType;
public static final HOGLIN : Lnet/minecraft/world/entity/EntityType;
public static final HOPPER_MINECART : Lnet/minecraft/world/entity/EntityType;
public static final HORSE : Lnet/minecraft/world/entity/EntityType;
public static final HUSK : Lnet/minecraft/world/entity/EntityType;
public static final ILLUSIONER : Lnet/minecraft/world/entity/EntityType;
public static final INTERACTION : Lnet/minecraft/world/entity/EntityType;
public static final IRON_GOLEM : Lnet/minecraft/world/entity/EntityType;
public static final ITEM : Lnet/minecraft/world/entity/EntityType;
public static final ITEM_DISPLAY : Lnet/minecraft/world/entity/EntityType;
public static final ITEM_FRAME : Lnet/minecraft/world/entity/EntityType;
public static final JUNGLE_BOAT : Lnet/minecraft/world/entity/EntityType;
public static final JUNGLE_CHEST_BOAT : Lnet/minecraft/world/entity/EntityType;
public static final LEASH_KNOT : Lnet/minecraft/world/entity/EntityType;
public static final LIGHTNING_BOLT : Lnet/minecraft/world/entity/EntityType;
public static final LLAMA : Lnet/minecraft/world/entity/EntityType;
public static final LLAMA_SPIT : Lnet/minecraft/world/entity/EntityType;
public static final MAGMA_CUBE : Lnet/minecraft/world/entity/EntityType;
public static final MANGROVE_BOAT : Lnet/minecraft/world/entity/EntityType;
public static final MANGROVE_CHEST_BOAT : Lnet/minecraft/world/entity/EntityType;
public static final MANNEQUIN : Lnet/minecraft/world/entity/EntityType;
public static final MARKER : Lnet/minecraft/world/entity/EntityType;
public static final MINECART : Lnet/minecraft/world/entity/EntityType;
public static final MOOSHROOM : Lnet/minecraft/world/entity/EntityType;
public static final MULE : Lnet/minecraft/world/entity/EntityType;
public static final NAUTILUS : Lnet/minecraft/world/entity/EntityType;
public static final OAK_BOAT : Lnet/minecraft/world/entity/EntityType;
public static final OAK_CHEST_BOAT : Lnet/minecraft/world/entity/EntityType;
public static final OCELOT : Lnet/minecraft/world/entity/EntityType;
public static final OMINOUS_ITEM_SPAWNER : Lnet/minecraft/world/entity/EntityType;
public static final PAINTING : Lnet/minecraft/world/entity/EntityType;
public static final PALE_OAK_BOAT : Lnet/minecraft/world/entity/EntityType;
public static final PALE_OAK_CHEST_BOAT : Lnet/minecraft/world/entity/EntityType;
public static final PANDA : Lnet/minecraft/world/entity/EntityType;
public static final PARCHED : Lnet/minecraft/world/entity/EntityType;
public static final PARROT : Lnet/minecraft/world/entity/EntityType;
public static final PHANTOM : Lnet/minecraft/world/entity/EntityType;
public static final PIG : Lnet/minecraft/world/entity/EntityType;
public static final PIGLIN : Lnet/minecraft/world/entity/EntityType;
public static final PIGLIN_BRUTE : Lnet/minecraft/world/entity/EntityType;
public static final PILLAGER : Lnet/minecraft/world/entity/EntityType;
public static final POLAR_BEAR : Lnet/minecraft/world/entity/EntityType;
public static final POPLAR_BOAT : Lnet/minecraft/world/entity/EntityType;
public static final POPLAR_CHEST_BOAT : Lnet/minecraft/world/entity/EntityType;
public static final SPLASH_POTION : Lnet/minecraft/world/entity/EntityType;
public static final LINGERING_POTION : Lnet/minecraft/world/entity/EntityType;
public static final PUFFERFISH : Lnet/minecraft/world/entity/EntityType;
public static final RABBIT : Lnet/minecraft/world/entity/EntityType;
public static final RAVAGER : Lnet/minecraft/world/entity/EntityType;
public static final SALMON : Lnet/minecraft/world/entity/EntityType;
public static final SHEEP : Lnet/minecraft/world/entity/EntityType;
public static final SHULKER : Lnet/minecraft/world/entity/EntityType;
public static final SHULKER_BULLET : Lnet/minecraft/world/entity/EntityType;
public static final SILVERFISH : Lnet/minecraft/world/entity/EntityType;
public static final SKELETON : Lnet/minecraft/world/entity/EntityType;
public static final SKELETON_HORSE : Lnet/minecraft/world/entity/EntityType;
public static final SLIME : Lnet/minecraft/world/entity/EntityType;
public static final SMALL_FIREBALL : Lnet/minecraft/world/entity/EntityType;
public static final SNIFFER : Lnet/minecraft/world/entity/EntityType;
public static final SNOWBALL : Lnet/minecraft/world/entity/EntityType;
public static final SNOW_GOLEM : Lnet/minecraft/world/entity/EntityType;
public static final SPAWNER_MINECART : Lnet/minecraft/world/entity/EntityType;
public static final SPECTRAL_ARROW : Lnet/minecraft/world/entity/EntityType;
public static final SPIDER : Lnet/minecraft/world/entity/EntityType;
public static final SPRUCE_BOAT : Lnet/minecraft/world/entity/EntityType;
public static final SPRUCE_CHEST_BOAT : Lnet/minecraft/world/entity/EntityType;
public static final SQUID : Lnet/minecraft/world/entity/EntityType;
public static final STRAY : Lnet/minecraft/world/entity/EntityType;
public static final STRIDER : Lnet/minecraft/world/entity/EntityType;
public static final SULFUR_CUBE : Lnet/minecraft/world/entity/EntityType;
public static final TADPOLE : Lnet/minecraft/world/entity/EntityType;
public static final TEXT_DISPLAY : Lnet/minecraft/world/entity/EntityType;
public static final TNT : Lnet/minecraft/world/entity/EntityType;
public static final TNT_MINECART : Lnet/minecraft/world/entity/EntityType;
public static final TRADER_LLAMA : Lnet/minecraft/world/entity/EntityType;
public static final TRIDENT : Lnet/minecraft/world/entity/EntityType;
public static final TROPICAL_FISH : Lnet/minecraft/world/entity/EntityType;
public static final TURTLE : Lnet/minecraft/world/entity/EntityType;
public static final VEX : Lnet/minecraft/world/entity/EntityType;
public static final VILLAGER : Lnet/minecraft/world/entity/EntityType;
public static final VINDICATOR : Lnet/minecraft/world/entity/EntityType;
public static final WANDERING_TRADER : Lnet/minecraft/world/entity/EntityType;
public static final WARDEN : Lnet/minecraft/world/entity/EntityType;
public static final WIND_CHARGE : Lnet/minecraft/world/entity/EntityType;
public static final WITCH : Lnet/minecraft/world/entity/EntityType;
public static final WITHER : Lnet/minecraft/world/entity/EntityType;
public static final WITHER_SKELETON : Lnet/minecraft/world/entity/EntityType;
public static final WITHER_SKULL : Lnet/minecraft/world/entity/EntityType;
public static final WOLF : Lnet/minecraft/world/entity/EntityType;
public static final ZOGLIN : Lnet/minecraft/world/entity/EntityType;
public static final ZOMBIE : Lnet/minecraft/world/entity/EntityType;
public static final ZOMBIE_HORSE : Lnet/minecraft/world/entity/EntityType;
public static final ZOMBIE_NAUTILUS : Lnet/minecraft/world/entity/EntityType;
public static final ZOMBIE_VILLAGER : Lnet/minecraft/world/entity/EntityType;
public static final ZOMBIFIED_PIGLIN : Lnet/minecraft/world/entity/EntityType;
public static final PLAYER : Lnet/minecraft/world/entity/EntityType;
public static final FISHING_BOBBER : Lnet/minecraft/world/entity/EntityType;
static final OP_ONLY_CUSTOM_DATA : Ljava/util/Set;
public <init>()V
private static boatFactory(Ljava/util/function/Supplier;)Lnet/minecraft/world/entity/EntityType$EntityFactory;
private static chestBoatFactory(Ljava/util/function/Supplier;)Lnet/minecraft/world/entity/EntityType$EntityFactory;
private static raftFactory(Ljava/util/function/Supplier;)Lnet/minecraft/world/entity/EntityType$EntityFactory;
private static chestRaftFactory(Ljava/util/function/Supplier;)Lnet/minecraft/world/entity/EntityType$EntityFactory;
private static register(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/world/entity/EntityType$Builder;)Lnet/minecraft/world/entity/EntityType;
private static synthetic lambda$chestRaftFactory$0(Ljava/util/function/Supplier;Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/Level;)Lnet/minecraft/world/entity/vehicle/boat/ChestRaft;
private static synthetic lambda$raftFactory$0(Ljava/util/function/Supplier;Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/Level;)Lnet/minecraft/world/entity/vehicle/boat/Raft;
private static synthetic lambda$chestBoatFactory$0(Ljava/util/function/Supplier;Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/Level;)Lnet/minecraft/world/entity/vehicle/boat/ChestBoat;
private static synthetic lambda$boatFactory$0(Ljava/util/function/Supplier;Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/Level;)Lnet/minecraft/world/entity/vehicle/boat/Boat;
private static synthetic lambda$static$21()Lnet/minecraft/world/item/Item;
private static synthetic lambda$static$20()Lnet/minecraft/world/item/Item;
private static synthetic lambda$static$19()Lnet/minecraft/world/item/Item;
private static synthetic lambda$static$18()Lnet/minecraft/world/item/Item;
private static synthetic lambda$static$17()Lnet/minecraft/world/item/Item;
private static synthetic lambda$static$16()Lnet/minecraft/world/item/Item;
private static synthetic lambda$static$15()Lnet/minecraft/world/item/Item;
private static synthetic lambda$static$14()Lnet/minecraft/world/item/Item;
private static synthetic lambda$static$13()Lnet/minecraft/world/item/Item;
private static synthetic lambda$static$12()Lnet/minecraft/world/item/Item;
private static synthetic lambda$static$11()Lnet/minecraft/world/item/Item;
private static synthetic lambda$static$10()Lnet/minecraft/world/item/Item;
private static synthetic lambda$static$9()Lnet/minecraft/world/item/Item;
private static synthetic lambda$static$8()Lnet/minecraft/world/item/Item;
private static synthetic lambda$static$7()Lnet/minecraft/world/item/Item;
private static synthetic lambda$static$6()Lnet/minecraft/world/item/Item;
private static synthetic lambda$static$5()Lnet/minecraft/world/item/Item;
private static synthetic lambda$static$4()Lnet/minecraft/world/item/Item;
private static synthetic lambda$static$3()Lnet/minecraft/world/item/Item;
private static synthetic lambda$static$2()Lnet/minecraft/world/item/Item;
private static synthetic lambda$static$1()Lnet/minecraft/world/item/Item;
private static synthetic lambda$static$0()Lnet/minecraft/world/item/Item;
static <clinit>()V
```
