---
type: "interface"
fqcn: "net.minecraft.client.resources.model.BlockStateModelLoader$LoadedModels"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.resources.model.BlockStateModelLoader$LoadedModels

System: [[20-Systems/net.minecraft.client.resources|net.minecraft.client.resources]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/util/Map;)V` | exact | invokespecial@26 in `ModelLoadingEventDispatcher.modifyBlockModelsOnLoad` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `models` | `()Ljava/util/Map;` | exact | invokevirtual@1 in `ModelLoadingEventDispatcher.modifyBlockModelsOnLoad` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |

## Declared members (1 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final models : Ljava/util/Map;
public <init>(Ljava/util/Map;)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public models()Ljava/util/Map;
```
