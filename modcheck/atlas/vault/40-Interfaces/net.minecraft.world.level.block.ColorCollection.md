---
type: "interface"
fqcn: "net.minecraft.world.level.block.ColorCollection"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.ColorCollection

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Obje` | exact | invokespecial@790 in `ConventionalBlockItemTags.<clinit>` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `<init>` | `(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Obje` | exact | invokespecial@835 in `ConventionalBlockTags.<clinit>` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `<init>` | `(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Obje` | exact | invokespecial@1605 in `ConventionalItemTags.<clinit>` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `<init>` | `(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Obje` | exact | invokespecial@1816 in `ConventionalItemTags.<clinit>` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `forEach` | `(Ljava/util/function/Consumer;)V` | exact | invokevirtual@116 in `ItemStorage.<clinit>` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `forEach` | `(Ljava/util/function/Consumer;)V` | exact | invokevirtual@150 in `ItemStorage.<clinit>` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `red` | `()Ljava/lang/Object;` | exact | invokevirtual@39 in `LivingEntityMixin.modifyBedForOccupiedState` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |

## Declared members (18 fields, 37 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final white : Ljava/lang/Object;
private final orange : Ljava/lang/Object;
private final magenta : Ljava/lang/Object;
private final lightBlue : Ljava/lang/Object;
private final yellow : Ljava/lang/Object;
private final lime : Ljava/lang/Object;
private final pink : Ljava/lang/Object;
private final gray : Ljava/lang/Object;
private final lightGray : Ljava/lang/Object;
private final cyan : Ljava/lang/Object;
private final purple : Ljava/lang/Object;
private final blue : Ljava/lang/Object;
private final brown : Ljava/lang/Object;
private final green : Ljava/lang/Object;
private final red : Ljava/lang/Object;
private final black : Ljava/lang/Object;
public static final VALUES : Lnet/minecraft/world/level/block/ColorCollection;
public static final NAMES : Lnet/minecraft/world/level/block/ColorCollection;
public <init>(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V
public static create(Ljava/lang/Object;)Lnet/minecraft/world/level/block/ColorCollection;
public static registerBlocks(Lnet/minecraft/world/level/block/ColorCollection;Lorg/apache/commons/lang3/function/TriFunction;Ljava/util/function/BiFunction;Ljava/util/function/Function;)Lnet/minecraft/world/level/block/ColorCollection;
public static registerBlockItems(Lnet/minecraft/world/level/block/ColorCollection;Lnet/minecraft/world/level/block/ColorCollection;Lorg/apache/commons/lang3/function/TriFunction;)Lnet/minecraft/world/level/block/ColorCollection;
public static registerItems(Lnet/minecraft/world/level/block/ColorCollection;Ljava/util/function/BiFunction;)Lnet/minecraft/world/level/block/ColorCollection;
public static prefixWithColor(Lnet/minecraft/world/level/block/ColorCollection;)Lnet/minecraft/world/level/block/ColorCollection;
public asList()Ljava/util/List;
public forEach(Ljava/util/function/Consumer;)V
public pick(Lnet/minecraft/world/item/DyeColor;)Ljava/lang/Object;
public map(Ljava/util/function/Function;)Lnet/minecraft/world/level/block/ColorCollection;
public static zipApply(Lnet/minecraft/world/level/block/ColorCollection;Lnet/minecraft/world/level/block/ColorCollection;Ljava/util/function/BiConsumer;)V
public static zipMap(Lnet/minecraft/world/level/block/ColorCollection;Lnet/minecraft/world/level/block/ColorCollection;Ljava/util/function/BiFunction;)Lnet/minecraft/world/level/block/ColorCollection;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public white()Ljava/lang/Object;
public orange()Ljava/lang/Object;
public magenta()Ljava/lang/Object;
public lightBlue()Ljava/lang/Object;
public yellow()Ljava/lang/Object;
public lime()Ljava/lang/Object;
public pink()Ljava/lang/Object;
public gray()Ljava/lang/Object;
public lightGray()Ljava/lang/Object;
public cyan()Ljava/lang/Object;
public purple()Ljava/lang/Object;
public blue()Ljava/lang/Object;
public brown()Ljava/lang/Object;
public green()Ljava/lang/Object;
public red()Ljava/lang/Object;
public black()Ljava/lang/Object;
private static synthetic lambda$prefixWithColor$0(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
private static synthetic lambda$registerItems$0(Ljava/util/function/BiFunction;Lnet/minecraft/world/item/DyeColor;Ljava/lang/Object;)Lnet/minecraft/world/item/Item;
private static synthetic lambda$registerBlockItems$0(Lorg/apache/commons/lang3/function/TriFunction;Lnet/minecraft/world/level/block/ColorCollection;Lnet/minecraft/world/item/DyeColor;Ljava/lang/Object;)Lnet/minecraft/world/item/Item;
private static synthetic lambda$registerBlocks$0(Lorg/apache/commons/lang3/function/TriFunction;Ljava/util/function/BiFunction;Ljava/util/function/Function;Lnet/minecraft/world/item/DyeColor;Ljava/lang/Object;)Lnet/minecraft/world/level/block/Block;
private static synthetic lambda$registerBlocks$1(Ljava/util/function/BiFunction;Lnet/minecraft/world/item/DyeColor;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)Lnet/minecraft/world/level/block/Block;
static <clinit>()V
```
