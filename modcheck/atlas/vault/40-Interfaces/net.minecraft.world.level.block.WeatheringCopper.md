---
type: "interface"
fqcn: "net.minecraft.world.level.block.WeatheringCopper"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.WeatheringCopper

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `lambda$static$0` | `@Inject at RETURN` | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |

## Declared members (15, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.world.level.block.WeatheringCopper extends net.minecraft.world.level.block.ChangeOverTimeBlock<net.minecraft.world.level.block.WeatheringCopper$WeatherState> {
    public static final java.util.function.Supplier<com.google.common.collect.BiMap<net.minecraft.world.level.block.Block, net.minecraft.world.level.block.Block>> NEXT_BY_BLOCK;
    public static final java.util.function.Supplier<com.google.common.collect.BiMap<net.minecraft.world.level.block.Block, net.minecraft.world.level.block.Block>> PREVIOUS_BY_BLOCK;
    public static java.util.Optional<net.minecraft.world.level.block.Block> getPrevious(net.minecraft.world.level.block.Block);
    public static net.minecraft.world.level.block.Block getFirst(net.minecraft.world.level.block.Block);
    public static java.util.Optional<net.minecraft.world.level.block.state.BlockState> getPrevious(net.minecraft.world.level.block.state.BlockState);
    public static java.util.Optional<net.minecraft.world.level.block.Block> getNext(net.minecraft.world.level.block.Block);
    public static net.minecraft.world.level.block.state.BlockState getFirst(net.minecraft.world.level.block.state.BlockState);
    public default java.util.Optional<net.minecraft.world.level.block.state.BlockState> getNext(net.minecraft.world.level.block.state.BlockState);
    public default float getChanceModifier();
    private static net.minecraft.world.level.block.state.BlockState lambda$getNext$0(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.block.Block);
    private static net.minecraft.world.level.block.state.BlockState lambda$getPrevious$0(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.block.Block);
    private static com.google.common.collect.BiMap lambda$static$2();
    private static com.google.common.collect.BiMap lambda$static$0();
    private static void lambda$static$1(com.google.common.collect.ImmutableBiMap$Builder, net.minecraft.world.level.block.WeatheringCopperCollection);
    static {};
}
```
