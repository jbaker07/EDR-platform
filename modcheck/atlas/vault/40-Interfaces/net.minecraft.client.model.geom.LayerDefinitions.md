---
type: "interface"
fqcn: "net.minecraft.client.model.geom.LayerDefinitions"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.model.geom.LayerDefinitions

System: [[20-Systems/net.minecraft.client.model|net.minecraft.client.model]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `createRoots` | `()Ljava/util/Map;` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (8 fields, 14 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final FISH_PATTERN_DEFORMATION : Lnet/minecraft/client/model/geom/builders/CubeDeformation;
private static final OUTER_ARMOR_DEFORMATION : Lnet/minecraft/client/model/geom/builders/CubeDeformation;
private static final INNER_ARMOR_DEFORMATION : Lnet/minecraft/client/model/geom/builders/CubeDeformation;
private static final BABY_OUTER_ARMOR_DEFORMATION : Lnet/minecraft/client/model/geom/builders/CubeDeformation;
private static final BABY_INNER_ARMOR_DEFORMATION : Lnet/minecraft/client/model/geom/builders/CubeDeformation;
private static final BABY_PIGLIN_INNER_ARMOR_DEFORMATION : Lnet/minecraft/client/model/geom/builders/CubeDeformation;
private static final BABY_PIGLIN_OUTER_ARMOR_DEFORMATION : Lnet/minecraft/client/model/geom/builders/CubeDeformation;
private static final BABY_PIGLIN_ARMOR_ARM_OFFSET : Lnet/minecraft/client/model/geom/PartPose;
public <init>()V
public static createRoots()Ljava/util/Map;
private static synthetic lambda$createRoots$10(Lcom/google/common/collect/ImmutableMap;Lnet/minecraft/client/model/geom/ModelLayerLocation;)Z
private static synthetic lambda$createRoots$9(Lnet/minecraft/client/model/geom/builders/MeshDefinition;)Lnet/minecraft/client/model/geom/builders/LayerDefinition;
private static synthetic lambda$createRoots$8(Lnet/minecraft/client/model/geom/builders/MeshTransformer;Lnet/minecraft/client/model/geom/builders/LayerDefinition;)Lnet/minecraft/client/model/geom/builders/LayerDefinition;
private static synthetic lambda$createRoots$7(Lnet/minecraft/client/model/geom/builders/MeshTransformer;Lnet/minecraft/client/model/geom/builders/LayerDefinition;)Lnet/minecraft/client/model/geom/builders/LayerDefinition;
private static synthetic lambda$createRoots$6(Lnet/minecraft/client/model/geom/builders/MeshTransformer;Lnet/minecraft/client/model/geom/builders/LayerDefinition;)Lnet/minecraft/client/model/geom/builders/LayerDefinition;
private static synthetic lambda$createRoots$5(Lnet/minecraft/client/model/geom/builders/LayerDefinition;)Lnet/minecraft/client/model/geom/builders/LayerDefinition;
private static synthetic lambda$createRoots$4(Lnet/minecraft/client/model/geom/builders/MeshDefinition;)Lnet/minecraft/client/model/geom/builders/LayerDefinition;
private static synthetic lambda$createRoots$3(Lnet/minecraft/client/model/geom/builders/MeshDefinition;)Lnet/minecraft/client/model/geom/builders/LayerDefinition;
private static synthetic lambda$createRoots$2(Lnet/minecraft/client/model/geom/builders/MeshDefinition;)Lnet/minecraft/client/model/geom/builders/LayerDefinition;
private static synthetic lambda$createRoots$1(Lnet/minecraft/client/model/geom/builders/MeshDefinition;)Lnet/minecraft/client/model/geom/builders/LayerDefinition;
private static synthetic lambda$createRoots$0(Lnet/minecraft/client/model/geom/builders/MeshDefinition;)Lnet/minecraft/client/model/geom/builders/LayerDefinition;
static <clinit>()V
```
