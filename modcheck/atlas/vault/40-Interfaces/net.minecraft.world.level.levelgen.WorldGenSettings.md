---
type: "interface"
fqcn: "net.minecraft.world.level.levelgen.WorldGenSettings"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.levelgen.WorldGenSettings

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| reads | `TYPELnet/minecraft/world/level/saveddata/SavedDataType;` | `` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (13, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.level.levelgen.WorldGenSettings extends net.minecraft.world.level.saveddata.SavedData {
    public static final com.mojang.serialization.Codec<net.minecraft.world.level.levelgen.WorldGenSettings> CODEC;
    public static final net.minecraft.world.level.saveddata.SavedDataType<net.minecraft.world.level.levelgen.WorldGenSettings> TYPE;
    private final net.minecraft.world.level.levelgen.WorldOptions options;
    private final net.minecraft.world.level.levelgen.WorldDimensions dimensions;
    public net.minecraft.world.level.levelgen.WorldGenSettings(net.minecraft.world.level.levelgen.WorldOptions, net.minecraft.world.level.levelgen.WorldDimensions);
    public static net.minecraft.world.level.levelgen.WorldGenSettings of(net.minecraft.world.level.levelgen.WorldOptions, net.minecraft.core.RegistryAccess);
    public net.minecraft.world.level.levelgen.WorldOptions options();
    public net.minecraft.world.level.levelgen.WorldDimensions dimensions();
    public int hashCode();
    public java.lang.String toString();
    private static net.minecraft.world.level.levelgen.WorldGenSettings lambda$static$1();
    private static com.mojang.datafixers.kinds.App lambda$static$0(com.mojang.serialization.codecs.RecordCodecBuilder$Instance);
    static {};
}
```
