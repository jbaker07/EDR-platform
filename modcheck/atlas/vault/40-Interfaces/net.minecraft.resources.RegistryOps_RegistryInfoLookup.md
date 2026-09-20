---
type: "interface"
fqcn: "net.minecraft.resources.RegistryOps$RegistryInfoLookup"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.resources.RegistryOps$RegistryInfoLookup

System: [[20-Systems/net.minecraft.resources|net.minecraft.resources]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `lookup(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | `` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `lookup(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | `` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `lookup(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | `` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |

## Declared members (1, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.resources.RegistryOps$RegistryInfoLookup {
    public abstract <T> java.util.Optional<net.minecraft.core.HolderGetter<T>> lookup(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<? extends T>>);
}
```
