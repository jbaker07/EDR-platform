---
type: "interface"
fqcn: "net.minecraft.util.CrudeIncrementalIntIdentityHashBiMap"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.CrudeIncrementalIntIdentityHashBiMap

System: [[20-Systems/net.minecraft.util|net.minecraft.util]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/core/IdMap`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `add` | `(Ljava/lang/Object;)I` | exact | invokevirtual@55 in `FabricEntityDataRegistryImpl.reorderHandlers` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `add` | `(Ljava/lang/Object;)I` | exact | invokevirtual@98 in `FabricEntityDataRegistryImpl.reorderHandlers` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `add` | `(Ljava/lang/Object;)I` | exact | invokevirtual@135 in `FabricEntityDataRegistryImpl.reorderHandlers` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `clear` | `()V` | exact | invokevirtual@22 in `FabricEntityDataRegistryImpl.reorderHandlers` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `iterator` | `()Ljava/util/Iterator;` | exact | invokevirtual@21 in `FabricEntityDataRegistryImpl.storeVanillaHandlers` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `iterator` | `()Ljava/util/Iterator;` | exact | invokevirtual@5 in `FabricEntityDataRegistryImpl.storeExternalHandlers` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `size` | `()I` | exact | invokevirtual@10 in `FabricEntityDataRegistryImpl.reorderHandlers` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `size` | `()I` | exact | invokevirtual@148 in `FabricEntityDataRegistryImpl.reorderHandlers` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |

## Declared members (8 fields, 20 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final NOT_FOUND : I
private static final EMPTY_SLOT : Ljava/lang/Object;
private static final LOADFACTOR : F
private keys : [Ljava/lang/Object;
private values : [I
private byId : [Ljava/lang/Object;
private nextId : I
private size : I
private <init>(I)V
private <init>([Ljava/lang/Object;[I[Ljava/lang/Object;II)V
public static create(I)Lnet/minecraft/util/CrudeIncrementalIntIdentityHashBiMap;
public getId(Ljava/lang/Object;)I
public byId(I)Ljava/lang/Object;
private getValue(I)I
public contains(Ljava/lang/Object;)Z
public contains(I)Z
public add(Ljava/lang/Object;)I
private nextId()I
private grow(I)V
public addMapping(Ljava/lang/Object;I)V
private hash(Ljava/lang/Object;)I
private indexOf(Ljava/lang/Object;I)I
private findEmpty(I)I
public iterator()Ljava/util/Iterator;
public clear()V
public size()I
public copy()Lnet/minecraft/util/CrudeIncrementalIntIdentityHashBiMap;
static <clinit>()V
```
