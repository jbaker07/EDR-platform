---
type: "interface"
fqcn: "net.minecraft.server.RegistryLayer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.RegistryLayer

System: [[20-Systems/net.minecraft.server|net.minecraft.server]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| reads | `RELOADABLELnet/minecraft/server/RegistryLayer;` | `` | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| reads | `WORLDLnet/minecraft/server/RegistryLayer;` | `` | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |

## Declared members (13, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.server.RegistryLayer extends java.lang.Enum<net.minecraft.server.RegistryLayer> {
    public static final net.minecraft.server.RegistryLayer STATIC;
    public static final net.minecraft.server.RegistryLayer WORLD;
    public static final net.minecraft.server.RegistryLayer DIMENSIONS;
    public static final net.minecraft.server.RegistryLayer RELOADABLE;
    private static final java.util.List<net.minecraft.server.RegistryLayer> VALUES;
    private static final net.minecraft.core.RegistryAccess$Frozen STATIC_ACCESS;
    private static final net.minecraft.server.RegistryLayer[] $VALUES;
    public static net.minecraft.server.RegistryLayer[] values();
    public static net.minecraft.server.RegistryLayer valueOf(java.lang.String);
    private net.minecraft.server.RegistryLayer();
    public static net.minecraft.core.LayeredRegistryAccess<net.minecraft.server.RegistryLayer> createRegistryAccess();
    private static net.minecraft.server.RegistryLayer[] $values();
    static {};
}
```
