---
type: "interface"
fqcn: "net.minecraft.client.resources.model.sprite.AtlasManager$AtlasConfig"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.resources.model.sprite.AtlasManager$AtlasConfig

System: [[20-Systems/net.minecraft.client.resources|net.minecraft.client.resources]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `definitionLocation()Lnet/minecraft/resources/Identifier;` | `` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `textureId()Lnet/minecraft/resources/Identifier;` | `` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (13, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.client.resources.model.sprite.AtlasManager$AtlasConfig extends java.lang.Record {
    private final net.minecraft.resources.Identifier textureId;
    private final net.minecraft.resources.Identifier definitionLocation;
    private final boolean createMipmaps;
    private final java.util.Set<net.minecraft.server.packs.metadata.MetadataSectionType<?>> additionalMetadata;
    public net.minecraft.client.resources.model.sprite.AtlasManager$AtlasConfig(net.minecraft.resources.Identifier, net.minecraft.resources.Identifier, boolean);
    public net.minecraft.client.resources.model.sprite.AtlasManager$AtlasConfig(net.minecraft.resources.Identifier, net.minecraft.resources.Identifier, boolean, java.util.Set<net.minecraft.server.packs.metadata.MetadataSectionType<?>>);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.resources.Identifier textureId();
    public net.minecraft.resources.Identifier definitionLocation();
    public boolean createMipmaps();
    public java.util.Set<net.minecraft.server.packs.metadata.MetadataSectionType<?>> additionalMetadata();
}
```
