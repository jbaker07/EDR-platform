---
type: "interface"
fqcn: "net.minecraft.client.renderer.debug.DebugRenderer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.debug.DebugRenderer

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `refreshRendererList` | `()V` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-debug-api-v1|fabric-debug-api-v1]] | direct_reference |
| reads | `renderers` | `Ljava/util/List;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-debug-api-v1|fabric-debug-api-v1]] | declared |

## Declared members (2 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final renderers : Ljava/util/List;
private lastDebugEntriesVersion : J
public <init>()V
public refreshRendererList()V
public emitGizmos(Lnet/minecraft/client/renderer/culling/Frustum;DDDF)V
public static getTargetedEntity(Lnet/minecraft/world/entity/Entity;I)Ljava/util/Optional;
private static mixColor(F)Lnet/minecraft/world/phys/Vec3;
private static shiftHue(FFFF)Lnet/minecraft/world/phys/Vec3;
```
