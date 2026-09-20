---
type: "interface"
fqcn: "net.minecraft.core.NonNullList"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.NonNullList

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

`class` public; extends `java/util/AbstractList`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `set` | `(ILjava/lang/Object;)Ljava/lang/Object;` | exact | invokevirtual@13 in `AbstractFurnaceBlockEntityMixin.setStackSuppressUpdate` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `set` | `(ILjava/lang/Object;)Ljava/lang/Object;` | exact | invokevirtual@13 in `ChiseledBookShelfBlockEntityMixin.setStackBypass` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (2 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final list : Ljava/util/List;
private final defaultValue : Ljava/lang/Object;
public static create()Lnet/minecraft/core/NonNullList;
public static createWithCapacity(I)Lnet/minecraft/core/NonNullList;
public static withSize(ILjava/lang/Object;)Lnet/minecraft/core/NonNullList;
public static of(Ljava/lang/Object;[Ljava/lang/Object;)Lnet/minecraft/core/NonNullList;
protected <init>(Ljava/util/List;Ljava/lang/Object;)V
public get(I)Ljava/lang/Object;
public set(ILjava/lang/Object;)Ljava/lang/Object;
public add(ILjava/lang/Object;)V
public remove(I)Ljava/lang/Object;
public size()I
public clear()V
```
