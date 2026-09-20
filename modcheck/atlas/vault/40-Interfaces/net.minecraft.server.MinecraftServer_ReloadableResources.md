---
type: "interface"
fqcn: "net.minecraft.server.MinecraftServer$ReloadableResources"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.MinecraftServer$ReloadableResources

System: [[20-Systems/net.minecraft.server|net.minecraft.server]]

`record` final; extends `java/lang/Record`; implements `java/lang/AutoCloseable`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `managers` | `()Lnet/minecraft/server/ReloadableServerResources;` | exact | invokevirtual@4 in `MinecraftServerMixin.getOrThrow` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `resourceManager` | `()Lnet/minecraft/server/packs/resources/CloseableResourceManager;` | exact | invokevirtual@17 in `MinecraftServerMixin.startResourceReload` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `resourceManager` | `()Lnet/minecraft/server/packs/resources/CloseableResourceManager;` | exact | invokevirtual@17 in `MinecraftServerMixin.lambda$endResourceReload$0` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |

## Declared members (2 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final resourceManager : Lnet/minecraft/server/packs/resources/CloseableResourceManager;
private final managers : Lnet/minecraft/server/ReloadableServerResources;
private <init>(Lnet/minecraft/server/packs/resources/CloseableResourceManager;Lnet/minecraft/server/ReloadableServerResources;)V
public close()V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public resourceManager()Lnet/minecraft/server/packs/resources/CloseableResourceManager;
public managers()Lnet/minecraft/server/ReloadableServerResources;
```
