---
type: "interface"
fqcn: "net.minecraft.client.renderer.chunk.RenderRegionCache"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.chunk.RenderRegionCache

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `createRegion` | `(Lnet/minecraft/client/multiplayer/ClientLevel;J)Lnet/minecraft/client` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| injects_into | `createRegion` | `(Lnet/minecraft/client/multiplayer/ClientLevel;J)Lnet/minecraft/client` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |

## Declared members (1 fields, 4 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final sectionCopyCache : Lit/unimi/dsi/fastutil/longs/Long2ObjectMap;
public <init>()V
public createRegion(Lnet/minecraft/client/multiplayer/ClientLevel;J)Lnet/minecraft/client/renderer/chunk/RenderSectionRegion;
private getSectionDataCopy(Lnet/minecraft/world/level/Level;III)Lnet/minecraft/client/renderer/chunk/SectionCopy;
private static synthetic lambda$getSectionDataCopy$0(Lnet/minecraft/world/level/Level;IIIJ)Lnet/minecraft/client/renderer/chunk/SectionCopy;
```
