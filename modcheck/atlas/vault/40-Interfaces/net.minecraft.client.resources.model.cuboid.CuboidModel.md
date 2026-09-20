---
type: "interface"
fqcn: "net.minecraft.client.resources.model.cuboid.CuboidModel"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.resources.model.cuboid.CuboidModel

System: [[20-Systems/net.minecraft.client.resources|net.minecraft.client.resources]]

`record` public final; extends `java/lang/Record`; implements `net/minecraft/client/resources/model/UnbakedModel`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `<clinit>` | `()V` | exact | @ModifyExpressionValue at ['NEW'] | client | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |

## Declared members (7 fields, 12 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final geometry : Lnet/minecraft/client/resources/model/geometry/UnbakedGeometry;
private final guiLight : Lnet/minecraft/client/resources/model/UnbakedModel$GuiLight;
private final ambientOcclusion : Ljava/lang/Boolean;
private final transforms : Lnet/minecraft/client/resources/model/cuboid/ItemTransforms;
private final textureSlots : Lnet/minecraft/client/resources/model/sprite/TextureSlots$Data;
private final parent : Lnet/minecraft/resources/Identifier;
static final GSON : Lcom/google/gson/Gson;
public <init>(Lnet/minecraft/client/resources/model/geometry/UnbakedGeometry;Lnet/minecraft/client/resources/model/UnbakedModel$GuiLight;Ljava/lang/Boolean;Lnet/minecraft/client/resources/model/cuboid/ItemTransforms;Lnet/minecraft/client/resources/model/sprite/TextureSlots$Data;Lnet/minecraft/resources/Identifier;)V
public static fromStream(Ljava/io/Reader;)Lnet/minecraft/client/resources/model/cuboid/CuboidModel;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public geometry()Lnet/minecraft/client/resources/model/geometry/UnbakedGeometry;
public guiLight()Lnet/minecraft/client/resources/model/UnbakedModel$GuiLight;
public ambientOcclusion()Ljava/lang/Boolean;
public transforms()Lnet/minecraft/client/resources/model/cuboid/ItemTransforms;
public textureSlots()Lnet/minecraft/client/resources/model/sprite/TextureSlots$Data;
public parent()Lnet/minecraft/resources/Identifier;
static <clinit>()V
```
