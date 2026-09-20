---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.BuiltInBlockModels"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.BuiltInBlockModels

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `createBlockModels` | `@Inject at INVOKE Lnet/minecraft/client/renderer/block/BuiltInBlockModels$Builde` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (46, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.block.BuiltInBlockModels {
    public net.minecraft.client.renderer.block.BuiltInBlockModels();
    private static void addDefaults(net.minecraft.client.renderer.block.BuiltInBlockModels$Builder);
    private static void createAir(net.minecraft.client.renderer.block.BuiltInBlockModels$Builder, net.minecraft.world.level.block.Block);
    private static net.minecraft.client.renderer.block.model.BlockModel$Unbaked special(net.minecraft.client.renderer.special.SpecialModelRenderer$Unbaked<?>);
    private static net.minecraft.client.renderer.block.model.BlockModel$Unbaked special(net.minecraft.client.renderer.special.SpecialModelRenderer$Unbaked<?>, com.mojang.math.Transformation);
    private static net.minecraft.client.renderer.block.BuiltInBlockModels$SpecialModelFactory createMobHead(net.minecraft.world.level.block.SkullBlock$Types);
    private static net.minecraft.client.renderer.block.BuiltInBlockModels$SpecialModelFactory createMobWallHead(net.minecraft.world.level.block.SkullBlock$Types);
    private static void createMobHeads(net.minecraft.client.renderer.block.BuiltInBlockModels$Builder, net.minecraft.world.level.block.SkullBlock$Types, net.minecraft.world.level.block.Block, net.minecraft.world.level.block.Block);
    private static net.minecraft.client.renderer.block.BuiltInBlockModels$SpecialModelFactory createPlayerHead();
    private static net.minecraft.client.renderer.block.BuiltInBlockModels$SpecialModelFactory createPlayerWallHead();
    private static net.minecraft.client.renderer.block.BuiltInBlockModels$SpecialModelFactory createBanner(net.minecraft.world.item.DyeColor);
    private static net.minecraft.client.renderer.block.BuiltInBlockModels$SpecialModelFactory createWallBanner(net.minecraft.world.item.DyeColor);
    private static net.minecraft.client.renderer.block.BuiltInBlockModels$SpecialModelFactory createShulkerBox();
    private static net.minecraft.client.renderer.block.BuiltInBlockModels$SpecialModelFactory createDyedShulkerBox(net.minecraft.world.item.DyeColor);
    private static net.minecraft.client.renderer.block.model.BlockModel$Unbaked createChest(net.minecraft.resources.Identifier, net.minecraft.world.level.block.state.properties.ChestType, net.minecraft.core.Direction);
    private static net.minecraft.client.renderer.block.BuiltInBlockModels$SpecialModelFactory createSingletonChest(net.minecraft.resources.Identifier);
    private static net.minecraft.client.renderer.block.BuiltInBlockModels$SpecialModelFactory createChest(net.minecraft.client.renderer.MultiblockChestResources<net.minecraft.resources.Identifier>);
    private static net.minecraft.client.renderer.block.BuiltInBlockModels$SpecialModelFactory createXmasChest(net.minecraft.client.renderer.MultiblockChestResources<net.minecraft.resources.Identifier>);
    private static net.minecraft.client.renderer.block.BuiltInBlockModels$SpecialModelFactory createCopperGolem(net.minecraft.world.level.block.WeatheringCopper$WeatherState);
    private static net.minecraft.client.renderer.block.BuiltInBlockModels$SpecialModelFactory createDecoratedPot();
    private static net.minecraft.client.renderer.block.model.BlockStateModelWrapper$Unbaked createBlockStateModelWrapper(net.minecraft.client.color.block.BlockColors, net.minecraft.world.level.block.state.BlockState);
    private static net.minecraft.client.renderer.block.model.CompositeBlockModel$Unbaked combineSpecialAndBlockModels(net.minecraft.client.renderer.block.model.BlockModel$Unbaked, net.minecraft.client.color.block.BlockColors, net.minecraft.world.level.block.state.BlockState);
    private static net.minecraft.client.renderer.block.SelectBlockModel$Unbaked createFlowerBedModel(net.minecraft.client.color.block.BlockColors, net.minecraft.world.level.block.state.BlockState);
    private static net.minecraft.client.renderer.block.model.BlockModel$Unbaked createEnchantingTable();
    private static <P extends java.lang.Comparable<P>> net.minecraft.client.renderer.block.BuiltInBlockModels$SpecialModelFactory specialModelWithPropertyDispatch(net.minecraft.world.level.block.state.properties.Property<P>, java.util.function.Function<P, net.minecraft.client.renderer.block.model.BlockModel$Unbaked>);
    private static <P1 extends java.lang.Comparable<P1>, P2 extends java.lang.Comparable<P2>> net.minecraft.client.renderer.block.BuiltInBlockModels$SpecialModelFactory specialModelWithPropertyDispatch(net.minecraft.world.level.block.state.properties.Property<P1>, net.minecraft.world.level.block.state.properties.Property<P2>, java.util.function.BiFunction<P1, P2, net.minecraft.client.renderer.block.model.BlockModel$Unbaked>);
    public static java.util.Map<net.minecraft.world.level.block.state.BlockState, net.minecraft.client.renderer.block.model.BlockModel$Unbaked> createBlockModels(net.minecraft.client.color.block.BlockColors);
    private static net.minecraft.client.renderer.block.model.BlockModel$Unbaked lambda$specialModelWithPropertyDispatch$1(net.minecraft.world.level.block.state.properties.Property, net.minecraft.world.level.block.state.properties.Property, java.util.function.BiFunction, net.minecraft.world.level.block.state.BlockState);
    private static net.minecraft.client.renderer.block.model.BlockModel$Unbaked lambda$specialModelWithPropertyDispatch$0(net.minecraft.world.level.block.state.properties.Property, java.util.function.Function, net.minecraft.world.level.block.state.BlockState);
    private static net.minecraft.client.renderer.block.model.BlockModel$Unbaked lambda$createDecoratedPot$0(net.minecraft.core.Direction);
    private static net.minecraft.client.renderer.block.model.BlockModel$Unbaked lambda$createCopperGolem$0(net.minecraft.world.level.block.WeatheringCopper$WeatherState, net.minecraft.core.Direction, net.minecraft.world.level.block.CopperGolemStatueBlock$Pose);
    private static net.minecraft.client.renderer.block.model.BlockModel$Unbaked lambda$createXmasChest$0(net.minecraft.client.renderer.MultiblockChestResources, net.minecraft.core.Direction, net.minecraft.world.level.block.state.properties.ChestType);
    private static net.minecraft.client.renderer.block.model.BlockModel$Unbaked lambda$createChest$0(net.minecraft.client.renderer.MultiblockChestResources, net.minecraft.core.Direction, net.minecraft.world.level.block.state.properties.ChestType);
    private static net.minecraft.client.renderer.block.model.BlockModel$Unbaked lambda$createSingletonChest$0(net.minecraft.resources.Identifier, net.minecraft.core.Direction);
    private static net.minecraft.client.renderer.block.model.BlockModel$Unbaked lambda$createDyedShulkerBox$0(net.minecraft.world.item.DyeColor, net.minecraft.core.Direction);
    private static net.minecraft.client.renderer.block.model.BlockModel$Unbaked lambda$createShulkerBox$0(net.minecraft.core.Direction);
    private static net.minecraft.client.renderer.block.model.BlockModel$Unbaked lambda$createWallBanner$0(net.minecraft.world.item.DyeColor, net.minecraft.core.Direction);
    private static net.minecraft.client.renderer.block.model.BlockModel$Unbaked lambda$createBanner$0(net.minecraft.world.item.DyeColor, java.lang.Integer);
    private static net.minecraft.client.renderer.block.model.BlockModel$Unbaked lambda$createPlayerWallHead$0(net.minecraft.core.Direction);
    private static net.minecraft.client.renderer.block.model.BlockModel$Unbaked lambda$createPlayerHead$0(java.lang.Integer);
    private static net.minecraft.client.renderer.block.model.BlockModel$Unbaked lambda$createMobWallHead$0(net.minecraft.world.level.block.SkullBlock$Types, net.minecraft.core.Direction);
    private static net.minecraft.client.renderer.block.model.BlockModel$Unbaked lambda$createMobHead$0(net.minecraft.world.level.block.SkullBlock$Types, java.lang.Integer);
    private static void lambda$addDefaults$3(net.minecraft.client.renderer.block.BuiltInBlockModels$Builder, net.minecraft.world.level.block.WeatheringCopper$WeatherState);
    private static void lambda$addDefaults$2(net.minecraft.client.renderer.block.BuiltInBlockModels$Builder, net.minecraft.world.item.DyeColor, net.minecraft.world.level.block.Block);
    private static void lambda$addDefaults$1(net.minecraft.client.renderer.block.BuiltInBlockModels$Builder, net.minecraft.world.item.DyeColor, net.minecraft.world.level.block.Block);
    private static void lambda$addDefaults$0(net.minecraft.client.renderer.block.BuiltInBlockModels$Builder, net.minecraft.world.item.DyeColor, net.minecraft.world.level.block.Block);
}
```
