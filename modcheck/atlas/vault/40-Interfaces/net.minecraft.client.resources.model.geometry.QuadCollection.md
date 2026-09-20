---
type: "interface"
fqcn: "net.minecraft.client.resources.model.geometry.QuadCollection"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.resources.model.geometry.QuadCollection

System: [[20-Systems/net.minecraft.client.resources|net.minecraft.client.resources]]

`class` public; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/util/List;Ljava/util/List;Ljava/util/List;Ljava/util/List;Ljava` | exact | invokespecial@25 in `MeshQuadCollection.<init>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getAll` | `()Ljava/util/List;` | exact | invokevirtual@95 in `SimpleUnbakedExtraModel.bakeResolved` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |

## Declared members (11 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final EMPTY : Lnet/minecraft/client/resources/model/geometry/QuadCollection;
private static final FLAGS_NOT_COMPUTED : I
private final all : Ljava/util/List;
private final unculled : Ljava/util/List;
private final north : Ljava/util/List;
private final south : Ljava/util/List;
private final east : Ljava/util/List;
private final west : Ljava/util/List;
private final up : Ljava/util/List;
private final down : Ljava/util/List;
private materialFlags : I
private <init>(Ljava/util/List;Ljava/util/List;Ljava/util/List;Ljava/util/List;Ljava/util/List;Ljava/util/List;Ljava/util/List;Ljava/util/List;)V
private static computeMaterialFlags(Ljava/util/List;)I
public getQuads(Lnet/minecraft/core/Direction;)Ljava/util/List;
public getAll()Ljava/util/List;
public materialFlags()I
public hasMaterialFlag(I)Z
static <clinit>()V
```
