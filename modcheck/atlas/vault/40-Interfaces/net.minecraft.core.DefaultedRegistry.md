---
type: "interface"
fqcn: "net.minecraft.core.DefaultedRegistry"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.DefaultedRegistry

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getKey(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | `` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `getKey(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | `` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `getKey(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | `` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `getKey(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | `` | client | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getKey(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | `` | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getKey(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | `` | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getKey(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | `` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getKey(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getResourceKey(Ljava/lang/Object;)Ljava/util/Optional;` | `` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `getTagOrEmpty(Lnet/minecraft/tags/TagKey;)Ljava/lang/Iterable;` | `` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `holderByNameCodec()Lcom/mojang/serialization/Codec;` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `stream()Ljava/util/stream/Stream;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (4, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.core.DefaultedRegistry<T> extends net.minecraft.core.Registry<T> {
    public abstract net.minecraft.resources.Identifier getKey(T);
    public abstract T getValue(net.minecraft.resources.Identifier);
    public abstract T byId(int);
    public abstract net.minecraft.resources.Identifier getDefaultKey();
}
```
