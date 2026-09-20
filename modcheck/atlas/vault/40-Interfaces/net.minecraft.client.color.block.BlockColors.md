---
type: "interface"
fqcn: "net.minecraft.client.color.block.BlockColors"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.color.block.BlockColors

System: [[20-Systems/net.minecraft.client.color|net.minecraft.client.color]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getTintSources` | `(Lnet/minecraft/world/level/block/state/BlockState;)Ljava/util/List;` | exact | invokevirtual@5 in `AltModelBlockRendererImpl.configureTintCache` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getTintSources` | `(Lnet/minecraft/world/level/block/state/BlockState;)Ljava/util/List;` | exact | invokevirtual@75 in `BlockColorRegistryImpl.register` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `register` | `(Ljava/util/List;[Lnet/minecraft/world/level/block/Block;)V` | exact | invokevirtual@72 in `BlockColorRegistryImpl.register` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `register` | `(Ljava/util/List;[Lnet/minecraft/world/level/block/Block;)V` | exact | invokevirtual@10 in `BlockColorRegistryImpl.lambda$initialize$0` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `createDefault` | `()Lnet/minecraft/client/color/block/BlockColors;` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (4 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final LILY_PAD_IN_WORLD : I
public static final LILY_PAD_DEFAULT : I
private static final BLANK_LAYER : Lnet/minecraft/client/color/block/BlockTintSource;
private final sources : Ljava/util/Map;
public <init>()V
public static createDefault()Lnet/minecraft/client/color/block/BlockColors;
public getTintSources(Lnet/minecraft/world/level/block/state/BlockState;)Ljava/util/List;
public getTintSource(Lnet/minecraft/world/level/block/state/BlockState;I)Lnet/minecraft/client/color/block/BlockTintSource;
public register(Ljava/util/List;[Lnet/minecraft/world/level/block/Block;)V
public getColoringProperties(Lnet/minecraft/world/level/block/Block;)Ljava/util/Set;
static <clinit>()V
```
