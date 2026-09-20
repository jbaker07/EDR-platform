---
type: "interface"
fqcn: "net.minecraft.server.packs.metadata.pack.PackMetadataSection"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.metadata.pack.PackMetadataSection

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/network/chat/Component;Lnet/minecraft/util/I` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `codecForPackType(Lnet/minecraft/server/packs/PackType;)Lcom/mojang/serializa` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (18, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.server.packs.metadata.pack.PackMetadataSection extends java.lang.Record {
    private final net.minecraft.network.chat.Component description;
    private final net.minecraft.util.InclusiveRange<net.minecraft.server.packs.metadata.pack.PackFormat> supportedFormats;
    private static final com.mojang.serialization.Codec<net.minecraft.server.packs.metadata.pack.PackMetadataSection> FALLBACK_CODEC;
    public static final net.minecraft.server.packs.metadata.MetadataSectionType<net.minecraft.server.packs.metadata.pack.PackMetadataSection> CLIENT_TYPE;
    public static final net.minecraft.server.packs.metadata.MetadataSectionType<net.minecraft.server.packs.metadata.pack.PackMetadataSection> SERVER_TYPE;
    public static final net.minecraft.server.packs.metadata.MetadataSectionType<net.minecraft.server.packs.metadata.pack.PackMetadataSection> FALLBACK_TYPE;
    public net.minecraft.server.packs.metadata.pack.PackMetadataSection(net.minecraft.network.chat.Component, net.minecraft.util.InclusiveRange<net.minecraft.server.packs.metadata.pack.PackFormat>);
    private static com.mojang.serialization.Codec<net.minecraft.server.packs.metadata.pack.PackMetadataSection> codecForPackType(net.minecraft.server.packs.PackType);
    public static net.minecraft.server.packs.metadata.MetadataSectionType<net.minecraft.server.packs.metadata.pack.PackMetadataSection> forPackType(net.minecraft.server.packs.PackType);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.network.chat.Component description();
    public net.minecraft.util.InclusiveRange<net.minecraft.server.packs.metadata.pack.PackFormat> supportedFormats();
    private static com.mojang.datafixers.kinds.App lambda$codecForPackType$0(net.minecraft.server.packs.PackType, com.mojang.serialization.codecs.RecordCodecBuilder$Instance);
    private static com.mojang.datafixers.kinds.App lambda$static$0(com.mojang.serialization.codecs.RecordCodecBuilder$Instance);
    private static net.minecraft.server.packs.metadata.pack.PackMetadataSection lambda$static$1(net.minecraft.network.chat.Component);
    static {};
}
```
