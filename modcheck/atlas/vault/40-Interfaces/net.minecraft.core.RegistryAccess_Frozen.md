---
type: "interface"
fqcn: "net.minecraft.core.RegistryAccess$Frozen"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.RegistryAccess$Frozen

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `lookupOrThrow(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/R` | `` | both | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `lookupOrThrow(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/R` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `lookupOrThrow(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/R` | `` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `lookupOrThrow(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/R` | `` | both | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `registries()Ljava/util/stream/Stream;` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |

## Declared members (0, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.core.RegistryAccess$Frozen extends net.minecraft.core.RegistryAccess {

}
```
