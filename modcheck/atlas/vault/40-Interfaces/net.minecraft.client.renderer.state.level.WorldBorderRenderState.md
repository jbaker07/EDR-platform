---
type: "interface"
fqcn: "net.minecraft.client.renderer.state.level.WorldBorderRenderState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.state.level.WorldBorderRenderState

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `reset` | `()V` | name_only | @Inject at ['TAIL'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (6 fields, 4 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public minX : D
public maxX : D
public minZ : D
public maxZ : D
public tint : I
public alpha : D
public <init>()V
public closestBorder(DD)Ljava/util/List;
public reset()V
private static synthetic lambda$closestBorder$0(Lnet/minecraft/client/renderer/state/level/WorldBorderRenderState$DistancePerDirection;)D
```
