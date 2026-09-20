---
type: "interface"
fqcn: "net.minecraft.world.level.storage.loot.functions.EnchantRandomlyFunction"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.loot.functions.EnchantRandomlyFunction

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `net/minecraft/world/level/storage/loot/functions/LootItemConditionalFunction`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| wraps | `lambda$run$1` | `(ZLnet/minecraft/world/item/ItemStack;Lnet/minecraft/core/Holder;)Z` | name_only | @Redirect at ['INVOKE'] | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (5 fields, 14 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
public static final MAP_CODEC : Lcom/mojang/serialization/MapCodec;
private final options : Ljava/util/Optional;
private final onlyCompatible : Z
private final includeAdditionalCostComponent : Z
private <init>(Ljava/util/Optional;Ljava/util/Optional;ZZ)V
public codec()Lcom/mojang/serialization/MapCodec;
public getReferencedContextParams()Ljava/util/Set;
public run(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/level/storage/loot/LootContext;)Lnet/minecraft/world/item/ItemStack;
private enchantItem(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/core/Holder;Lnet/minecraft/world/level/storage/loot/LootContext;)Lnet/minecraft/world/item/ItemStack;
public static randomEnchantment()Lnet/minecraft/world/level/storage/loot/functions/EnchantRandomlyFunction$Builder;
public static randomApplicableEnchantment(Lnet/minecraft/core/HolderGetter;)Lnet/minecraft/world/level/storage/loot/functions/EnchantRandomlyFunction$Builder;
private static synthetic lambda$run$1(ZLnet/minecraft/world/item/ItemStack;Lnet/minecraft/core/Holder;)Z
private static synthetic lambda$run$0(Lnet/minecraft/world/level/storage/loot/LootContext;)Ljava/util/stream/Stream;
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
private static synthetic lambda$static$3(Lnet/minecraft/world/level/storage/loot/functions/EnchantRandomlyFunction;)Ljava/lang/Boolean;
private static synthetic lambda$static$2(Lnet/minecraft/world/level/storage/loot/functions/EnchantRandomlyFunction;)Ljava/lang/Boolean;
private static synthetic lambda$static$1(Lnet/minecraft/world/level/storage/loot/functions/EnchantRandomlyFunction;)Ljava/util/Optional;
static <clinit>()V
```
