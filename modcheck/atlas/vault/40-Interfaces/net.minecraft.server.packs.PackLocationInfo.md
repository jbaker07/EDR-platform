---
type: "interface"
fqcn: "net.minecraft.server.packs.PackLocationInfo"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.PackLocationInfo

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Ljava/lang/String;Lnet/minecraft/network/chat/Component;Lne` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `"<init>"(Ljava/lang/String;Lnet/minecraft/network/chat/Component;Lne` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `knownPackInfo()Ljava/util/Optional;` | `` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `source()Lnet/minecraft/server/packs/repository/PackSource;` | `` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (14, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.server.packs.PackLocationInfo extends java.lang.Record {
    private final java.lang.String id;
    private final net.minecraft.network.chat.Component title;
    private final net.minecraft.server.packs.repository.PackSource source;
    private final java.util.Optional<net.minecraft.server.packs.repository.KnownPack> knownPackInfo;
    public net.minecraft.server.packs.PackLocationInfo(java.lang.String, net.minecraft.network.chat.Component, net.minecraft.server.packs.repository.PackSource, java.util.Optional<net.minecraft.server.packs.repository.KnownPack>);
    public net.minecraft.network.chat.Component createChatLink(boolean, net.minecraft.network.chat.Component);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public java.lang.String id();
    public net.minecraft.network.chat.Component title();
    public net.minecraft.server.packs.repository.PackSource source();
    public java.util.Optional<net.minecraft.server.packs.repository.KnownPack> knownPackInfo();
    private net.minecraft.network.chat.Style lambda$createChatLink$0(boolean, net.minecraft.network.chat.Component, net.minecraft.network.chat.Style);
}
```
