---
type: "interface"
fqcn: "net.minecraft.world.level.block.WeatheringCopperCollection"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.WeatheringCopperCollection

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `weathering()Lnet/minecraft/world/level/block/WeatheringCopperCollectio` | `` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `zipUnwaxedWaxed(Ljava/util/function/BiConsumer;)V` | `` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |

## Declared members (36, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.level.block.WeatheringCopperCollection<T> extends java.lang.Record {
    private final net.minecraft.world.level.block.WeatheringCopperCollection$ByState<T> weathering;
    private final net.minecraft.world.level.block.WeatheringCopperCollection$ByState<T> waxed;
    public static final net.minecraft.world.level.block.WeatheringCopperCollection$ByState<net.minecraft.world.level.block.WeatheringCopper$WeatherState> STATES;
    public static final net.minecraft.world.level.block.WeatheringCopperCollection<java.lang.String> PREFIXES;
    public net.minecraft.world.level.block.WeatheringCopperCollection(net.minecraft.world.level.block.WeatheringCopperCollection$ByState<T>, net.minecraft.world.level.block.WeatheringCopperCollection$ByState<T>);
    public static net.minecraft.world.level.block.WeatheringCopperCollection<java.lang.String> prefixWithState(net.minecraft.world.level.block.WeatheringCopperCollection<java.lang.String>);
    public static net.minecraft.world.level.block.WeatheringCopperCollection<java.lang.String> create(java.lang.String);
    public static net.minecraft.world.level.block.WeatheringCopperCollection<java.lang.String> same(net.minecraft.world.level.block.WeatheringCopperCollection$ByState<java.lang.String>);
    public static <WaxedBlock extends net.minecraft.world.level.block.Block, WeatheringBlock extends net.minecraft.world.level.block.Block & net.minecraft.world.level.block.WeatheringCopper, Id> net.minecraft.world.level.block.WeatheringCopperCollection<net.minecraft.world.level.block.Block> registerBlocks(net.minecraft.world.level.block.WeatheringCopperCollection<Id>, org.apache.commons.lang3.function.TriFunction<Id, java.util.function.Function<net.minecraft.world.level.block.state.BlockBehaviour$Properties, net.minecraft.world.level.block.Block>, net.minecraft.world.level.block.state.BlockBehaviour$Properties, net.minecraft.world.level.block.Block>, java.util.function.BiFunction<net.minecraft.world.level.block.WeatheringCopper$WeatherState, net.minecraft.world.level.block.state.BlockBehaviour$Properties, WaxedBlock>, java.util.function.BiFunction<net.minecraft.world.level.block.WeatheringCopper$WeatherState, net.minecraft.world.level.block.state.BlockBehaviour$Properties, WeatheringBlock>, java.util.function.Function<net.minecraft.world.level.block.WeatheringCopper$WeatherState, net.minecraft.world.level.block.state.BlockBehaviour$Properties>);
    public static <Id> net.minecraft.world.level.block.WeatheringCopperCollection<net.minecraft.world.item.Item> registerItems(net.minecraft.world.level.block.WeatheringCopperCollection<Id>, net.minecraft.world.level.block.WeatheringCopperCollection<net.minecraft.world.level.block.Block>, java.util.function.BiFunction<Id, net.minecraft.world.level.block.Block, net.minecraft.world.item.Item>);
    public static net.minecraft.world.level.block.WeatheringCopperCollection<net.minecraft.data.BlockFamily> createFamily(java.util.function.BiFunction<java.lang.String, net.minecraft.world.level.block.WeatheringCopper$WeatherState, net.minecraft.data.BlockFamily>, java.util.function.BiFunction<java.lang.String, net.minecraft.world.level.block.WeatheringCopper$WeatherState, net.minecraft.data.BlockFamily>);
    public java.util.List<T> asList();
    public void forEach(java.util.function.Consumer<T>);
    public <U> net.minecraft.world.level.block.WeatheringCopperCollection<U> map(java.util.function.Function<T, U>);
    public <U> net.minecraft.world.level.block.WeatheringCopperCollection<U> apply(java.util.function.Function<net.minecraft.world.level.block.WeatheringCopperCollection$ByState<T>, net.minecraft.world.level.block.WeatheringCopperCollection$ByState<U>>);
    public <U> net.minecraft.world.level.block.WeatheringCopperCollection<U> apply(java.util.function.Function<net.minecraft.world.level.block.WeatheringCopperCollection$ByState<T>, net.minecraft.world.level.block.WeatheringCopperCollection$ByState<U>>, java.util.function.Function<net.minecraft.world.level.block.WeatheringCopperCollection$ByState<T>, net.minecraft.world.level.block.WeatheringCopperCollection$ByState<U>>);
    public static <T, U> void zipApply(net.minecraft.world.level.block.WeatheringCopperCollection<T>, net.minecraft.world.level.block.WeatheringCopperCollection<U>, java.util.function.BiConsumer<T, U>);
    public static <T, U, R> net.minecraft.world.level.block.WeatheringCopperCollection<R> zipMap(net.minecraft.world.level.block.WeatheringCopperCollection<T>, net.minecraft.world.level.block.WeatheringCopperCollection<U>, java.util.function.BiFunction<T, U, R>);
    public void zipUnwaxedWaxed(java.util.function.BiConsumer<T, T>);
    public static <T, U> void zipApply(net.minecraft.world.level.block.WeatheringCopperCollection$ByState<T>, net.minecraft.world.level.block.WeatheringCopperCollection$ByState<U>, java.util.function.BiConsumer<T, U>);
    public static <T, U, R> net.minecraft.world.level.block.WeatheringCopperCollection$ByState<R> zipMap(net.minecraft.world.level.block.WeatheringCopperCollection$ByState<T>, net.minecraft.world.level.block.WeatheringCopperCollection$ByState<U>, java.util.function.BiFunction<T, U, R>);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.world.level.block.WeatheringCopperCollection$ByState<T> weathering();
    public net.minecraft.world.level.block.WeatheringCopperCollection$ByState<T> waxed();
    private static net.minecraft.world.level.block.WeatheringCopperCollection$ByState lambda$createFamily$1(java.util.function.BiFunction, net.minecraft.world.level.block.WeatheringCopperCollection$ByState);
    private static net.minecraft.world.level.block.WeatheringCopperCollection$ByState lambda$createFamily$0(java.util.function.BiFunction, net.minecraft.world.level.block.WeatheringCopperCollection$ByState);
    private static net.minecraft.world.level.block.WeatheringCopperCollection$ByState lambda$registerBlocks$3(org.apache.commons.lang3.function.TriFunction, java.util.function.BiFunction, java.util.function.Function, net.minecraft.world.level.block.WeatheringCopperCollection$ByState);
    private static net.minecraft.world.level.block.Block lambda$registerBlocks$4(org.apache.commons.lang3.function.TriFunction, java.util.function.BiFunction, java.util.function.Function, net.minecraft.world.level.block.WeatheringCopper$WeatherState, java.lang.Object);
    private static net.minecraft.world.level.block.Block lambda$registerBlocks$5(java.util.function.BiFunction, net.minecraft.world.level.block.WeatheringCopper$WeatherState, net.minecraft.world.level.block.state.BlockBehaviour$Properties);
    private static net.minecraft.world.level.block.WeatheringCopperCollection$ByState lambda$registerBlocks$0(org.apache.commons.lang3.function.TriFunction, java.util.function.BiFunction, java.util.function.Function, net.minecraft.world.level.block.WeatheringCopperCollection$ByState);
    private static net.minecraft.world.level.block.Block lambda$registerBlocks$1(org.apache.commons.lang3.function.TriFunction, java.util.function.BiFunction, java.util.function.Function, net.minecraft.world.level.block.WeatheringCopper$WeatherState, java.lang.Object);
    private static net.minecraft.world.level.block.Block lambda$registerBlocks$2(java.util.function.BiFunction, net.minecraft.world.level.block.WeatheringCopper$WeatherState, net.minecraft.world.level.block.state.BlockBehaviour$Properties);
    private static java.lang.String lambda$prefixWithState$0(java.lang.String, java.lang.String);
    static {};
}
```
