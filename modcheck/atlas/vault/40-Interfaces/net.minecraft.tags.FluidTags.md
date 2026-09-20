---
type: "interface"
fqcn: "net.minecraft.tags.FluidTags"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.tags.FluidTags

System: [[20-Systems/net.minecraft.tags|net.minecraft.tags]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| reads | `LAVALnet/minecraft/tags/TagKey;` | `` | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| reads | `WATERLnet/minecraft/tags/TagKey;` | `` | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |

## Declared members (13, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.tags.FluidTags {
    public static final net.minecraft.tags.TagKey<net.minecraft.world.level.material.Fluid> WATER;
    public static final net.minecraft.tags.TagKey<net.minecraft.world.level.material.Fluid> LAVA;
    public static final net.minecraft.tags.TagKey<net.minecraft.world.level.material.Fluid> SUPPORTS_SUGAR_CANE_ADJACENTLY;
    public static final net.minecraft.tags.TagKey<net.minecraft.world.level.material.Fluid> SUPPORTS_LILY_PAD;
    public static final net.minecraft.tags.TagKey<net.minecraft.world.level.material.Fluid> SUPPORTS_FROGSPAWN;
    public static final net.minecraft.tags.TagKey<net.minecraft.world.level.material.Fluid> BUBBLE_COLUMN_CAN_OCCUPY;
    public static final net.minecraft.tags.TagKey<net.minecraft.world.level.material.Fluid> AXOLOTL_TRIES_TO_FIND;
    public static final net.minecraft.tags.TagKey<net.minecraft.world.level.material.Fluid> DOLPHIN_TRIES_TO_FIND;
    public static final net.minecraft.tags.TagKey<net.minecraft.world.level.material.Fluid> FROG_TRIES_TO_FIND_LAND_NEAR;
    public static final net.minecraft.tags.TagKey<net.minecraft.world.level.material.Fluid> ENTITY_FLOATABLE;
    private net.minecraft.tags.FluidTags();
    private static net.minecraft.tags.TagKey<net.minecraft.world.level.material.Fluid> create(java.lang.String);
    static {};
}
```
