---
type: "interface"
fqcn: "net.minecraft.client.renderer.item.CuboidItemModelWrapper"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.item.CuboidItemModelWrapper

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/client/renderer/item/ItemModel`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `<init>` | `(Ljava/util/List;Lnet/minecraft/client/resources/model/geometry/QuadCo` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| injects_into | `update` | `(Lnet/minecraft/client/renderer/item/ItemStackRenderState;Lnet/minecra` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (6 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final tints : Ljava/util/List;
private final animated : Z
private final itemQuads : Lnet/minecraft/client/resources/model/geometry/ItemQuads;
private final extents : Ljava/util/function/Supplier;
private final properties : Lnet/minecraft/client/renderer/item/ModelRenderProperties;
private final transformation : Lorg/joml/Matrix4fc;
private <init>(Ljava/util/List;Lnet/minecraft/client/resources/model/geometry/QuadCollection;Lnet/minecraft/client/renderer/item/ModelRenderProperties;Lorg/joml/Matrix4fc;)V
public static computeExtents(Ljava/util/List;)[Lorg/joml/Vector3fc;
public update(Lnet/minecraft/client/renderer/item/ItemStackRenderState;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/client/renderer/item/ItemModelResolver;Lnet/minecraft/world/item/ItemDisplayContext;Lnet/minecraft/client/multiplayer/ClientLevel;Lnet/minecraft/world/entity/ItemOwner;I)V
private static validateAtlasUsage(Ljava/util/List;)V
private static hasSpecialAnimatedTexture(Lnet/minecraft/world/item/ItemStack;)Z
private static synthetic lambda$computeExtents$0(I)[Lorg/joml/Vector3fc;
private static synthetic lambda$new$0(Lnet/minecraft/client/resources/model/geometry/QuadCollection;)[Lorg/joml/Vector3fc;
```
