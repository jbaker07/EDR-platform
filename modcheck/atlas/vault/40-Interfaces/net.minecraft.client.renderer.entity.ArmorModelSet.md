---
type: "interface"
fqcn: "net.minecraft.client.renderer.entity.ArmorModelSet"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.entity.ArmorModelSet

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `map` | `(Ljava/util/function/Function;)Lnet/minecraft/client/renderer/entity/A` | exact | invokevirtual@62 in `ModelLayerRegistry.registerArmorModelLayers` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `putFrom` | `(Lnet/minecraft/client/renderer/entity/ArmorModelSet;Lcom/google/commo` | exact | invokevirtual@121 in `LayerDefinitionsMixin.registerExtraModelData` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (4 fields, 13 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final head : Ljava/lang/Object;
private final chest : Ljava/lang/Object;
private final legs : Ljava/lang/Object;
private final feet : Ljava/lang/Object;
public <init>(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V
public get(Lnet/minecraft/world/entity/EquipmentSlot;)Ljava/lang/Object;
public map(Ljava/util/function/Function;)Lnet/minecraft/client/renderer/entity/ArmorModelSet;
public putFrom(Lnet/minecraft/client/renderer/entity/ArmorModelSet;Lcom/google/common/collect/ImmutableMap$Builder;)V
public static bake(Lnet/minecraft/client/renderer/entity/ArmorModelSet;Lnet/minecraft/client/model/geom/EntityModelSet;Ljava/util/function/Function;)Lnet/minecraft/client/renderer/entity/ArmorModelSet;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public head()Ljava/lang/Object;
public chest()Ljava/lang/Object;
public legs()Ljava/lang/Object;
public feet()Ljava/lang/Object;
private static synthetic lambda$bake$0(Ljava/util/function/Function;Lnet/minecraft/client/model/geom/EntityModelSet;Lnet/minecraft/client/model/geom/ModelLayerLocation;)Lnet/minecraft/client/model/HumanoidModel;
```
