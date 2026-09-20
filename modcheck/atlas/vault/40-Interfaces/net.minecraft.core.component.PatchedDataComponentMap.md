---
type: "interface"
fqcn: "net.minecraft.core.component.PatchedDataComponentMap"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.component.PatchedDataComponentMap

System: [[20-Systems/net.minecraft.core.component|net.minecraft.core.component]]

`class` public final; extends `java/lang/Object`; implements `net/minecraft/core/component/DataComponentMap`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `fromPatch` | `(Lnet/minecraft/core/component/DataComponentMap;Lnet/minecraft/core/co` | exact | invokestatic@15 in `VariantCodecs.validateComponents` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `fromPatch` | `(Lnet/minecraft/core/component/DataComponentMap;Lnet/minecraft/core/co` | exact | invokestatic@32 in `FluidVariantImpl.<init>` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (3 fields, 24 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final prototype : Lnet/minecraft/core/component/DataComponentMap;
private patch : Lit/unimi/dsi/fastutil/objects/Reference2ObjectMap;
private copyOnWrite : Z
public <init>(Lnet/minecraft/core/component/DataComponentMap;)V
private <init>(Lnet/minecraft/core/component/DataComponentMap;Lit/unimi/dsi/fastutil/objects/Reference2ObjectMap;Z)V
public static fromPatch(Lnet/minecraft/core/component/DataComponentMap;Lnet/minecraft/core/component/DataComponentPatch;)Lnet/minecraft/core/component/PatchedDataComponentMap;
private static isPatchSanitized(Lnet/minecraft/core/component/DataComponentMap;Lit/unimi/dsi/fastutil/objects/Reference2ObjectMap;)Z
public get(Lnet/minecraft/core/component/DataComponentType;)Ljava/lang/Object;
public hasNonDefault(Lnet/minecraft/core/component/DataComponentType;)Z
public set(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)Ljava/lang/Object;
public set(Lnet/minecraft/core/component/TypedDataComponent;)Ljava/lang/Object;
public remove(Lnet/minecraft/core/component/DataComponentType;)Ljava/lang/Object;
public applyPatch(Lnet/minecraft/core/component/DataComponentPatch;)V
private applyPatch(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)V
public restorePatch(Lnet/minecraft/core/component/DataComponentPatch;)V
public clearPatch()V
public setAll(Lnet/minecraft/core/component/DataComponentMap;)V
private ensureMapOwnership()V
public keySet()Ljava/util/Set;
public iterator()Ljava/util/Iterator;
public size()I
public asPatch()Lnet/minecraft/core/component/DataComponentPatch;
public copy()Lnet/minecraft/core/component/PatchedDataComponentMap;
public toImmutableMap()Lnet/minecraft/core/component/DataComponentMap;
public equals(Ljava/lang/Object;)Z
public hashCode()I
public toString()Ljava/lang/String;
```
