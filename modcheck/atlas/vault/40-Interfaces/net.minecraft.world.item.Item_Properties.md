---
type: "interface"
fqcn: "net.minecraft.world.item.Item$Properties"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.Item$Properties

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`class` public; extends `java/lang/Object`; implements `net/fabricmc/fabric/api/item/v1/FabricItem$Properties`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `componentInitializer` | `Lnet/minecraft/core/component/DataComponentInitializers$Initializer;` | exact | getfield@7 in `FabricItem$Properties.modifyComponents` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| reads | `id` | `Lnet/minecraft/resources/ResourceKey;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | declared |
| reads | `model` | `Lnet/minecraft/resources/DependantName;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | declared |
| writes | `componentInitializer` | `Lnet/minecraft/core/component/DataComponentInitializers$Initializer;` | exact | putfield@16 in `FabricItem$Properties.modifyComponents` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (8 fields, 65 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final BLOCK_DESCRIPTION_ID : Lnet/minecraft/resources/DependantName;
private static final ITEM_DESCRIPTION_ID : Lnet/minecraft/resources/DependantName;
private componentInitializer : Lnet/minecraft/core/component/DataComponentInitializers$Initializer;
private craftingRemainingItem : Lnet/minecraft/world/item/ItemStackTemplate;
private requiredFeatures : Lnet/minecraft/world/flag/FeatureFlagSet;
private id : Lnet/minecraft/resources/ResourceKey;
private descriptionId : Lnet/minecraft/resources/DependantName;
private final model : Lnet/minecraft/resources/DependantName;
public <init>()V
public food(Lnet/minecraft/world/food/FoodProperties;)Lnet/minecraft/world/item/Item$Properties;
public food(Lnet/minecraft/world/food/FoodProperties;Lnet/minecraft/world/item/component/Consumable;)Lnet/minecraft/world/item/Item$Properties;
public villagerFood(I)Lnet/minecraft/world/item/Item$Properties;
public usingConvertsTo(Lnet/minecraft/world/item/Item;)Lnet/minecraft/world/item/Item$Properties;
public useCooldown(F)Lnet/minecraft/world/item/Item$Properties;
public stacksTo(I)Lnet/minecraft/world/item/Item$Properties;
public durability(I)Lnet/minecraft/world/item/Item$Properties;
public craftRemainder(Lnet/minecraft/world/item/Item;)Lnet/minecraft/world/item/Item$Properties;
public craftRemainder(Lnet/minecraft/world/item/ItemStackTemplate;)Lnet/minecraft/world/item/Item$Properties;
public rarity(Lnet/minecraft/world/item/Rarity;)Lnet/minecraft/world/item/Item$Properties;
public fireResistant()Lnet/minecraft/world/item/Item$Properties;
public jukeboxPlayable(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/world/item/Item$Properties;
public enchantable(I)Lnet/minecraft/world/item/Item$Properties;
public potPattern(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/world/item/Item$Properties;
public repairable(Lnet/minecraft/world/item/Item;)Lnet/minecraft/world/item/Item$Properties;
public repairable(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/world/item/Item$Properties;
public equippable(Lnet/minecraft/world/entity/EquipmentSlot;)Lnet/minecraft/world/item/Item$Properties;
public equippableUnswappable(Lnet/minecraft/world/entity/EquipmentSlot;)Lnet/minecraft/world/item/Item$Properties;
public tool(Lnet/minecraft/world/item/ToolMaterial;Lnet/minecraft/tags/TagKey;FFF)Lnet/minecraft/world/item/Item$Properties;
public pickaxe(Lnet/minecraft/world/item/ToolMaterial;FF)Lnet/minecraft/world/item/Item$Properties;
public axe(Lnet/minecraft/world/item/ToolMaterial;FF)Lnet/minecraft/world/item/Item$Properties;
public hoe(Lnet/minecraft/world/item/ToolMaterial;FF)Lnet/minecraft/world/item/Item$Properties;
public shovel(Lnet/minecraft/world/item/ToolMaterial;FF)Lnet/minecraft/world/item/Item$Properties;
public sword(Lnet/minecraft/world/item/ToolMaterial;FF)Lnet/minecraft/world/item/Item$Properties;
public spear(Lnet/minecraft/world/item/ToolMaterial;FFFFFFFFF)Lnet/minecraft/world/item/Item$Properties;
public spawnEgg(Lnet/minecraft/world/entity/EntityType;)Lnet/minecraft/world/item/Item$Properties;
public humanoidArmor(Lnet/minecraft/world/item/equipment/ArmorMaterial;Lnet/minecraft/world/item/equipment/ArmorType;)Lnet/minecraft/world/item/Item$Properties;
public wolfArmor(Lnet/minecraft/world/item/equipment/ArmorMaterial;)Lnet/minecraft/world/item/Item$Properties;
public horseArmor(Lnet/minecraft/world/item/equipment/ArmorMaterial;)Lnet/minecraft/world/item/Item$Properties;
public nautilusArmor(Lnet/minecraft/world/item/equipment/ArmorMaterial;)Lnet/minecraft/world/item/Item$Properties;
public trimMaterial(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/world/item/Item$Properties;
public signText()Lnet/minecraft/world/item/Item$Properties;
public cookingFuel(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/world/item/Item$Properties;
public brewingFuel(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/world/item/Item$Properties;
public requiredFeatures([Lnet/minecraft/world/flag/FeatureFlag;)Lnet/minecraft/world/item/Item$Properties;
public requiredFeatures(Lnet/minecraft/world/flag/FeatureFlagSet;)Lnet/minecraft/world/item/Item$Properties;
public setId(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/world/item/Item$Properties;
public overrideDescription(Ljava/lang/String;)Lnet/minecraft/world/item/Item$Properties;
public useBlockDescriptionPrefix()Lnet/minecraft/world/item/Item$Properties;
public useItemDescriptionPrefix()Lnet/minecraft/world/item/Item$Properties;
public final itemIdOrThrow()Lnet/minecraft/resources/ResourceKey;
protected effectiveDescriptionId()Ljava/lang/String;
public effectiveModel()Lnet/minecraft/resources/Identifier;
public component(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)Lnet/minecraft/world/item/Item$Properties;
public delayedComponent(Lnet/minecraft/core/component/DataComponentType;Lnet/minecraft/core/component/DataComponentInitializers$SingleComponentInitializer;)Lnet/minecraft/world/item/Item$Properties;
public delayedHolderComponent(Lnet/minecraft/core/component/DataComponentType;Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/world/item/Item$Properties;
public attributes(Lnet/minecraft/world/item/component/ItemAttributeModifiers;)Lnet/minecraft/world/item/Item$Properties;
public compostable(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/world/item/Item$Properties;
public final loweredMobVisibility([Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/world/item/Item$Properties;
private finalizeInitializer(Lnet/minecraft/network/chat/Component;Lnet/minecraft/resources/Identifier;)Lnet/minecraft/core/component/DataComponentInitializers$Initializer;
private static synthetic lambda$finalizeInitializer$0(Lnet/minecraft/network/chat/Component;Lnet/minecraft/resources/Identifier;Lnet/minecraft/core/component/DataComponentMap$Builder;Lnet/minecraft/core/HolderLookup$Provider;Lnet/minecraft/resources/ResourceKey;)V
private static synthetic lambda$finalizeInitializer$1(Lnet/minecraft/core/component/DataComponentMap;)V
private static synthetic lambda$loweredMobVisibility$0([Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/world/item/component/MobVisibility;
private static synthetic lambda$delayedHolderComponent$0(Lnet/minecraft/core/component/DataComponentType;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/core/component/DataComponentMap$Builder;Lnet/minecraft/core/HolderLookup$Provider;Lnet/minecraft/resources/ResourceKey;)V
private static synthetic lambda$shovel$0(Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/core/Holder;
private static synthetic lambda$hoe$0(Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/core/Holder;
private static synthetic lambda$axe$0(Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/core/Holder;
private static synthetic lambda$potPattern$0(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/core/Holder;
private static synthetic lambda$jukeboxPlayable$0(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/world/item/JukeboxPlayable;
private static synthetic lambda$fireResistant$0(Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/world/item/component/DamageResistant;
private static synthetic lambda$new$0(Lnet/minecraft/core/component/DataComponentMap$Builder;Lnet/minecraft/core/HolderLookup$Provider;Lnet/minecraft/resources/ResourceKey;)V
private static synthetic lambda$static$1(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/String;
private static synthetic lambda$static$0(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/String;
static <clinit>()V
```
