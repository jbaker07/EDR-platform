---
type: "interface"
fqcn: "net.minecraft.core.component.DataComponentPatch$Builder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.component.DataComponentPatch$Builder

System: [[20-Systems/net.minecraft.core.component|net.minecraft.core.component]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `build` | `()Lnet/minecraft/core/component/DataComponentPatch;` | exact | invokevirtual@13 in `DefaultCustomIngredients.components` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `build` | `()Lnet/minecraft/core/component/DataComponentPatch;` | exact | invokevirtual@16 in `CustomDataIngredient.createEntryDisplay` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `build` | `()Lnet/minecraft/core/component/DataComponentPatch;` | exact | invokevirtual@15 in `TransferApiImpl.mergePatches` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `build` | `()Lnet/minecraft/core/component/DataComponentPatch;` | exact | invokevirtual@174 in `BundleContentsStorage$BundleSlotWrapper.extract` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `build` | `()Lnet/minecraft/core/component/DataComponentPatch;` | exact | invokevirtual@85 in `BundleContentsStorage.insert` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `build` | `()Lnet/minecraft/core/component/DataComponentPatch;` | exact | invokevirtual@90 in `ItemContainerContentsStorage$ContainerSlotWrapper.setStack` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `set` | `(Ljava/lang/Iterable;)Lnet/minecraft/core/component/DataComponentPatch` | exact | invokevirtual@10 in `TransferApiImpl.writeChangesTo` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `set` | `(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)Ln` | exact | invokevirtual@13 in `CustomDataIngredient.createEntryDisplay` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `set` | `(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)Ln` | exact | invokevirtual@171 in `BundleContentsStorage$BundleSlotWrapper.extract` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `set` | `(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)Ln` | exact | invokevirtual@82 in `BundleContentsStorage.insert` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `set` | `(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)Ln` | exact | invokevirtual@87 in `ItemContainerContentsStorage$ContainerSlotWrapper.setStack` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (1 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final map : Lit/unimi/dsi/fastutil/objects/Reference2ObjectMap;
private <init>()V
public set(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)Lnet/minecraft/core/component/DataComponentPatch$Builder;
public remove(Lnet/minecraft/core/component/DataComponentType;)Lnet/minecraft/core/component/DataComponentPatch$Builder;
public set(Lnet/minecraft/core/component/TypedDataComponent;)Lnet/minecraft/core/component/DataComponentPatch$Builder;
public set(Ljava/lang/Iterable;)Lnet/minecraft/core/component/DataComponentPatch$Builder;
public build()Lnet/minecraft/core/component/DataComponentPatch;
```
