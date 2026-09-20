---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.BuiltInBlockModels"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.BuiltInBlockModels

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `createBlockModels` | `(Lnet/minecraft/client/color/block/BlockColors;)Ljava/util/Map;` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (0 fields, 46 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public <init>()V
public static addDefaults(Lnet/minecraft/client/renderer/block/BuiltInBlockModels$Builder;)V
public static createAir(Lnet/minecraft/client/renderer/block/BuiltInBlockModels$Builder;Lnet/minecraft/world/level/block/Block;)V
public static special(Lnet/minecraft/client/renderer/special/SpecialModelRenderer$Unbaked;)Lnet/minecraft/client/renderer/block/model/BlockModel$Unbaked;
public static special(Lnet/minecraft/client/renderer/special/SpecialModelRenderer$Unbaked;Lcom/mojang/math/Transformation;)Lnet/minecraft/client/renderer/block/model/BlockModel$Unbaked;
public static createMobHead(Lnet/minecraft/world/level/block/SkullBlock$Types;)Lnet/minecraft/client/renderer/block/BuiltInBlockModels$SpecialModelFactory;
public static createMobWallHead(Lnet/minecraft/world/level/block/SkullBlock$Types;)Lnet/minecraft/client/renderer/block/BuiltInBlockModels$SpecialModelFactory;
public static createMobHeads(Lnet/minecraft/client/renderer/block/BuiltInBlockModels$Builder;Lnet/minecraft/world/level/block/SkullBlock$Types;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V
public static createPlayerHead()Lnet/minecraft/client/renderer/block/BuiltInBlockModels$SpecialModelFactory;
public static createPlayerWallHead()Lnet/minecraft/client/renderer/block/BuiltInBlockModels$SpecialModelFactory;
public static createBanner(Lnet/minecraft/world/item/DyeColor;)Lnet/minecraft/client/renderer/block/BuiltInBlockModels$SpecialModelFactory;
public static createWallBanner(Lnet/minecraft/world/item/DyeColor;)Lnet/minecraft/client/renderer/block/BuiltInBlockModels$SpecialModelFactory;
public static createShulkerBox()Lnet/minecraft/client/renderer/block/BuiltInBlockModels$SpecialModelFactory;
public static createDyedShulkerBox(Lnet/minecraft/world/item/DyeColor;)Lnet/minecraft/client/renderer/block/BuiltInBlockModels$SpecialModelFactory;
public static createChest(Lnet/minecraft/resources/Identifier;Lnet/minecraft/world/level/block/state/properties/ChestType;Lnet/minecraft/core/Direction;)Lnet/minecraft/client/renderer/block/model/BlockModel$Unbaked;
public static createSingletonChest(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/block/BuiltInBlockModels$SpecialModelFactory;
public static createChest(Lnet/minecraft/client/renderer/MultiblockChestResources;)Lnet/minecraft/client/renderer/block/BuiltInBlockModels$SpecialModelFactory;
public static createXmasChest(Lnet/minecraft/client/renderer/MultiblockChestResources;)Lnet/minecraft/client/renderer/block/BuiltInBlockModels$SpecialModelFactory;
public static createCopperGolem(Lnet/minecraft/world/level/block/WeatheringCopper$WeatherState;)Lnet/minecraft/client/renderer/block/BuiltInBlockModels$SpecialModelFactory;
public static createDecoratedPot()Lnet/minecraft/client/renderer/block/BuiltInBlockModels$SpecialModelFactory;
public static createBlockStateModelWrapper(Lnet/minecraft/client/color/block/BlockColors;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/client/renderer/block/model/BlockStateModelWrapper$Unbaked;
public static combineSpecialAndBlockModels(Lnet/minecraft/client/renderer/block/model/BlockModel$Unbaked;Lnet/minecraft/client/color/block/BlockColors;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/client/renderer/block/model/CompositeBlockModel$Unbaked;
public static createFlowerBedModel(Lnet/minecraft/client/color/block/BlockColors;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/client/renderer/block/SelectBlockModel$Unbaked;
public static createEnchantingTable()Lnet/minecraft/client/renderer/block/model/BlockModel$Unbaked;
public static specialModelWithPropertyDispatch(Lnet/minecraft/world/level/block/state/properties/Property;Ljava/util/function/Function;)Lnet/minecraft/client/renderer/block/BuiltInBlockModels$SpecialModelFactory;
public static specialModelWithPropertyDispatch(Lnet/minecraft/world/level/block/state/properties/Property;Lnet/minecraft/world/level/block/state/properties/Property;Ljava/util/function/BiFunction;)Lnet/minecraft/client/renderer/block/BuiltInBlockModels$SpecialModelFactory;
public static createBlockModels(Lnet/minecraft/client/color/block/BlockColors;)Ljava/util/Map;
private static synthetic lambda$specialModelWithPropertyDispatch$1(Lnet/minecraft/world/level/block/state/properties/Property;Lnet/minecraft/world/level/block/state/properties/Property;Ljava/util/function/BiFunction;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/client/renderer/block/model/BlockModel$Unbaked;
private static synthetic lambda$specialModelWithPropertyDispatch$0(Lnet/minecraft/world/level/block/state/properties/Property;Ljava/util/function/Function;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/client/renderer/block/model/BlockModel$Unbaked;
private static synthetic lambda$createDecoratedPot$0(Lnet/minecraft/core/Direction;)Lnet/minecraft/client/renderer/block/model/BlockModel$Unbaked;
private static synthetic lambda$createCopperGolem$0(Lnet/minecraft/world/level/block/WeatheringCopper$WeatherState;Lnet/minecraft/core/Direction;Lnet/minecraft/world/level/block/CopperGolemStatueBlock$Pose;)Lnet/minecraft/client/renderer/block/model/BlockModel$Unbaked;
private static synthetic lambda$createXmasChest$0(Lnet/minecraft/client/renderer/MultiblockChestResources;Lnet/minecraft/core/Direction;Lnet/minecraft/world/level/block/state/properties/ChestType;)Lnet/minecraft/client/renderer/block/model/BlockModel$Unbaked;
private static synthetic lambda$createChest$0(Lnet/minecraft/client/renderer/MultiblockChestResources;Lnet/minecraft/core/Direction;Lnet/minecraft/world/level/block/state/properties/ChestType;)Lnet/minecraft/client/renderer/block/model/BlockModel$Unbaked;
private static synthetic lambda$createSingletonChest$0(Lnet/minecraft/resources/Identifier;Lnet/minecraft/core/Direction;)Lnet/minecraft/client/renderer/block/model/BlockModel$Unbaked;
private static synthetic lambda$createDyedShulkerBox$0(Lnet/minecraft/world/item/DyeColor;Lnet/minecraft/core/Direction;)Lnet/minecraft/client/renderer/block/model/BlockModel$Unbaked;
private static synthetic lambda$createShulkerBox$0(Lnet/minecraft/core/Direction;)Lnet/minecraft/client/renderer/block/model/BlockModel$Unbaked;
private static synthetic lambda$createWallBanner$0(Lnet/minecraft/world/item/DyeColor;Lnet/minecraft/core/Direction;)Lnet/minecraft/client/renderer/block/model/BlockModel$Unbaked;
private static synthetic lambda$createBanner$0(Lnet/minecraft/world/item/DyeColor;Ljava/lang/Integer;)Lnet/minecraft/client/renderer/block/model/BlockModel$Unbaked;
private static synthetic lambda$createPlayerWallHead$0(Lnet/minecraft/core/Direction;)Lnet/minecraft/client/renderer/block/model/BlockModel$Unbaked;
private static synthetic lambda$createPlayerHead$0(Ljava/lang/Integer;)Lnet/minecraft/client/renderer/block/model/BlockModel$Unbaked;
private static synthetic lambda$createMobWallHead$0(Lnet/minecraft/world/level/block/SkullBlock$Types;Lnet/minecraft/core/Direction;)Lnet/minecraft/client/renderer/block/model/BlockModel$Unbaked;
private static synthetic lambda$createMobHead$0(Lnet/minecraft/world/level/block/SkullBlock$Types;Ljava/lang/Integer;)Lnet/minecraft/client/renderer/block/model/BlockModel$Unbaked;
private static synthetic lambda$addDefaults$3(Lnet/minecraft/client/renderer/block/BuiltInBlockModels$Builder;Lnet/minecraft/world/level/block/WeatheringCopper$WeatherState;)V
private static synthetic lambda$addDefaults$2(Lnet/minecraft/client/renderer/block/BuiltInBlockModels$Builder;Lnet/minecraft/world/item/DyeColor;Lnet/minecraft/world/level/block/Block;)V
private static synthetic lambda$addDefaults$1(Lnet/minecraft/client/renderer/block/BuiltInBlockModels$Builder;Lnet/minecraft/world/item/DyeColor;Lnet/minecraft/world/level/block/Block;)V
private static synthetic lambda$addDefaults$0(Lnet/minecraft/client/renderer/block/BuiltInBlockModels$Builder;Lnet/minecraft/world/item/DyeColor;Lnet/minecraft/world/level/block/Block;)V
```
