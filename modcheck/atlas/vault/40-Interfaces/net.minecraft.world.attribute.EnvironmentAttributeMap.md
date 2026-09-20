---
type: "interface"
fqcn: "net.minecraft.world.attribute.EnvironmentAttributeMap"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.attribute.EnvironmentAttributeMap

System: [[20-Systems/net.minecraft.world.attribute|net.minecraft.world.attribute]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `builder()Lnet/minecraft/world/attribute/EnvironmentAttributeMap$Bui` | `` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `equals(Ljava/lang/Object;)Z` | `` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |

## Declared members (21, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.attribute.EnvironmentAttributeMap {
    public static final net.minecraft.world.attribute.EnvironmentAttributeMap EMPTY;
    public static final com.mojang.serialization.Codec<net.minecraft.world.attribute.EnvironmentAttributeMap> CODEC;
    public static final com.mojang.serialization.Codec<net.minecraft.world.attribute.EnvironmentAttributeMap> NETWORK_CODEC;
    public static final com.mojang.serialization.Codec<net.minecraft.world.attribute.EnvironmentAttributeMap> CODEC_ONLY_POSITIONAL;
    private final java.util.Map<net.minecraft.world.attribute.EnvironmentAttribute<?>, net.minecraft.world.attribute.EnvironmentAttributeMap$Entry<?, ?>> entries;
    private static net.minecraft.world.attribute.EnvironmentAttributeMap filterSyncable(net.minecraft.world.attribute.EnvironmentAttributeMap);
    private net.minecraft.world.attribute.EnvironmentAttributeMap(java.util.Map<net.minecraft.world.attribute.EnvironmentAttribute<?>, net.minecraft.world.attribute.EnvironmentAttributeMap$Entry<?, ?>>);
    public static net.minecraft.world.attribute.EnvironmentAttributeMap$Builder builder();
    public <Value> net.minecraft.world.attribute.EnvironmentAttributeMap$Entry<Value, ?> get(net.minecraft.world.attribute.EnvironmentAttribute<Value>);
    public <Value> Value applyModifier(net.minecraft.world.attribute.EnvironmentAttribute<Value>, Value);
    public boolean contains(net.minecraft.world.attribute.EnvironmentAttribute<?>);
    public java.util.Set<net.minecraft.world.attribute.EnvironmentAttribute<?>> keySet();
    public boolean equals(java.lang.Object);
    public int hashCode();
    public java.lang.String toString();
    private static com.mojang.serialization.DataResult lambda$static$2(net.minecraft.world.attribute.EnvironmentAttributeMap);
    private static java.lang.String lambda$static$4(java.util.List);
    private static boolean lambda$static$3(net.minecraft.world.attribute.EnvironmentAttribute);
    private static com.mojang.serialization.Codec lambda$static$0();
    private static java.util.Map lambda$static$1(net.minecraft.world.attribute.EnvironmentAttributeMap);
    static {};
}
```
