---
type: "interface"
fqcn: "net.minecraft.tags.TagFile"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.tags.TagFile

System: [[20-Systems/net.minecraft.tags|net.minecraft.tags]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `entries()Ljava/util/List;` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `remove()Ljava/util/List;` | `` | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `remove()Ljava/util/List;` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `replace()Z` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |

## Declared members (11, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.tags.TagFile extends java.lang.Record {
    private final java.util.List<net.minecraft.tags.TagEntry> entries;
    private final boolean replace;
    public static final com.mojang.serialization.Codec<net.minecraft.tags.TagFile> CODEC;
    public net.minecraft.tags.TagFile(java.util.List<net.minecraft.tags.TagEntry>, boolean);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public java.util.List<net.minecraft.tags.TagEntry> entries();
    public boolean replace();
    private static com.mojang.datafixers.kinds.App lambda$static$0(com.mojang.serialization.codecs.RecordCodecBuilder$Instance);
    static {};
}
```
