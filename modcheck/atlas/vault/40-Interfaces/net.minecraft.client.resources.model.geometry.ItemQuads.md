---
type: "interface"
fqcn: "net.minecraft.client.resources.model.geometry.ItemQuads"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.resources.model.geometry.ItemQuads

System: [[20-Systems/net.minecraft.client.resources|net.minecraft.client.resources]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `all` | `()Ljava/util/List;` | exact | invokevirtual@168 in `SubmitNodeCollectionMixin.submitItem` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `solid` | `()Ljava/util/List;` | exact | invokevirtual@81 in `SubmitNodeCollectionMixin.submitItem` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `solid` | `()Ljava/util/List;` | exact | invokevirtual@124 in `SubmitNodeCollectionMixin.submitItem` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `translucent` | `()Ljava/util/List;` | exact | invokevirtual@18 in `SubmitNodeCollectionMixin.submitItem` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `translucent` | `()Ljava/util/List;` | exact | invokevirtual@61 in `SubmitNodeCollectionMixin.submitItem` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (4 fields, 10 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final all : Ljava/util/List;
private final solid : Ljava/util/List;
private final translucent : Ljava/util/List;
public static final EMPTY : Lnet/minecraft/client/resources/model/geometry/ItemQuads;
public <init>(Ljava/util/List;Ljava/util/List;Ljava/util/List;)V
public static split(Ljava/util/List;)Lnet/minecraft/client/resources/model/geometry/ItemQuads;
public isEmpty()Z
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public all()Ljava/util/List;
public solid()Ljava/util/List;
public translucent()Ljava/util/List;
static <clinit>()V
```
