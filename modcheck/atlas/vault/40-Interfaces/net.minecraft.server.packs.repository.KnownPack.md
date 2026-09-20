---
type: "interface"
fqcn: "net.minecraft.server.packs.repository.KnownPack"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.repository.KnownPack

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (15, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.server.packs.repository.KnownPack extends java.lang.Record {
    private final java.lang.String namespace;
    private final java.lang.String id;
    private final java.lang.String version;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, net.minecraft.server.packs.repository.KnownPack> STREAM_CODEC;
    public static final java.lang.String VANILLA_NAMESPACE;
    public net.minecraft.server.packs.repository.KnownPack(java.lang.String, java.lang.String, java.lang.String);
    public static net.minecraft.server.packs.repository.KnownPack vanilla(java.lang.String);
    public boolean isVanilla();
    public java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public java.lang.String namespace();
    public java.lang.String id();
    public java.lang.String version();
    static {};
}
```
