---
type: "interface"
fqcn: "net.minecraft.server.packs.repository.Pack$Metadata"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.repository.Pack$Metadata

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `overlays()Ljava/util/List;` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (12, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.server.packs.repository.Pack$Metadata extends java.lang.Record {
    private final net.minecraft.network.chat.Component description;
    private final net.minecraft.server.packs.repository.PackCompatibility compatibility;
    private final net.minecraft.world.flag.FeatureFlagSet requestedFeatures;
    private final java.util.List<java.lang.String> overlays;
    public net.minecraft.server.packs.repository.Pack$Metadata(net.minecraft.network.chat.Component, net.minecraft.server.packs.repository.PackCompatibility, net.minecraft.world.flag.FeatureFlagSet, java.util.List<java.lang.String>);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.network.chat.Component description();
    public net.minecraft.server.packs.repository.PackCompatibility compatibility();
    public net.minecraft.world.flag.FeatureFlagSet requestedFeatures();
    public java.util.List<java.lang.String> overlays();
}
```
