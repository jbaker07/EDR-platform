---
type: "interface"
fqcn: "net.minecraft.client.model.geom.ModelPart"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.model.geom.ModelPart

System: [[20-Systems/net.minecraft.client.model|net.minecraft.client.model]]

`class` public final; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `x` | `F` | exact | getfield@2 in `ModelMixin.copyTransforms` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `xRot` | `F` | exact | getfield@26 in `ModelMixin.copyTransforms` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `xScale` | `F` | exact | getfield@50 in `ModelMixin.copyTransforms` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `y` | `F` | exact | getfield@10 in `ModelMixin.copyTransforms` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `yRot` | `F` | exact | getfield@34 in `ModelMixin.copyTransforms` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `yScale` | `F` | exact | getfield@58 in `ModelMixin.copyTransforms` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `z` | `F` | exact | getfield@18 in `ModelMixin.copyTransforms` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `zRot` | `F` | exact | getfield@42 in `ModelMixin.copyTransforms` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `zScale` | `F` | exact | getfield@66 in `ModelMixin.copyTransforms` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| writes | `x` | `F` | exact | putfield@5 in `ModelMixin.copyTransforms` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| writes | `xRot` | `F` | exact | putfield@29 in `ModelMixin.copyTransforms` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| writes | `xScale` | `F` | exact | putfield@53 in `ModelMixin.copyTransforms` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| writes | `y` | `F` | exact | putfield@13 in `ModelMixin.copyTransforms` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| writes | `yRot` | `F` | exact | putfield@37 in `ModelMixin.copyTransforms` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| writes | `yScale` | `F` | exact | putfield@61 in `ModelMixin.copyTransforms` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| writes | `z` | `F` | exact | putfield@21 in `ModelMixin.copyTransforms` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| writes | `zRot` | `F` | exact | putfield@45 in `ModelMixin.copyTransforms` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| writes | `zScale` | `F` | exact | putfield@69 in `ModelMixin.copyTransforms` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (15 fields, 29 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final DEFAULT_SCALE : F
public x : F
public y : F
public z : F
public xRot : F
public yRot : F
public zRot : F
public xScale : F
public yScale : F
public zScale : F
public visible : Z
public skipDraw : Z
private final cubes : Ljava/util/List;
private final children : Ljava/util/Map;
private initialPose : Lnet/minecraft/client/model/geom/PartPose;
public <init>(Ljava/util/List;Ljava/util/Map;)V
public storePose()Lnet/minecraft/client/model/geom/PartPose;
public getInitialPose()Lnet/minecraft/client/model/geom/PartPose;
public setInitialPose(Lnet/minecraft/client/model/geom/PartPose;)V
public resetPose()V
public loadPose(Lnet/minecraft/client/model/geom/PartPose;)V
public hasChild(Ljava/lang/String;)Z
public getChild(Ljava/lang/String;)Lnet/minecraft/client/model/geom/ModelPart;
public setPos(FFF)V
public setRotation(FFF)V
public render(Lcom/mojang/blaze3d/vertex/PoseStack;Lcom/mojang/blaze3d/vertex/VertexConsumer;II)V
public render(Lcom/mojang/blaze3d/vertex/PoseStack;Lcom/mojang/blaze3d/vertex/VertexConsumer;III)V
public rotateBy(Lorg/joml/Quaternionf;)V
public getExtentsForGui(Lcom/mojang/blaze3d/vertex/PoseStack;Ljava/util/function/Consumer;)V
public visit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/model/geom/ModelPart$Visitor;)V
private visit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/model/geom/ModelPart$Visitor;Ljava/lang/String;)V
public translateAndRotate(Lcom/mojang/blaze3d/vertex/PoseStack;)V
private compile(Lcom/mojang/blaze3d/vertex/PoseStack$Pose;Lcom/mojang/blaze3d/vertex/VertexConsumer;III)V
public getRandomCube(Lnet/minecraft/util/RandomSource;)Lnet/minecraft/client/model/geom/ModelPart$Cube;
public isEmpty()Z
public offsetPos(Lorg/joml/Vector3f;)V
public offsetRotation(Lorg/joml/Vector3f;)V
public offsetScale(Lorg/joml/Vector3f;)V
public getAllParts()Ljava/util/List;
public createPartLookup()Ljava/util/function/Function;
private addAllChildren(Ljava/util/function/BiConsumer;)V
private static synthetic lambda$getAllParts$0(Ljava/util/List;Ljava/lang/String;Lnet/minecraft/client/model/geom/ModelPart;)V
private static synthetic lambda$visit$0(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/model/geom/ModelPart$Visitor;Ljava/lang/String;Ljava/lang/String;Lnet/minecraft/client/model/geom/ModelPart;)V
private static synthetic lambda$getExtentsForGui$0(Ljava/util/function/Consumer;Lcom/mojang/blaze3d/vertex/PoseStack$Pose;Ljava/lang/String;ILnet/minecraft/client/model/geom/ModelPart$Cube;)V
```
