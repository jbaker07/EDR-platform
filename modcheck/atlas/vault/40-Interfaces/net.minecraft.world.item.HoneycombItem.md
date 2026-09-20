---
type: "interface"
fqcn: "net.minecraft.world.item.HoneycombItem"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.HoneycombItem

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `lambda$static$0` | `@Inject at RETURN` | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |

## Declared members (25, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.item.HoneycombItem extends net.minecraft.world.item.Item implements net.minecraft.world.item.SignApplicator {
    public static final java.util.function.Supplier<com.google.common.collect.BiMap<net.minecraft.world.level.block.Block, net.minecraft.world.level.block.Block>> WAXABLES;
    public static final java.util.function.Supplier<com.google.common.collect.BiMap<net.minecraft.world.level.block.Block, net.minecraft.world.level.block.Block>> WAX_OFF_BY_BLOCK;
    public static final com.google.common.collect.ImmutableMap<net.minecraft.world.level.block.Block, com.mojang.datafixers.util.Pair<net.minecraft.data.recipes.RecipeCategory, java.lang.String>> WAXED_RECIPES;
    public net.minecraft.world.item.HoneycombItem(net.minecraft.world.item.Item$Properties);
    public net.minecraft.world.InteractionResult useOn(net.minecraft.world.item.context.UseOnContext);
    public static java.util.Optional<net.minecraft.world.level.block.state.BlockState> getWaxed(net.minecraft.world.level.block.state.BlockState);
    public boolean tryApplyToSign(net.minecraft.world.level.Level, net.minecraft.world.level.block.entity.SignBlockEntity, net.minecraft.world.level.block.entity.SignTextSlot, net.minecraft.world.item.ItemStack, net.minecraft.world.entity.player.Player);
    public boolean canApplyToSign(net.minecraft.world.level.block.entity.SignText, net.minecraft.world.item.ItemStack, net.minecraft.world.entity.player.Player);
    private static net.minecraft.world.level.block.state.BlockState lambda$getWaxed$0(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.block.Block);
    private static net.minecraft.world.InteractionResult lambda$useOn$0(net.minecraft.world.item.context.UseOnContext, net.minecraft.core.BlockPos, net.minecraft.world.level.Level, net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.block.state.BlockState);
    private static void lambda$static$13(com.google.common.collect.ImmutableMap$Builder, net.minecraft.world.item.HoneycombItem$WaxedRecipeGroup, net.minecraft.world.level.block.Block);
    private static com.mojang.datafixers.util.Pair lambda$static$12(net.minecraft.world.level.block.Block);
    private static com.mojang.datafixers.util.Pair lambda$static$11(net.minecraft.world.level.block.Block);
    private static com.mojang.datafixers.util.Pair lambda$static$10(net.minecraft.world.level.block.Block);
    private static com.mojang.datafixers.util.Pair lambda$static$9(net.minecraft.world.level.block.Block);
    private static com.mojang.datafixers.util.Pair lambda$static$8(net.minecraft.world.level.block.Block);
    private static com.mojang.datafixers.util.Pair lambda$static$7(net.minecraft.world.level.block.Block);
    private static com.mojang.datafixers.util.Pair lambda$static$6(net.minecraft.world.level.block.Block);
    private static com.mojang.datafixers.util.Pair lambda$static$5(net.minecraft.world.level.block.Block);
    private static com.mojang.datafixers.util.Pair lambda$static$4(net.minecraft.world.level.block.Block);
    private static com.mojang.datafixers.util.Pair lambda$static$3(net.minecraft.world.level.block.Block);
    private static com.google.common.collect.BiMap lambda$static$2();
    private static com.google.common.collect.BiMap lambda$static$0();
    private static void lambda$static$1(com.google.common.collect.ImmutableBiMap$Builder, net.minecraft.world.level.block.WeatheringCopperCollection);
    static {};
}
```
