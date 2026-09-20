---
type: "interface"
fqcn: "net.minecraft.world.level.block.WeatheringCopperCollection$ByState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.WeatheringCopperCollection$ByState

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `progressMapping(Ljava/util/function/BiConsumer;)V` | `` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |

## Declared members (17, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.level.block.WeatheringCopperCollection$ByState<T> extends java.lang.Record {
    private final T unaffected;
    private final T exposed;
    private final T weathered;
    private final T oxidized;
    public net.minecraft.world.level.block.WeatheringCopperCollection$ByState(T, T, T, T);
    public static <T> net.minecraft.world.level.block.WeatheringCopperCollection$ByState<T> create(T);
    public <U> net.minecraft.world.level.block.WeatheringCopperCollection$ByState<U> map(java.util.function.Function<T, U>);
    public T pick(net.minecraft.world.level.block.WeatheringCopper$WeatherState);
    public void forEach(java.util.function.Consumer<T>);
    public void progressMapping(java.util.function.BiConsumer<T, T>);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public T unaffected();
    public T exposed();
    public T weathered();
    public T oxidized();
}
```
