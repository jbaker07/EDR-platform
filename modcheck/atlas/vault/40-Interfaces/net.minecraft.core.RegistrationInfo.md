---
type: "interface"
fqcn: "net.minecraft.core.RegistrationInfo"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.RegistrationInfo

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Ljava/util/Optional;Lcom/mojang/serialization/Lifecycle;)V` | `` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `"<init>"(Ljava/util/Optional;Lcom/mojang/serialization/Lifecycle;)V` | `` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `"<init>"(Ljava/util/Optional;Lcom/mojang/serialization/Lifecycle;)V` | `` | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `knownPackInfo()Ljava/util/Optional;` | `` | both | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `lifecycle()Lcom/mojang/serialization/Lifecycle;` | `` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `lifecycle()Lcom/mojang/serialization/Lifecycle;` | `` | both | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `lifecycle()Lcom/mojang/serialization/Lifecycle;` | `` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `lifecycle()Lcom/mojang/serialization/Lifecycle;` | `` | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (10, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.core.RegistrationInfo extends java.lang.Record {
    private final java.util.Optional<net.minecraft.server.packs.repository.KnownPack> knownPackInfo;
    private final com.mojang.serialization.Lifecycle lifecycle;
    public static final net.minecraft.core.RegistrationInfo BUILT_IN;
    public net.minecraft.core.RegistrationInfo(java.util.Optional<net.minecraft.server.packs.repository.KnownPack>, com.mojang.serialization.Lifecycle);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public java.util.Optional<net.minecraft.server.packs.repository.KnownPack> knownPackInfo();
    public com.mojang.serialization.Lifecycle lifecycle();
    static {};
}
```
