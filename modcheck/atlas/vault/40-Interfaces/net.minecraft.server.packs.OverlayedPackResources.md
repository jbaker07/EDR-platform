---
type: "interface"
fqcn: "net.minecraft.server.packs.OverlayedPackResources"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.OverlayedPackResources

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/server/packs/PackResources;Ljava/util/List;)` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (10, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.packs.OverlayedPackResources implements net.minecraft.server.packs.PackResources {
    private final net.minecraft.server.packs.PackMetadataResources primaryPackMetadataResources;
    private final java.util.List<net.minecraft.server.packs.PackResources> packResourcesStack;
    public net.minecraft.server.packs.OverlayedPackResources(net.minecraft.server.packs.PackResources, java.util.List<net.minecraft.server.packs.PackResources>);
    public net.minecraft.server.packs.resources.IoSupplier<java.io.InputStream> getRootResource(java.lang.String...);
    public net.minecraft.server.packs.resources.IoSupplier<java.io.InputStream> getResource(net.minecraft.server.packs.PackType, net.minecraft.resources.Identifier);
    public void listResources(net.minecraft.server.packs.PackType, java.lang.String, java.lang.String, net.minecraft.server.packs.PackResources$ResourceOutput);
    public java.util.Set<java.lang.String> getNamespaces(net.minecraft.server.packs.PackType);
    public <T> T getMetadataSection(net.minecraft.server.packs.metadata.MetadataSectionType<T>) throws java.io.IOException;
    public net.minecraft.server.packs.PackLocationInfo location();
    public void close();
}
```
