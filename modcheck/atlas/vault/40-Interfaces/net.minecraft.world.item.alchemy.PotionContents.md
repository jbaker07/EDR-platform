---
type: "interface"
fqcn: "net.minecraft.world.item.alchemy.PotionContents"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.alchemy.PotionContents

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`record` public final; extends `java/lang/Record`; implements `net/minecraft/world/item/component/ConsumableListener`, `net/minecraft/world/item/component/TooltipProvider`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/core/Holder;)V` | exact | invokespecial@18 in `FluidStorage.lambda$static$4` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `potion` | `()Ljava/util/Optional;` | exact | invokevirtual@39 in `FabricItem.getCreatorNamespace` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `potion` | `()Ljava/util/Optional;` | exact | invokevirtual@41 in `WaterPotionStorage.isWaterPotion` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `EMPTY` | `Lnet/minecraft/world/item/alchemy/PotionContents;` | exact | getstatic@16 in `WaterPotionStorage.isWaterPotion` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `EMPTY` | `Lnet/minecraft/world/item/alchemy/PotionContents;` | exact | getstatic@19 in `WaterPotionStorage.mapToGlassBottle` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (10 fields, 33 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final potion : Ljava/util/Optional;
private final customColor : Ljava/util/Optional;
private final customEffects : Ljava/util/List;
private final customName : Ljava/util/Optional;
public static final EMPTY : Lnet/minecraft/world/item/alchemy/PotionContents;
private static final NO_EFFECT : Lnet/minecraft/network/chat/Component;
public static final BASE_POTION_COLOR : I
private static final FULL_CODEC : Lcom/mojang/serialization/Codec;
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public <init>(Lnet/minecraft/core/Holder;)V
public <init>(Ljava/util/Optional;Ljava/util/Optional;Ljava/util/List;Ljava/util/Optional;)V
public static createItemStack(Lnet/minecraft/world/item/Item;Lnet/minecraft/core/Holder;)Lnet/minecraft/world/item/ItemStack;
public is(Lnet/minecraft/core/Holder;)Z
public is(Lnet/minecraft/tags/TagKey;)Z
public isPotionWithoutCustomEffects()Z
public getAllEffects()Ljava/lang/Iterable;
public forEachEffect(Ljava/util/function/Consumer;F)V
public withPotion(Lnet/minecraft/core/Holder;)Lnet/minecraft/world/item/alchemy/PotionContents;
public withEffectAdded(Lnet/minecraft/world/effect/MobEffectInstance;)Lnet/minecraft/world/item/alchemy/PotionContents;
public getColor()I
public getColorOr(I)I
public getName(Ljava/lang/String;)Lnet/minecraft/network/chat/Component;
public static getColorOptional(Ljava/lang/Iterable;)Ljava/util/OptionalInt;
public hasEffects()Z
public customEffects()Ljava/util/List;
public applyToLivingEntity(Lnet/minecraft/world/entity/LivingEntity;F)V
public static addPotionTooltip(Ljava/lang/Iterable;Ljava/util/function/Consumer;FF)V
public static getPotionDescription(Lnet/minecraft/core/Holder;I)Lnet/minecraft/network/chat/MutableComponent;
public onConsume(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/component/Consumable;)V
public addToTooltip(Lnet/minecraft/world/item/Item$TooltipContext;Ljava/util/function/Consumer;Lnet/minecraft/world/item/TooltipFlag;Lnet/minecraft/core/component/DataComponentGetter;)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public potion()Ljava/util/Optional;
public customColor()Ljava/util/Optional;
public customName()Ljava/util/Optional;
private static synthetic lambda$addPotionTooltip$0(Ljava/util/List;Lnet/minecraft/core/Holder;Lnet/minecraft/world/entity/ai/attributes/AttributeModifier;)V
private static synthetic lambda$applyToLivingEntity$0(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/effect/MobEffectInstance;)V
private synthetic lambda$getName$0()Ljava/util/Optional;
private static synthetic lambda$getName$1(Lnet/minecraft/core/Holder;)Ljava/lang/String;
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
static <clinit>()V
```
