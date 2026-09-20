---
type: "interface"
fqcn: "net.minecraft.server.packs.resources.Resource"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.resources.Resource

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getFabricPackSource()Lnet/minecraft/server/packs/repository/PackSource;` | `` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `getFabricPackSource()Lnet/minecraft/server/packs/repository/PackSource;` | `` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `getFabricPackSource()Lnet/minecraft/server/packs/repository/PackSource;` | `` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `openAsReader()Ljava/io/BufferedReader;` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `source()Lnet/minecraft/server/packs/PackResources;` | `` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (13, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.packs.resources.Resource {
    private final net.minecraft.server.packs.PackResources source;
    private final net.minecraft.server.packs.resources.IoSupplier<java.io.InputStream> streamSupplier;
    private final net.minecraft.server.packs.resources.IoSupplier<net.minecraft.server.packs.resources.ResourceMetadata> metadataSupplier;
    private net.minecraft.server.packs.resources.ResourceMetadata cachedMetadata;
    public net.minecraft.server.packs.resources.Resource(net.minecraft.server.packs.PackResources, net.minecraft.server.packs.resources.IoSupplier<java.io.InputStream>, net.minecraft.server.packs.resources.IoSupplier<net.minecraft.server.packs.resources.ResourceMetadata>);
    public net.minecraft.server.packs.resources.Resource(net.minecraft.server.packs.PackResources, net.minecraft.server.packs.resources.IoSupplier<java.io.InputStream>);
    public net.minecraft.server.packs.PackResources source();
    public java.lang.String sourcePackId();
    public java.util.Optional<net.minecraft.server.packs.repository.KnownPack> knownPackInfo();
    public java.io.InputStream open() throws java.io.IOException;
    public java.io.BufferedReader openAsReader() throws java.io.IOException;
    public java.lang.String readAllAsString() throws java.io.IOException;
    public net.minecraft.server.packs.resources.ResourceMetadata metadata() throws java.io.IOException;
}
```
