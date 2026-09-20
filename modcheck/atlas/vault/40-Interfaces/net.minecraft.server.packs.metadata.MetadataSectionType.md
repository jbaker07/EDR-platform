---
type: "interface"
fqcn: "net.minecraft.server.packs.metadata.MetadataSectionType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.metadata.MetadataSectionType

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Ljava/lang/String;Lcom/mojang/serialization/Codec;)V` | `` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |

## Declared members (9, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.server.packs.metadata.MetadataSectionType<T> extends java.lang.Record {
    private final java.lang.String name;
    private final com.mojang.serialization.Codec<T> codec;
    public net.minecraft.server.packs.metadata.MetadataSectionType(java.lang.String, com.mojang.serialization.Codec<T>);
    public net.minecraft.server.packs.metadata.MetadataSectionType$WithValue<T> withValue(T);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public java.lang.String name();
    public com.mojang.serialization.Codec<T> codec();
}
```
