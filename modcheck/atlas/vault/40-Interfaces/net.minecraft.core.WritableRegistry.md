---
type: "interface"
fqcn: "net.minecraft.core.WritableRegistry"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.WritableRegistry

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `bindTags(Ljava/util/Map;)V` | `` | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `key()Lnet/minecraft/resources/ResourceKey;` | `` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| calls | `key()Lnet/minecraft/resources/ResourceKey;` | `` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (4, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.core.WritableRegistry<T> extends net.minecraft.core.Registry<T> {
    public abstract net.minecraft.core.Holder$Reference<T> register(net.minecraft.resources.ResourceKey<T>, T, net.minecraft.core.RegistrationInfo);
    public abstract void bindTags(java.util.Map<net.minecraft.tags.TagKey<T>, java.util.List<net.minecraft.core.Holder<T>>>);
    public abstract boolean isEmpty();
    public abstract net.minecraft.core.HolderGetter<T> createRegistrationLookup();
}
```
