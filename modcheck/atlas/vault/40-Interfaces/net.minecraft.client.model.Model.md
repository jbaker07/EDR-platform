---
type: "interface"
fqcn: "net.minecraft.client.model.Model"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.model.Model

System: [[20-Systems/net.minecraft.client.model|net.minecraft.client.model]]

`abstract_class` public abstract; extends `java/lang/Object`; implements `net/fabricmc/fabric/api/client/rendering/v1/FabricModel`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/client/model/geom/ModelPart;Ljava/util/function/Functi` | exact | invokespecial@16 in `TransformCopyingModel.<init>` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `copyTransforms` | `(Lnet/minecraft/client/model/Model;)V` | inherited_exact | invokevirtual@23 in `TransformCopyingModel.setupAnim` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `copyTransforms` | `(Lnet/minecraft/client/model/Model;)V` | inherited_exact | invokevirtual@5 in `TransformCopyingModel.copyTransforms` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getChildPart` | `(Ljava/lang/String;)Lnet/minecraft/client/model/geom/ModelPart;` | inherited_exact | invokevirtual@5 in `TransformCopyingModel.getChildPart` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `root` | `()Lnet/minecraft/client/model/geom/ModelPart;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | declared |
| calls | `root` | `()Lnet/minecraft/client/model/geom/ModelPart;` | exact | invokevirtual@2 in `TransformCopyingModel.<init>` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `root` | `()Lnet/minecraft/client/model/geom/ModelPart;` | exact | invokevirtual@1 in `ModelMixin.copyTransforms` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `root` | `()Lnet/minecraft/client/model/geom/ModelPart;` | exact | invokevirtual@12 in `ModelMixin.copyTransforms` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `setupAnim` | `(Ljava/lang/Object;)V` | exact | invokevirtual@12 in `TransformCopyingModel.setupAnim` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `setupAnim` | `(Ljava/lang/Object;)V` | exact | invokevirtual@41 in `TransformCopyingModel.setupAnim` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `<init>` | `(Lnet/minecraft/client/model/geom/ModelPart;Ljava/util/function/Functi` | name_only | @Inject at ['TAIL'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (3 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
protected final root : Lnet/minecraft/client/model/geom/ModelPart;
protected final renderType : Ljava/util/function/Function;
private final allParts : Ljava/util/List;
public <init>(Lnet/minecraft/client/model/geom/ModelPart;Ljava/util/function/Function;)V
public final renderType()Ljava/util/function/Function;
public final renderType(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public final renderToBuffer(Lcom/mojang/blaze3d/vertex/PoseStack;Lcom/mojang/blaze3d/vertex/VertexConsumer;III)V
public final root()Lnet/minecraft/client/model/geom/ModelPart;
public final allParts()Ljava/util/List;
public setupAnim(Ljava/lang/Object;)V
public final resetPose()V
```
