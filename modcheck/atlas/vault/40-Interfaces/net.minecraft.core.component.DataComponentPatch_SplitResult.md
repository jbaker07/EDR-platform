---
type: "interface"
fqcn: "net.minecraft.core.component.DataComponentPatch$SplitResult"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.component.DataComponentPatch$SplitResult

System: [[20-Systems/net.minecraft.core.component|net.minecraft.core.component]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `added` | `()Lnet/minecraft/core/component/DataComponentMap;` | exact | invokevirtual@22 in `ComponentsIngredient.test` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `added` | `()Lnet/minecraft/core/component/DataComponentMap;` | exact | invokevirtual@7 in `TransferApiImpl.writeChangesTo` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `removed` | `()Ljava/util/Set;` | exact | invokevirtual@77 in `ComponentsIngredient.test` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `removed` | `()Ljava/util/Set;` | exact | invokevirtual@15 in `TransferApiImpl.writeChangesTo` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (3 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final added : Lnet/minecraft/core/component/DataComponentMap;
private final removed : Ljava/util/Set;
public static final EMPTY : Lnet/minecraft/core/component/DataComponentPatch$SplitResult;
public <init>(Lnet/minecraft/core/component/DataComponentMap;Ljava/util/Set;)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public added()Lnet/minecraft/core/component/DataComponentMap;
public removed()Ljava/util/Set;
static <clinit>()V
```
