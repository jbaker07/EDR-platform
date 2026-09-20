---
type: "interface"
fqcn: "net.minecraft.server.packs.resources.ResourceMetadata"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.resources.ResourceMetadata

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `fromJsonStream(Ljava/io/InputStream;)Lnet/minecraft/server/packs/resources` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getSection(Lnet/minecraft/server/packs/metadata/MetadataSectionType;)L` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (10, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.server.packs.resources.ResourceMetadata {
    public static final net.minecraft.server.packs.resources.ResourceMetadata EMPTY;
    public static final net.minecraft.server.packs.resources.IoSupplier<net.minecraft.server.packs.resources.ResourceMetadata> EMPTY_SUPPLIER;
    public static net.minecraft.server.packs.resources.ResourceMetadata fromJsonStream(java.io.InputStream) throws java.io.IOException;
    public abstract <T> java.util.Optional<T> getSection(net.minecraft.server.packs.metadata.MetadataSectionType<T>);
    public default <T> java.util.Optional<net.minecraft.server.packs.metadata.MetadataSectionType$WithValue<T>> getTypedSection(net.minecraft.server.packs.metadata.MetadataSectionType<T>);
    public static <T> net.minecraft.server.packs.resources.ResourceMetadata of(net.minecraft.server.packs.metadata.MetadataSectionType<T>, T);
    public static <T1, T2> net.minecraft.server.packs.resources.ResourceMetadata of(net.minecraft.server.packs.metadata.MetadataSectionType<T1>, T1, net.minecraft.server.packs.metadata.MetadataSectionType<T2>, T2);
    public default java.util.List<net.minecraft.server.packs.metadata.MetadataSectionType$WithValue<?>> getTypedSections(java.util.Collection<net.minecraft.server.packs.metadata.MetadataSectionType<?>>);
    private static net.minecraft.server.packs.resources.ResourceMetadata lambda$static$0() throws java.io.IOException;
    static {};
}
```
