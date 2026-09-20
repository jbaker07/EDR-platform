---
type: "interface"
fqcn: "net.minecraft.tags.TagLoader$EntryWithSource"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.tags.TagLoader$EntryWithSource

System: [[20-Systems/net.minecraft.tags|net.minecraft.tags]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/tags/TagEntry;Ljava/lang/String;)V` | `` | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `"<init>"(Lnet/minecraft/tags/TagEntry;Ljava/lang/String;)V` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `entry()Lnet/minecraft/tags/TagEntry;` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `source()Ljava/lang/String;` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |

## Declared members (8, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.tags.TagLoader$EntryWithSource extends java.lang.Record {
    private final net.minecraft.tags.TagEntry entry;
    private final java.lang.String source;
    public net.minecraft.tags.TagLoader$EntryWithSource(net.minecraft.tags.TagEntry, java.lang.String);
    public java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.tags.TagEntry entry();
    public java.lang.String source();
}
```
