---
type: "interface"
fqcn: "net.minecraft.core.RegistrySetBuilder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.RegistrySetBuilder

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"()V` | `` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `add(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/core/re` | `` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `build(Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/c` | `` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (14, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.core.RegistrySetBuilder {
    private final java.util.List<net.minecraft.core.RegistrySetBuilder$RegistryStub> entries;
    public net.minecraft.core.RegistrySetBuilder();
    private static net.minecraft.core.RegistrySetBuilder$RegistryStub placeholderStub(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<?>>);
    public <T> net.minecraft.core.RegistrySetBuilder add(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>>, net.minecraft.core.registries.SingleRegistryBootstrap<T>);
    public net.minecraft.core.RegistrySetBuilder add(net.minecraft.core.registries.MultiRegistryBootstrap);
    private static net.minecraft.core.HolderLookup$Provider buildProviderWithContext(net.minecraft.core.HolderLookup$Provider, java.util.stream.Stream<? extends net.minecraft.core.HolderLookup$RegistryLookup<?>>);
    public net.minecraft.core.HolderLookup$Provider build(net.minecraft.core.HolderLookup$Provider);
    private static java.util.Set<net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<?>>> findRegistriesMissingFromPatch(net.minecraft.core.HolderLookup$Provider, net.minecraft.core.HolderLookup$Provider, java.util.List<net.minecraft.core.RegistrySetBuilder$RegistryStub>);
    public net.minecraft.core.RegistrySetBuilder$PatchedRegistries buildPatch(net.minecraft.core.HolderLookup$Provider, net.minecraft.core.HolderLookup$Provider, net.minecraft.core.Cloner$Factory);
    private static <T> net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>> eyerollCast(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<? extends T>>);
    private static java.util.stream.Stream<net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<?>>> newRegistryKeys(java.util.stream.Stream<net.minecraft.core.RegistrySetBuilder$RegistryStub>);
    private static boolean lambda$findRegistriesMissingFromPatch$0(java.util.Set, net.minecraft.resources.ResourceKey);
    private static void lambda$buildProviderWithContext$1(java.util.Map, net.minecraft.core.HolderLookup$RegistryLookup);
    private static void lambda$buildProviderWithContext$0(java.util.Map, net.minecraft.core.HolderLookup$RegistryLookup);
}
```
