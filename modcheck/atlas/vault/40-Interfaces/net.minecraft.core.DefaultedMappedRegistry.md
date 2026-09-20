---
type: "interface"
fqcn: "net.minecraft.core.DefaultedMappedRegistry"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.DefaultedMappedRegistry

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

`class` public; extends `net/minecraft/core/MappedRegistry`; implements `net/minecraft/core/DefaultedRegistry`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/lang/String;Lnet/minecraft/resources/ResourceKey;Lcom/mojang/se` | exact | invokespecial@13 in `FabricRegistryBuilder.createDefaulted` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (2 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final defaultKey : Lnet/minecraft/resources/Identifier;
private defaultValue : Lnet/minecraft/core/Holder$Reference;
public <init>(Ljava/lang/String;Lnet/minecraft/resources/ResourceKey;Lcom/mojang/serialization/Lifecycle;Z)V
public register(Lnet/minecraft/resources/ResourceKey;Ljava/lang/Object;Lnet/minecraft/core/RegistrationInfo;)Lnet/minecraft/core/Holder$Reference;
public getId(Ljava/lang/Object;)I
public getKey(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;
public getValue(Lnet/minecraft/resources/Identifier;)Ljava/lang/Object;
public getOptional(Lnet/minecraft/resources/Identifier;)Ljava/util/Optional;
public getAny()Ljava/util/Optional;
public byId(I)Ljava/lang/Object;
public getRandom(Lnet/minecraft/util/RandomSource;)Ljava/util/Optional;
public getDefaultKey()Lnet/minecraft/resources/Identifier;
private synthetic lambda$getRandom$0()Ljava/util/Optional;
```
