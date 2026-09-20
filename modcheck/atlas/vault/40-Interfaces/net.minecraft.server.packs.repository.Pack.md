---
type: "interface"
fqcn: "net.minecraft.server.packs.repository.Pack"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.repository.Pack

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getId()Ljava/lang/String;` | `` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getId()Ljava/lang/String;` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getId()Ljava/lang/String;` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getPackSource()Lnet/minecraft/server/packs/repository/PackSource;` | `` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getPackSource()Lnet/minecraft/server/packs/repository/PackSource;` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `open()Ljava/util/stream/Stream;` | `` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `open()Ljava/util/stream/Stream;` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `open()Ljava/util/stream/Stream;` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `readMetaAndCreate(Lnet/minecraft/server/packs/PackLocationInfo;Lnet/minecraft` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `readMetaAndCreate(Lnet/minecraft/server/packs/PackLocationInfo;Lnet/minecraft` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| injects_into | `readPackMetadata` | `@ModifyVariable at STORE` | both | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |

## Declared members (25, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.packs.repository.Pack {
    private static final org.slf4j.Logger LOGGER;
    private final net.minecraft.server.packs.PackLocationInfo location;
    private final net.minecraft.server.packs.repository.Pack$ResourcesSupplier resources;
    private final net.minecraft.server.packs.repository.Pack$Metadata metadata;
    private final net.minecraft.server.packs.PackSelectionConfig selectionConfig;
    public static net.minecraft.server.packs.repository.Pack readMetaAndCreate(net.minecraft.server.packs.PackLocationInfo, net.minecraft.server.packs.repository.Pack$ResourcesSupplier, net.minecraft.server.packs.PackType, net.minecraft.server.packs.PackSelectionConfig);
    public net.minecraft.server.packs.repository.Pack(net.minecraft.server.packs.PackLocationInfo, net.minecraft.server.packs.repository.Pack$ResourcesSupplier, net.minecraft.server.packs.repository.Pack$Metadata, net.minecraft.server.packs.PackSelectionConfig);
    public static net.minecraft.server.packs.repository.Pack$Metadata readPackMetadata(net.minecraft.server.packs.PackLocationInfo, net.minecraft.server.packs.repository.Pack$ResourcesSupplier, net.minecraft.server.packs.metadata.pack.PackFormat, net.minecraft.server.packs.PackType);
    public net.minecraft.server.packs.PackLocationInfo location();
    public net.minecraft.network.chat.Component getTitle();
    public net.minecraft.network.chat.Component getDescription();
    public net.minecraft.network.chat.Component getChatLink(boolean);
    public net.minecraft.server.packs.repository.PackCompatibility getCompatibility();
    public net.minecraft.world.flag.FeatureFlagSet getRequestedFeatures();
    public net.minecraft.server.packs.PackMetadataResources openMetadata();
    public java.util.stream.Stream<net.minecraft.server.packs.PackResources> open();
    public java.lang.String getId();
    public net.minecraft.server.packs.PackSelectionConfig selectionConfig();
    public boolean isRequired();
    public boolean isFixedPosition();
    public net.minecraft.server.packs.repository.Pack$Position getDefaultPosition();
    public net.minecraft.server.packs.repository.PackSource getPackSource();
    public boolean equals(java.lang.Object);
    public int hashCode();
    static {};
}
```
