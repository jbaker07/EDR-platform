---
type: "interface"
fqcn: "net.minecraft.server.packs.PackMetadataResources"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.PackMetadataResources

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getMetadataSection(Lnet/minecraft/server/packs/metadata/MetadataSectionType;)L` | `` | both | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |

## Declared members (4, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.server.packs.PackMetadataResources extends java.lang.AutoCloseable {
    public abstract net.minecraft.server.packs.PackLocationInfo location();
    public abstract net.minecraft.server.packs.resources.IoSupplier<java.io.InputStream> getRootResource(java.lang.String...);
    public abstract <T> T getMetadataSection(net.minecraft.server.packs.metadata.MetadataSectionType<T>) throws java.io.IOException;
    public abstract void close();
}
```
