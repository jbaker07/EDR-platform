---
type: "interface"
fqcn: "net.minecraft.core.component.DataComponentMap$Builder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.component.DataComponentMap$Builder

System: [[20-Systems/net.minecraft.core.component|net.minecraft.core.component]]

`class` public; extends `java/lang/Object`; implements `net/fabricmc/fabric/api/item/v1/FabricComponentMapBuilder`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `addAll` | `(Lnet/minecraft/core/component/DataComponentMap;)Lnet/minecraft/core/c` | exact | invokevirtual@48 in `DefaultItemComponentImpl$ModifyContextImpl.modify` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `addAll` | `(Lnet/minecraft/core/component/DataComponentMap;)Lnet/minecraft/core/c` | exact | invokevirtual@44 in `EnchantmentUtil.modify` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `build` | `()Lnet/minecraft/core/component/DataComponentMap;` | exact | invokevirtual@74 in `DefaultItemComponentImpl$ModifyContextImpl.modify` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `build` | `()Lnet/minecraft/core/component/DataComponentMap;` | exact | invokevirtual@166 in `EnchantmentUtil.modify` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `get` | `(Lnet/minecraft/core/component/DataComponentType;)Ljava/lang/Object;` | inherited_exact | invokevirtual@5 in `FabricItem$Properties.lambda$modifyComponent$0` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `set` | `(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)Ln` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | declared |
| calls | `set` | `(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)Ln` | exact | invokevirtual@16 in `FabricItem$Properties.lambda$modifyComponent$0` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| reads | `map` | `Lit/unimi/dsi/fastutil/objects/Reference2ObjectMap;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | declared |

## Declared members (2 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final map : Lit/unimi/dsi/fastutil/objects/Reference2ObjectMap;
private validator : Ljava/util/function/Consumer;
private <init>()V
public set(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)Lnet/minecraft/core/component/DataComponentMap$Builder;
 setUnchecked(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)V
public addAll(Lnet/minecraft/core/component/DataComponentMap;)Lnet/minecraft/core/component/DataComponentMap$Builder;
public addValidator(Ljava/util/function/Consumer;)Lnet/minecraft/core/component/DataComponentMap$Builder;
public build()Lnet/minecraft/core/component/DataComponentMap;
private static buildFromMapTrusted(Ljava/util/Map;)Lnet/minecraft/core/component/DataComponentMap;
private static synthetic lambda$new$0(Lnet/minecraft/core/component/DataComponentMap;)V
```
