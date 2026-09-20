---
type: "interface"
fqcn: "net.minecraft.server.packs.PackSelectionConfig"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.PackSelectionConfig

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(ZLnet/minecraft/server/packs/repository/Pack$Position;Z)V` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `"<init>"(ZLnet/minecraft/server/packs/repository/Pack$Position;Z)V` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (10, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.server.packs.PackSelectionConfig extends java.lang.Record {
    private final boolean required;
    private final net.minecraft.server.packs.repository.Pack$Position defaultPosition;
    private final boolean fixedPosition;
    public net.minecraft.server.packs.PackSelectionConfig(boolean, net.minecraft.server.packs.repository.Pack$Position, boolean);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public boolean required();
    public net.minecraft.server.packs.repository.Pack$Position defaultPosition();
    public boolean fixedPosition();
}
```
