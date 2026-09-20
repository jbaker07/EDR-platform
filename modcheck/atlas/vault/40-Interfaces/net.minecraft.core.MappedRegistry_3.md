---
type: "interface"
fqcn: "net.minecraft.core.MappedRegistry$3"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.MappedRegistry$3

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

`class` ; extends `java/lang/Object`; implements `net/minecraft/core/Registry$PendingTags`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `apply` | `()V` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| reads | `this$0` | `Lnet/minecraft/core/MappedRegistry;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | declared |

## Declared members (4 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
final synthetic val$pendingContents : Ljava/util/Map;
final synthetic val$patchedHolder : Lnet/minecraft/core/HolderLookup$RegistryLookup;
final synthetic val$pendingTags : Lcom/google/common/collect/ImmutableMap;
final synthetic this$0 : Lnet/minecraft/core/MappedRegistry;
 <init>(Lnet/minecraft/core/MappedRegistry;Ljava/util/Map;Lnet/minecraft/core/HolderLookup$RegistryLookup;Lcom/google/common/collect/ImmutableMap;)V
public key()Lnet/minecraft/resources/ResourceKey;
public size()I
public lookup()Lnet/minecraft/core/HolderLookup$RegistryLookup;
public apply()V
private static synthetic lambda$apply$0(Ljava/util/Map;Lnet/minecraft/tags/TagKey;Lnet/minecraft/core/HolderSet$Named;)V
```
