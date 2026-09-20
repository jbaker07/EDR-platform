---
type: "interface"
fqcn: "net.minecraft.data.AtlasIds"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.AtlasIds

System: [[20-Systems/net.minecraft.data|net.minecraft.data]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| reads | `BLOCKSLnet/minecraft/resources/Identifier;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `ITEMSLnet/minecraft/resources/Identifier;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (14, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.data.AtlasIds {
    public static final net.minecraft.resources.Identifier BANNER_PATTERNS;
    public static final net.minecraft.resources.Identifier BLOCKS;
    public static final net.minecraft.resources.Identifier ITEMS;
    public static final net.minecraft.resources.Identifier CHESTS;
    public static final net.minecraft.resources.Identifier DECORATED_POT;
    public static final net.minecraft.resources.Identifier GUI;
    public static final net.minecraft.resources.Identifier MAP_DECORATIONS;
    public static final net.minecraft.resources.Identifier PAINTINGS;
    public static final net.minecraft.resources.Identifier PARTICLES;
    public static final net.minecraft.resources.Identifier SHIELD_PATTERNS;
    public static final net.minecraft.resources.Identifier SHULKER_BOXES;
    public static final net.minecraft.resources.Identifier CELESTIALS;
    public net.minecraft.data.AtlasIds();
    static {};
}
```
