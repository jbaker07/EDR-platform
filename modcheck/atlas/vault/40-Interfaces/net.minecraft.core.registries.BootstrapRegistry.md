---
type: "interface"
fqcn: "net.minecraft.core.registries.BootstrapRegistry"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.registries.BootstrapRegistry

System: [[20-Systems/net.minecraft.core.registries|net.minecraft.core.registries]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/core/HolderLookup$RegistryLookup`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/resources/ResourceKey;Lcom/mojang/serialization/Lifecy` | exact | invokespecial@34 in `FabricRecipeProvider$FabricBootstrapContext.<init>` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Holder$Refe` | inherited_exact | invokevirtual@36 in `FabricRecipeProvider$FabricBootstrapContext.register` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (3 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final key : Lnet/minecraft/resources/ResourceKey;
private final lifecycle : Lcom/mojang/serialization/Lifecycle;
private storage : Lnet/minecraft/core/registries/BootstrapRegistry$Storage;
public <init>(Lnet/minecraft/resources/ResourceKey;Lcom/mojang/serialization/Lifecycle;)V
public key()Lnet/minecraft/resources/ResourceKey;
public registryLifecycle()Lcom/mojang/serialization/Lifecycle;
public freeze()V
public removeIf(Ljava/util/function/Predicate;)V
public get(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;
public listElements()Ljava/util/stream/Stream;
public get(Lnet/minecraft/tags/TagKey;)Ljava/util/Optional;
public listTags()Ljava/util/stream/Stream;
```
