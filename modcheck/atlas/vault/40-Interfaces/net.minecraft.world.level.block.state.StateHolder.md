---
type: "interface"
fqcn: "net.minecraft.world.level.block.state.StateHolder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.state.StateHolder

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Ljava/lang/Object;[Lnet/minecraft/world/level/block/state/p` | `` | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |

## Declared members (34, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public abstract class net.minecraft.world.level.block.state.StateHolder<O, S> {
    private static final int VALUE_NOT_FOUND;
    public static final java.lang.String ID_TAG;
    public static final java.lang.String PROPERTIES_TAG;
    protected final O owner;
    private final net.minecraft.world.level.block.state.properties.Property<?>[] propertyKeys;
    private final java.lang.Comparable<?>[] propertyValues;
    private S[][] neighbors;
    static final boolean $assertionsDisabled;
    protected net.minecraft.world.level.block.state.StateHolder(O, net.minecraft.world.level.block.state.properties.Property<?>[], java.lang.Comparable<?>[]);
    public <T extends java.lang.Comparable<T>> S cycle(net.minecraft.world.level.block.state.properties.Property<T>);
    protected static <T> T findNextInCollection(java.util.List<T>, T);
    public java.lang.String toString();
    public final boolean equals(java.lang.Object);
    public int hashCode();
    public java.util.Collection<net.minecraft.world.level.block.state.properties.Property<?>> getProperties();
    private int valueIndex(net.minecraft.world.level.block.state.properties.Property<?>);
    public boolean hasProperty(net.minecraft.world.level.block.state.properties.Property<?>);
    private <T extends java.lang.Comparable<T>> T getNullableValue(net.minecraft.world.level.block.state.properties.Property<T>);
    public <T extends java.lang.Comparable<T>> T getValue(net.minecraft.world.level.block.state.properties.Property<T>);
    public <T extends java.lang.Comparable<T>> java.util.Optional<T> getOptionalValue(net.minecraft.world.level.block.state.properties.Property<T>);
    public <T extends java.lang.Comparable<T>> T getValueOrElse(net.minecraft.world.level.block.state.properties.Property<T>, T);
    public <T extends java.lang.Comparable<T>, V extends T> S setValue(net.minecraft.world.level.block.state.properties.Property<T>, V);
    public <T extends java.lang.Comparable<T>, V extends T> S trySetValue(net.minecraft.world.level.block.state.properties.Property<T>, V);
    private <T extends java.lang.Comparable<T>, V extends T> S setValueInternal(net.minecraft.world.level.block.state.properties.Property<T>, int, V);
    void initializeNeighbors(S[][]);
    public boolean isSingletonState();
    public java.util.stream.Stream<net.minecraft.world.level.block.state.properties.Property$Value<?>> getValues();
    private static <T extends java.lang.Comparable<T>> net.minecraft.world.level.block.state.properties.Property$Value<T> createValue(net.minecraft.world.level.block.state.properties.Property<T>, java.lang.Comparable<?>);
    protected static <O, S extends net.minecraft.world.level.block.state.StateHolder<O, S>> com.mojang.serialization.Codec<S> codec(com.mojang.serialization.Codec<O>, java.util.function.Function<O, S>, java.util.function.Function<O, net.minecraft.world.level.block.state.StateDefinition<O, S>>);
    private static com.mojang.serialization.MapCodec lambda$codec$1(java.util.function.Function, java.util.function.Function, java.lang.Object);
    private static net.minecraft.world.level.block.state.StateHolder lambda$codec$2(net.minecraft.world.level.block.state.StateHolder, java.util.Optional);
    private static java.lang.Object lambda$codec$0(net.minecraft.world.level.block.state.StateHolder);
    private net.minecraft.world.level.block.state.properties.Property$Value lambda$getValues$0(int);
    static {};
}
```
