---
type: "interface"
fqcn: "net.minecraft.resources.RegistryOps"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.resources.RegistryOps

System: [[20-Systems/net.minecraft.resources|net.minecraft.resources]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `retrieveGetter(Lnet/minecraft/resources/ResourceKey;)Lcom/mojang/serializa` | `` | both | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `lookupProviderLnet/minecraft/resources/RegistryOps$RegistryInfoLookup;` | `` | both | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |

## Declared members (23, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.resources.RegistryOps<T> extends net.minecraft.resources.DelegatingOps<T> {
    private final net.minecraft.resources.RegistryOps$RegistryInfoLookup lookupProvider;
    public static <T> net.minecraft.resources.RegistryOps<T> create(com.mojang.serialization.DynamicOps<T>, net.minecraft.core.HolderLookup$Provider);
    public static <T> net.minecraft.resources.RegistryOps<T> create(com.mojang.serialization.DynamicOps<T>, net.minecraft.resources.RegistryOps$RegistryInfoLookup);
    public static <T> com.mojang.serialization.Dynamic<T> injectRegistryContext(com.mojang.serialization.Dynamic<T>, net.minecraft.core.HolderLookup$Provider);
    private net.minecraft.resources.RegistryOps(com.mojang.serialization.DynamicOps<T>, net.minecraft.resources.RegistryOps$RegistryInfoLookup);
    public <U> net.minecraft.resources.RegistryOps<U> withParent(com.mojang.serialization.DynamicOps<U>);
    public <E> java.util.Optional<net.minecraft.core.HolderGetter<E>> getter(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<? extends E>>);
    public boolean equals(java.lang.Object);
    public int hashCode();
    public static <E, O> com.mojang.serialization.codecs.RecordCodecBuilder<O, net.minecraft.core.HolderGetter<E>> retrieveGetter(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<? extends E>>);
    public static <E, O> com.mojang.serialization.codecs.RecordCodecBuilder<O, net.minecraft.core.Holder$Reference<E>> retrieveElement(net.minecraft.resources.ResourceKey<E>);
    private static net.minecraft.core.Holder$Reference lambda$retrieveElement$5(java.lang.Object);
    private static com.mojang.serialization.DataResult lambda$retrieveElement$0(net.minecraft.resources.ResourceKey, net.minecraft.resources.ResourceKey, com.mojang.serialization.DynamicOps);
    private static java.lang.String lambda$retrieveElement$4();
    private static com.mojang.serialization.DataResult lambda$retrieveElement$2(net.minecraft.resources.ResourceKey);
    private static java.lang.String lambda$retrieveElement$3(net.minecraft.resources.ResourceKey);
    private static java.util.Optional lambda$retrieveElement$1(net.minecraft.resources.ResourceKey, net.minecraft.core.HolderGetter);
    private static net.minecraft.core.HolderGetter lambda$retrieveGetter$5(java.lang.Object);
    private static com.mojang.serialization.DataResult lambda$retrieveGetter$0(net.minecraft.resources.ResourceKey, com.mojang.serialization.DynamicOps);
    private static java.lang.String lambda$retrieveGetter$4();
    private static com.mojang.serialization.DataResult lambda$retrieveGetter$2(net.minecraft.resources.ResourceKey);
    private static java.lang.String lambda$retrieveGetter$3(net.minecraft.resources.ResourceKey);
    private static com.mojang.serialization.DataResult lambda$retrieveGetter$1(net.minecraft.core.HolderGetter);
}
```
