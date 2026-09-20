---
type: "interface"
fqcn: "net.minecraft.server.packs.PackResources"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.PackResources

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `close()V` | `` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `close()V` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `close()V` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `location()Lnet/minecraft/server/packs/PackLocationInfo;` | `` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (7, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.server.packs.PackResources extends net.minecraft.server.packs.PackMetadataResources {
    public static final java.lang.String METADATA_EXTENSION;
    public static final java.lang.String PACK_META;
    public abstract net.minecraft.server.packs.resources.IoSupplier<java.io.InputStream> getResource(net.minecraft.server.packs.PackType, net.minecraft.resources.Identifier);
    public abstract void listResources(net.minecraft.server.packs.PackType, java.lang.String, java.lang.String, net.minecraft.server.packs.PackResources$ResourceOutput);
    public abstract java.util.Set<java.lang.String> getNamespaces(net.minecraft.server.packs.PackType);
    public default java.lang.String packId();
    public default java.util.Optional<net.minecraft.server.packs.repository.KnownPack> knownPackInfo();
}
```
