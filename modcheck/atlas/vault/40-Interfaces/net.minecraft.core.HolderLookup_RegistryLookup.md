---
type: "interface"
fqcn: "net.minecraft.core.HolderLookup$RegistryLookup"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.HolderLookup$RegistryLookup

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `key()Lnet/minecraft/resources/ResourceKey;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `listElementIds()Ljava/util/stream/Stream;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `listElements()Ljava/util/stream/Stream;` | `` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `listElements()Ljava/util/stream/Stream;` | `` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `listElements()Ljava/util/stream/Stream;` | `` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `listTags()Ljava/util/stream/Stream;` | `` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `listTags()Ljava/util/stream/Stream;` | `` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `registryLifecycle()Lcom/mojang/serialization/Lifecycle;` | `` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `registryLifecycle()Lcom/mojang/serialization/Lifecycle;` | `` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |

## Declared members (5, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.core.HolderLookup$RegistryLookup<T> extends net.minecraft.core.HolderLookup<T> {
    public abstract net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<? extends T>> key();
    public abstract com.mojang.serialization.Lifecycle registryLifecycle();
    public default net.minecraft.core.HolderLookup$RegistryLookup<T> filterFeatures(net.minecraft.world.flag.FeatureFlagSet);
    public default net.minecraft.core.HolderLookup$RegistryLookup<T> filterElements(java.util.function.Predicate<T>);
    private static boolean lambda$filterFeatures$0(net.minecraft.world.flag.FeatureFlagSet, java.lang.Object);
}
```
