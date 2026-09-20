---
type: "interface"
fqcn: "net.minecraft.client.resources.model.SimpleModelWrapper"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.resources.model.SimpleModelWrapper

System: [[20-Systems/net.minecraft.client.resources|net.minecraft.client.resources]]

`record` public final; extends `java/lang/Record`; implements `net/minecraft/client/renderer/block/dispatch/BlockStateModelPart`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/client/resources/model/geometry/QuadCollection;ZLnet/m` | exact | invokespecial@225 in `SimpleUnbakedExtraModel.bakeResolved` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| injects_into | `findNonBlockSprites` | `(Lnet/minecraft/client/resources/model/geometry/QuadCollection;)Lcom/g` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `quads` | `Lnet/minecraft/client/resources/model/geometry/QuadCollection;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | declared |
| reads | `useAmbientOcclusion` | `Z` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | declared |
| wraps | `bake` | `(Lnet/minecraft/client/resources/model/ModelBaker;Lnet/minecraft/resou` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (4 fields, 12 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final quads : Lnet/minecraft/client/resources/model/geometry/QuadCollection;
private final useAmbientOcclusion : Z
private final particleMaterial : Lnet/minecraft/client/resources/model/sprite/Material$Baked;
private static final LOGGER : Lorg/slf4j/Logger;
public <init>(Lnet/minecraft/client/resources/model/geometry/QuadCollection;ZLnet/minecraft/client/resources/model/sprite/Material$Baked;)V
public static bake(Lnet/minecraft/client/resources/model/ModelBaker;Lnet/minecraft/resources/Identifier;Lnet/minecraft/client/renderer/block/dispatch/ModelState;)Lnet/minecraft/client/renderer/block/dispatch/BlockStateModelPart;
public static findNonBlockSprites(Lnet/minecraft/client/resources/model/geometry/QuadCollection;)Lcom/google/common/collect/Multimap;
public getQuads(Lnet/minecraft/core/Direction;)Ljava/util/List;
public materialFlags()I
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public quads()Lnet/minecraft/client/resources/model/geometry/QuadCollection;
public useAmbientOcclusion()Z
public particleMaterial()Lnet/minecraft/client/resources/model/sprite/Material$Baked;
static <clinit>()V
```
