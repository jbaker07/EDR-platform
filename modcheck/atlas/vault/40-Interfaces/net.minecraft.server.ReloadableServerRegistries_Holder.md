---
type: "interface"
fqcn: "net.minecraft.server.ReloadableServerRegistries$Holder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.ReloadableServerRegistries$Holder

System: [[20-Systems/net.minecraft.server|net.minecraft.server]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `lookup` | `()Lnet/minecraft/core/HolderLookup$Provider;` | exact | invokevirtual@7 in `LootUtil.getEntryOrDirect` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |

## Declared members (1 fields, 4 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final registries : Lnet/minecraft/core/HolderLookup$Provider;
public <init>(Lnet/minecraft/core/HolderLookup$Provider;)V
public lookup()Lnet/minecraft/core/HolderLookup$Provider;
public getLootTable(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/world/level/storage/loot/LootTable;
private static synthetic lambda$getLootTable$0(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/core/HolderLookup$RegistryLookup;)Ljava/util/Optional;
```
