---
type: "interface"
fqcn: "net.minecraft.client.renderer.chunk.RenderRegionCache"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.chunk.RenderRegionCache

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `createRegion` | `@Inject at INVOKE Lnet/minecraft/client/renderer/chunk/RenderRegionCache;getSect` | client | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| injects_into | `createRegion` | `@Inject at RETURN` | client | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |

## Declared members (5, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.chunk.RenderRegionCache {
    private final it.unimi.dsi.fastutil.longs.Long2ObjectMap<net.minecraft.client.renderer.chunk.SectionCopy> sectionCopyCache;
    public net.minecraft.client.renderer.chunk.RenderRegionCache();
    public net.minecraft.client.renderer.chunk.RenderSectionRegion createRegion(net.minecraft.client.multiplayer.ClientLevel, long);
    private net.minecraft.client.renderer.chunk.SectionCopy getSectionDataCopy(net.minecraft.world.level.Level, int, int, int);
    private static net.minecraft.client.renderer.chunk.SectionCopy lambda$getSectionDataCopy$0(net.minecraft.world.level.Level, int, int, int, long);
}
```
