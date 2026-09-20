---
type: "interface"
fqcn: "net.minecraft.util.LightCoordsUtil"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.LightCoordsUtil

System: [[20-Systems/net.minecraft.util|net.minecraft.util]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `max(II)I` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `pack(II)I` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (18, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.util.LightCoordsUtil {
    public static final int FULL_BRIGHT;
    public static final int FULL_SKY;
    private static final int MAX_SMOOTH_LIGHT_LEVEL;
    public net.minecraft.util.LightCoordsUtil();
    public static int pack(int, int);
    public static int block(int);
    public static int sky(int);
    public static int withBlock(int, int);
    public static int smoothPack(int, int);
    public static int smoothBlock(int);
    public static int smoothSky(int);
    public static int addSmoothBlockEmission(int, float);
    public static int max(int, int);
    public static int lightCoordsWithEmission(int, int);
    public static int smoothBlend(int, int, int, int);
    public static int smoothWeightedBlend(int, int, int, int, float, float, float, float);
    public static int getLightCoords(net.minecraft.world.level.BlockAndLightGetter, net.minecraft.core.BlockPos);
    public static int getLightCoords(net.minecraft.util.LightCoordsUtil$BrightnessGetter, net.minecraft.world.level.BlockAndLightGetter, net.minecraft.world.level.block.state.BlockState, net.minecraft.core.BlockPos);
}
```
