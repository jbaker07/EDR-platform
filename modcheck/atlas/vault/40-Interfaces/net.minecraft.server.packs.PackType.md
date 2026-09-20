---
type: "interface"
fqcn: "net.minecraft.server.packs.PackType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.PackType

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getDirectory()Ljava/lang/String;` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getDirectory()Ljava/lang/String;` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `values()[Lnet/minecraft/server/packs/PackType;` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `SERVER_DATALnet/minecraft/server/packs/PackType;` | `` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `SERVER_DATALnet/minecraft/server/packs/PackType;` | `` | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (10, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.server.packs.PackType extends java.lang.Enum<net.minecraft.server.packs.PackType> {
    public static final net.minecraft.server.packs.PackType CLIENT_RESOURCES;
    public static final net.minecraft.server.packs.PackType SERVER_DATA;
    private final java.lang.String directory;
    private static final net.minecraft.server.packs.PackType[] $VALUES;
    public static net.minecraft.server.packs.PackType[] values();
    public static net.minecraft.server.packs.PackType valueOf(java.lang.String);
    private net.minecraft.server.packs.PackType(java.lang.String);
    public java.lang.String getDirectory();
    private static net.minecraft.server.packs.PackType[] $values();
    static {};
}
```
